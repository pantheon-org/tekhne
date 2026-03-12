#!/usr/bin/env sh
set -eu

TARGET="${1:-skills}"

find "$TARGET" -mindepth 1 -type d -empty -print -delete
