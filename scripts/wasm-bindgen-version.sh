#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cargo metadata --format-version 1 --manifest-path "$repo_root/Cargo.toml" --locked \
  | jq -er '[.packages[] | select(.name == "wasm-bindgen") | .version] | if length == 1 then .[0] else error("expected one wasm-bindgen version, found \(length)") end'
