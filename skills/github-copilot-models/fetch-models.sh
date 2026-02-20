#!/usr/bin/env bash
set -euo pipefail

# GitHub Copilot Models Query Script
# Fetches available models from GitHub Copilot API with filtering and formatting

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

# Default options
OUTPUT_JSON=false
PICKER_ONLY=false
CATEGORY=""
FAMILY=""
VENDOR=""
VISION_ONLY=false

# Help text
show_help() {
  cat << EOF
${BOLD}GitHub Copilot Models Query${RESET}

Fetch available AI models from GitHub Copilot API with filtering options.

${BOLD}USAGE:${RESET}
  $0 [OPTIONS]

${BOLD}OPTIONS:${RESET}
  --json              Output raw JSON response
  --picker-only       Show only featured models (model_picker_enabled: true)
  --category <cat>    Filter by category (powerful, versatile, lightweight)
  --family <name>     Filter by model family (e.g., gpt-5.2, claude-sonnet-4.5)
  --vendor <vendor>   Filter by vendor (OpenAI, Anthropic, Google, etc.)
  --vision            Show only models with vision support
  -h, --help          Show this help message

${BOLD}EXAMPLES:${RESET}
  # Show all available models (formatted)
  $0

  # Get raw JSON for parsing
  $0 --json

  # Show only featured models
  $0 --picker-only

  # Find powerful models
  $0 --category powerful

  # Find all Claude models
  $0 --family claude

  # Find Anthropic models with vision
  $0 --vendor Anthropic --vision

${BOLD}OUTPUT:${RESET}
  Without --json: Human-readable table format
  With --json:    Raw API response for parsing with jq

${BOLD}AUTHENTICATION:${RESET}
  Reads token from: ~/.local/share/opencode/auth.json
  
  If authentication fails, run:
    opencode auth add github-copilot

EOF
  exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --json)
      OUTPUT_JSON=true
      shift
      ;;
    --picker-only)
      PICKER_ONLY=true
      shift
      ;;
    --category)
      CATEGORY="$2"
      shift 2
      ;;
    --family)
      FAMILY="$2"
      shift 2
      ;;
    --vendor)
      VENDOR="$2"
      shift 2
      ;;
    --vision)
      VISION_ONLY=true
      shift
      ;;
    -h|--help)
      show_help
      ;;
    *)
      echo -e "${RED}Error: Unknown option: $1${RESET}" >&2
      echo "Run '$0 --help' for usage information." >&2
      exit 1
      ;;
  esac
done

# Check for required commands
for cmd in jq curl; do
  if ! command -v "$cmd" &> /dev/null; then
    echo -e "${RED}Error: Required command '$cmd' not found${RESET}" >&2
    echo "Install it with: brew install $cmd" >&2
    exit 1
  fi
done

# Get auth token from OpenCode config
AUTH_FILE="$HOME/.local/share/opencode/auth.json"
if [[ ! -f "$AUTH_FILE" ]]; then
  echo -e "${RED}Error: OpenCode auth file not found${RESET}" >&2
  echo "Expected: $AUTH_FILE" >&2
  echo "" >&2
  echo "Authenticate with: opencode auth add github-copilot" >&2
  exit 1
fi

AUTH_TOKEN=$(jq -r '.["github-copilot"].access // empty' "$AUTH_FILE")
if [[ -z "$AUTH_TOKEN" ]]; then
  echo -e "${RED}Error: GitHub Copilot token not found in auth file${RESET}" >&2
  echo "" >&2
  echo "Authenticate with: opencode auth add github-copilot" >&2
  exit 1
fi

# Fetch models from API
API_URL="https://api.githubcopilot.com/models"
RESPONSE=$(curl -s -H "Authorization: Bearer $AUTH_TOKEN" "$API_URL")

# Check for API errors
if ! echo "$RESPONSE" | jq empty 2>/dev/null; then
  echo -e "${RED}Error: Invalid JSON response from API${RESET}" >&2
  echo "$RESPONSE" >&2
  exit 1
fi

if echo "$RESPONSE" | jq -e '.message' &>/dev/null; then
  ERROR_MSG=$(echo "$RESPONSE" | jq -r '.message')
  echo -e "${RED}API Error: $ERROR_MSG${RESET}" >&2
  exit 1
fi

# If raw JSON requested, output and exit
if [[ "$OUTPUT_JSON" == true ]]; then
  echo "$RESPONSE"
  exit 0
fi

# Build jq filter based on options
JQ_FILTER=".data[]"

if [[ "$PICKER_ONLY" == true ]]; then
  JQ_FILTER="$JQ_FILTER | select(.model_picker_enabled == true)"
fi

if [[ -n "$CATEGORY" ]]; then
  JQ_FILTER="$JQ_FILTER | select(.model_picker_category == \"$CATEGORY\")"
fi

if [[ -n "$FAMILY" ]]; then
  JQ_FILTER="$JQ_FILTER | select(.capabilities.family | contains(\"$FAMILY\"))"
fi

if [[ -n "$VENDOR" ]]; then
  JQ_FILTER="$JQ_FILTER | select(.vendor == \"$VENDOR\")"
fi

if [[ "$VISION_ONLY" == true ]]; then
  JQ_FILTER="$JQ_FILTER | select(.capabilities.supports.vision == true)"
fi

# Extract filtered models
MODELS=$(echo "$RESPONSE" | jq -c "$JQ_FILTER")

if [[ -z "$MODELS" ]]; then
  echo -e "${YELLOW}No models found matching the specified criteria${RESET}"
  exit 0
fi

# Print header
echo -e "${BOLD}${BLUE}GitHub Copilot Available Models${RESET}"
echo ""

# Count models by category
POWERFUL_COUNT=$(echo "$MODELS" | jq -s '[.[] | select(.model_picker_category == "powerful")] | length')
VERSATILE_COUNT=$(echo "$MODELS" | jq -s '[.[] | select(.model_picker_category == "versatile")] | length')
LIGHTWEIGHT_COUNT=$(echo "$MODELS" | jq -s '[.[] | select(.model_picker_category == "lightweight")] | length')
TOTAL_COUNT=$(echo "$MODELS" | jq -s 'length')

echo -e "${CYAN}Total: $TOTAL_COUNT models${RESET}"
if [[ $POWERFUL_COUNT -gt 0 ]]; then echo -e "  ${MAGENTA}Powerful:${RESET} $POWERFUL_COUNT"; fi
if [[ $VERSATILE_COUNT -gt 0 ]]; then echo -e "  ${GREEN}Versatile:${RESET} $VERSATILE_COUNT"; fi
if [[ $LIGHTWEIGHT_COUNT -gt 0 ]]; then echo -e "  ${YELLOW}Lightweight:${RESET} $LIGHTWEIGHT_COUNT"; fi
echo ""

# Print models grouped by category
for cat in "powerful" "versatile" "lightweight"; do
  CAT_MODELS=$(echo "$MODELS" | jq -s "[.[] | select(.model_picker_category == \"$cat\")]")
  CAT_COUNT=$(echo "$CAT_MODELS" | jq 'length')
  
  if [[ $CAT_COUNT -eq 0 ]]; then
    continue
  fi
  
  # Category header
  case $cat in
    powerful)
      echo -e "${BOLD}${MAGENTA}━━━ POWERFUL MODELS ━━━${RESET}"
      ;;
    versatile)
      echo -e "${BOLD}${GREEN}━━━ VERSATILE MODELS ━━━${RESET}"
      ;;
    lightweight)
      echo -e "${BOLD}${YELLOW}━━━ LIGHTWEIGHT MODELS ━━━${RESET}"
      ;;
  esac
  
  # Print each model
  echo "$CAT_MODELS" | jq -r '.[] | 
    "\(.id)|\(.vendor)|\(.capabilities.limits.max_context_window_tokens)|\(.capabilities.limits.max_output_tokens)|\(.capabilities.supports.vision // false)|\(.preview // false)"
  ' | while IFS='|' read -r id vendor context output vision preview; do
    # Format vision
    if [[ "$vision" == "true" ]]; then
      VISION_ICON="📷"
    else
      VISION_ICON="  "
    fi
    
    # Format preview
    if [[ "$preview" == "true" ]]; then
      PREVIEW_TAG=" ${CYAN}[PREVIEW]${RESET}"
    else
      PREVIEW_TAG=""
    fi
    
    # Format context (add K suffix)
    CONTEXT_K=$((context / 1000))
    OUTPUT_K=$((output / 1000))
    
    echo -e "  ${VISION_ICON} ${BOLD}$id${RESET}${PREVIEW_TAG}"
    echo -e "      ${CYAN}Vendor:${RESET} $vendor  ${CYAN}Context:${RESET} ${CONTEXT_K}K  ${CYAN}Output:${RESET} ${OUTPUT_K}K"
  done
  
  echo ""
done

# Print legend
echo -e "${BOLD}${BLUE}Legend:${RESET}"
echo -e "  📷 = Vision support"
echo -e "  ${CYAN}[PREVIEW]${RESET} = Preview/experimental model"
echo ""
echo -e "${BOLD}Usage:${RESET}"
echo -e "  Switch model: ${GREEN}opencode run --model <model-id> \"your prompt\"${RESET}"
echo -e "  Full details: ${GREEN}$0 --json | jq '.data[] | select(.id == \"<model-id>\")'${RESET}"
