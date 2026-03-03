#!/usr/bin/env sh
# Prune old audit directories, keeping only the most recent ones
# Usage: ./prune-audits.sh [--keep <num>]

set -e

AUDIT_ROOT=".context/audits/skill-audit"
KEEP=5

while [ "$#" -gt 0 ]; do
  case "$1" in
    --keep)
      if [ "$#" -ge 2 ]; then
        KEEP="$2"
        shift 2
      fi
      ;;
    --help|-h)
      echo "Usage: $0 [--keep <num>]"
      echo "  --keep <num>  Number of audits to keep (default: 5)"
      exit 0
      ;;
    *)
      shift
      ;;
  esac
done

if [ ! -d "$AUDIT_ROOT" ]; then
  echo "No audit directory found: $AUDIT_ROOT"
  exit 0
fi

echo "=== Audit Pruning ==="
echo "Keeping last $KEEP audits"
echo

DATES_DIRS=$(find "$AUDIT_ROOT" -mindepth 1 -maxdepth 1 -type d -not -name 'latest' | sort -r)

COUNT=0
for dir in $DATES_DIRS; do
  COUNT=$((COUNT + 1))
  if [ "$COUNT" -gt "$KEEP" ]; then
    echo "Removing: $dir"
    rm -rf "$dir"
  else
    echo "Keeping: $dir"
  fi
done

if [ -L "$AUDIT_ROOT/latest" ] || [ -d "$AUDIT_ROOT/latest" ]; then
  LATEST_TARGET=$(readlink "$AUDIT_ROOT/latest" 2>/dev/null || echo "directory")
  echo "Latest symlink points to: $LATEST_TARGET"
fi

echo
echo "Done. Kept $((COUNT > KEEP ? KEEP : COUNT)) of $COUNT audits."
