#!/usr/bin/env bash
# CloudFormation Event Watcher
# Watches CloudFormation events in real-time during stack updates

set -euo pipefail

STACK_NAME="${1:-}"
WATCH_INTERVAL="${2:-5}"

if [ -z "$STACK_NAME" ]; then
  echo "Usage: $0 <stack-name> [watch-interval-seconds]"
  echo ""
  echo "Example: $0 LctMonitoringStack-ST 5"
  exit 1
fi

echo "Watching CloudFormation events for stack: $STACK_NAME"
echo "Refresh interval: ${WATCH_INTERVAL} seconds"
echo "Press Ctrl+C to stop"
echo ""
echo "----------------------------------------"

LAST_TIMESTAMP=""

while true; do
  # Get latest events
  EVENTS=$(aws cloudformation describe-stack-events \
    --stack-name "$STACK_NAME" \
    --max-items 20 \
    --query 'StackEvents[*].[Timestamp,LogicalResourceId,ResourceType,ResourceStatus,ResourceStatusReason]' \
    --output text 2>/dev/null || echo "")

  if [ -z "$EVENTS" ]; then
    echo "No events found or stack doesn't exist"
    sleep "$WATCH_INTERVAL"
    continue
  fi

  # Filter events newer than last seen
  NEW_EVENTS=$(echo "$EVENTS" | while read -r line; do
    TIMESTAMP=$(echo "$line" | awk '{print $1}')
    if [ -z "$LAST_TIMESTAMP" ] || [[ "$TIMESTAMP" > "$LAST_TIMESTAMP" ]]; then
      echo "$line"
    fi
  done)

  if [ -n "$NEW_EVENTS" ]; then
    echo "$NEW_EVENTS" | while read -r line; do
      TIMESTAMP=$(echo "$line" | awk '{print $1"T"$2}')
      RESOURCE=$(echo "$line" | awk '{print $3}')
      TYPE=$(echo "$line" | awk '{print $4}')
      STATUS=$(echo "$line" | awk '{print $5}')
      REASON=$(echo "$line" | cut -d$'\t' -f6-)

      # Color code based on status
      case "$STATUS" in
        *COMPLETE*)
          COLOR="\033[0;32m" # Green
          ;;
        *FAILED* | *ROLLBACK*)
          COLOR="\033[0;31m" # Red
          ;;
        *IN_PROGRESS*)
          COLOR="\033[0;33m" # Yellow
          ;;
        *)
          COLOR="\033[0m" # No color
          ;;
      esac

      printf "${COLOR}%-20s %-30s %-25s %-20s %s\033[0m\n" \
        "$TIMESTAMP" "$RESOURCE" "$TYPE" "$STATUS" "$REASON"
    done

    # Update last timestamp
    LAST_TIMESTAMP=$(echo "$NEW_EVENTS" | head -1 | awk '{print $1}')
  fi

  sleep "$WATCH_INTERVAL"
done
