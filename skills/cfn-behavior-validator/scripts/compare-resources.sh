#!/usr/bin/env bash
# Compare CloudFormation Resource Properties
# Compares a specific resource's properties before and after a change

set -euo pipefail

STACK_NAME="${1:-}"
LOGICAL_RESOURCE_ID="${2:-}"

if [ -z "$STACK_NAME" ] || [ -z "$LOGICAL_RESOURCE_ID" ]; then
  echo "Usage: $0 <stack-name> <logical-resource-id>"
  echo ""
  echo "Example: $0 LctMonitoringStack-ST ProcessAlertEmailSubscription"
  echo ""
  echo "This script captures the current state of a resource."
  echo "Run it before and after a deployment to compare changes."
  exit 1
fi

OUTPUT_DIR=".context/cfn-resource-snapshots"
mkdir -p "$OUTPUT_DIR"

TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
OUTPUT_FILE="$OUTPUT_DIR/${STACK_NAME}_${LOGICAL_RESOURCE_ID}_${TIMESTAMP}.json"

echo "Capturing resource state for: $LOGICAL_RESOURCE_ID"
echo "Stack: $STACK_NAME"

# Get physical resource ID
PHYSICAL_ID=$(aws cloudformation describe-stack-resource \
  --stack-name "$STACK_NAME" \
  --logical-resource-id "$LOGICAL_RESOURCE_ID" \
  --query 'StackResourceDetail.PhysicalResourceId' \
  --output text 2>/dev/null || echo "NOT_FOUND")

if [ "$PHYSICAL_ID" = "NOT_FOUND" ]; then
  echo "❌ Resource not found in stack"
  exit 1
fi

echo "Physical Resource ID: $PHYSICAL_ID"

# Get full resource details
RESOURCE_DETAILS=$(aws cloudformation describe-stack-resource \
  --stack-name "$STACK_NAME" \
  --logical-resource-id "$LOGICAL_RESOURCE_ID" \
  --output json)

# Save to file
echo "$RESOURCE_DETAILS" | jq '.' > "$OUTPUT_FILE"

echo "✅ Resource state captured to: $OUTPUT_FILE"
echo ""
echo "Resource Details:"
echo "$RESOURCE_DETAILS" | jq '{
  LogicalResourceId: .StackResourceDetail.LogicalResourceId,
  PhysicalResourceId: .StackResourceDetail.PhysicalResourceId,
  ResourceType: .StackResourceDetail.ResourceType,
  ResourceStatus: .StackResourceDetail.ResourceStatus,
  LastUpdatedTimestamp: .StackResourceDetail.LastUpdatedTimestamp,
  Metadata: .StackResourceDetail.Metadata
}'

# Compare with previous snapshot if exists
PREVIOUS_SNAPSHOTS=$(ls -t "$OUTPUT_DIR/${STACK_NAME}_${LOGICAL_RESOURCE_ID}"_*.json 2>/dev/null | tail -n +2 || echo "")

if [ -n "$PREVIOUS_SNAPSHOTS" ]; then
  PREVIOUS_SNAPSHOT=$(echo "$PREVIOUS_SNAPSHOTS" | head -1)
  echo ""
  echo "📊 Comparing with previous snapshot: $(basename "$PREVIOUS_SNAPSHOT")"
  echo ""
  
  # Compare Physical Resource IDs
  PREV_PHYSICAL_ID=$(jq -r '.StackResourceDetail.PhysicalResourceId' "$PREVIOUS_SNAPSHOT")
  
  if [ "$PHYSICAL_ID" != "$PREV_PHYSICAL_ID" ]; then
    echo "⚠️  Physical Resource ID CHANGED (resource was replaced)"
    echo "   Previous: $PREV_PHYSICAL_ID"
    echo "   Current:  $PHYSICAL_ID"
  else
    echo "✅ Physical Resource ID unchanged (resource was updated in place)"
    echo "   ID: $PHYSICAL_ID"
  fi
  
  # Compare timestamps
  PREV_TIMESTAMP=$(jq -r '.StackResourceDetail.LastUpdatedTimestamp' "$PREVIOUS_SNAPSHOT")
  CURR_TIMESTAMP=$(jq -r '.StackResourceDetail.LastUpdatedTimestamp' "$OUTPUT_FILE")
  
  echo ""
  echo "Last Updated:"
  echo "   Previous: $PREV_TIMESTAMP"
  echo "   Current:  $CURR_TIMESTAMP"
  
  # Full diff
  echo ""
  echo "Full Diff:"
  diff <(jq -S '.' "$PREVIOUS_SNAPSHOT") <(jq -S '.' "$OUTPUT_FILE") || true
fi

echo ""
echo "💡 Run this script again after deployment to see changes"
