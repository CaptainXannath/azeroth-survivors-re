#!/bin/bash
set -euo pipefail

REPO_ROOT="$(cd "$SRCROOT/.." && pwd)"
cd "$REPO_ROOT"

rustup target add aarch64-apple-ios
cargo build --locked --release \
  --target aarch64-apple-ios \
  -p warcraft-survivors \
  --bin warcraft-survivors \
  --no-default-features

mkdir -p "$(dirname "$TARGET_BUILD_DIR/$EXECUTABLE_PATH")"
cp \
  "$REPO_ROOT/target/aarch64-apple-ios/release/warcraft-survivors" \
  "$TARGET_BUILD_DIR/$EXECUTABLE_PATH"
chmod +x "$TARGET_BUILD_DIR/$EXECUTABLE_PATH"
