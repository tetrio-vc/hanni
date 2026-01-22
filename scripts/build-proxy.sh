#!/usr/bin/env bash
set -e

BIN="socks5-proxy"
TARGET="${TARGET:-$(rustc -vV | sed -n 's/^host: //p')}"

cargo build -p "$BIN" --target "$TARGET"

mkdir -p src-tauri/bin
mv "target/$TARGET/debug/$BIN" \
   "src-tauri/bin/$BIN-$TARGET"
