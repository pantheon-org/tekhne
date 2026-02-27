#!/usr/bin/env sh
set -e

# Skill Management Automation Script
# Automatically manages skills lifecycle: import -> lint -> review -> publish

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE="pantheon-ai"
SKILLS_DIR="skills"

# Logging functions
log_info() {
    printf "${BLUE}[INFO]${NC} %s\n" "$1"
}

log_success() {
    printf "${GREEN}[SUCCESS]${NC} %s\n" "$1"
}

log_warning() {
    printf "${YELLOW}[WARNING]${NC} %s\n" "$1"
}

log_error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
}

# Check if tessl is available
check_tessl() {
    if ! command -v tessl >/dev/null 2>&1; then
        log_error "tessl command not found. Please install tessl first."
        exit 1
    fi
}

# Import skill without tile.json
import_skill() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    log_info "Importing skill: $skill_name"
    
    if tessl skill import "$skill_path" --workspace="$WORKSPACE"; then
        log_success "Successfully imported $skill_name"
        return 0
    else
        log_error "Failed to import $skill_name"
        return 1
    fi
}

# Lint skill with tile.json
lint_skill() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    log_info "Linting skill: $skill_name"
    
    if tessl skill lint "$skill_path"; then
        log_success "Linting passed for $skill_name"
        return 0
    else
        log_error "Linting failed for $skill_name"
        return 1
    fi
}

# Review skill with tile.json
review_skill() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    log_info "Reviewing skill: $skill_name"
    
    if tessl skill review "$skill_path"; then
        log_success "Review passed for $skill_name"
        return 0
    else
        log_error "Review failed for $skill_name"
        return 1
    fi
}

# Check if skill is already published
is_skill_published() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    # Try to get skill info to see if it's published
    # This is a placeholder - adjust based on tessl's actual API
    if tessl skill info "$skill_name" --workspace="$WORKSPACE" >/dev/null 2>&1; then
        return 0  # Already published
    else
        return 1  # Not published
    fi
}

# Publish skill
publish_skill() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    if is_skill_published "$skill_path"; then
        log_info "Skill $skill_name is already published"
        return 0
    fi
    
    log_info "Publishing skill: $skill_name"
    
    if tessl skill publish "$skill_path"; then
        log_success "Successfully published $skill_name"
        return 0
    else
        log_error "Failed to publish $skill_name"
        return 1
    fi
}

# Process a single skill
process_skill() {
    skill_path="$1"
    skill_name="$(basename "$skill_path")"
    
    # Check if skill directory exists and has SKILL.md
    if [ ! -d "$skill_path" ] || [ ! -f "$skill_path/SKILL.md" ]; then
        log_warning "Skipping $skill_name - not a valid skill directory"
        return 1
    fi
    
    log_info "Processing skill: $skill_name"
    
    if [ -f "$skill_path/tile.json" ]; then
        # Skill has tile.json - lint, review, and publish
        log_info "$skill_name has tile.json - running lint and review"
        
        if lint_skill "$skill_path" && review_skill "$skill_path"; then
            log_success "$skill_name passed linting and review"
            publish_skill "$skill_path"
        else
            log_error "$skill_name failed linting or review - skipping publish"
            return 1
        fi
    else
        # Skill doesn't have tile.json - import it
        log_info "$skill_name missing tile.json - importing"
        import_skill "$skill_path"
    fi
}

# Main execution
main() {
    log_info "Starting skill management automation..."
    
    # Check prerequisites
    check_tessl
    
    # Change to repository root if we're not there
    if [ ! -d "$SKILLS_DIR" ]; then
        log_error "Skills directory not found. Please run this script from the repository root."
        exit 1
    fi
    
    # Process each skill
    success_count=0
    error_count=0
    
    for skill_dir in "$SKILLS_DIR"/*; do
        if [ -d "$skill_dir" ]; then
            if process_skill "$skill_dir"; then
                success_count=$((success_count + 1))
            else
                error_count=$((error_count + 1))
            fi
        fi
    done
    
    # Summary
    echo
    log_info "=== SUMMARY ==="
    log_success "Successfully processed: $success_count skills"
    if [ $error_count -gt 0 ]; then
        log_error "Failed to process: $error_count skills"
    fi
    
    if [ $error_count -eq 0 ]; then
        log_success "All skills processed successfully! 🎉"
        exit 0
    else
        log_warning "Some skills had issues. Check the output above for details."
        exit 1
    fi
}

# Help function
show_help() {
    cat << EOF
Skill Management Automation Script

USAGE:
    $0 [OPTIONS] [SKILL_NAME]

OPTIONS:
    -h, --help      Show this help message
    -w, --workspace Set tessl workspace (default: pantheon-ai)

EXAMPLES:
    $0                          # Process all skills
    $0 agents-md               # Process only the agents-md skill
    $0 --workspace=my-org      # Use different workspace

DESCRIPTION:
    This script automates the tessl skill lifecycle:
    1. Skills without tile.json are imported using 'tessl skill import'
    2. Skills with tile.json are linted and reviewed
    3. Skills that pass linting and review are published (if not already)

EOF
}

# Parse command line arguments
while [ $# -gt 0 ]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -w|--workspace)
            WORKSPACE="$2"
            shift 2
            ;;
        --workspace=*)
            WORKSPACE="${1#*=}"
            shift
            ;;
        -*)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
        *)
            # Process specific skill
            if [ -d "$SKILLS_DIR/$1" ]; then
                process_skill "$SKILLS_DIR/$1"
                exit $?
            else
                log_error "Skill '$1' not found in $SKILLS_DIR/"
                exit 1
            fi
            ;;
    esac
done

# Run main function if no specific skill was requested
main