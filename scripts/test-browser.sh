#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

browser="${1:?Specify firefox or chromium}"
profile="${2:?Specify debug or release}"

unset NO_HEADLESS SAFARIDRIVER SAFARIDRIVER_REMOTE MSEDGEDRIVER MSEDGEDRIVER_REMOTE
unset CHROMEDRIVER_REMOTE GECKODRIVER_REMOTE

binding_version="$(wasm-bindgen --version)"
runner_version="$(wasm-bindgen-test-runner --version)"
[[ "$binding_version" == "wasm-bindgen 0.2.100"* ]] || {
  echo "Expected wasm-bindgen 0.2.100, found: $binding_version" >&2
  exit 1
}
[[ "$runner_version" == "wasm-bindgen-test-runner 0.2.100"* ]] || {
  echo "Expected wasm-bindgen-test-runner 0.2.100, found: $runner_version" >&2
  exit 1
}

case "$browser" in
  firefox)
    unset CHROMEDRIVER CHROMEDRIVER_REMOTE
    driver_path="$(command -v geckodriver)" || exit 1
    [[ -n "$driver_path" ]] || exit 1
    export GECKODRIVER="$driver_path"
    ;;
  chromium)
    unset GECKODRIVER GECKODRIVER_REMOTE
    driver_path="$(command -v chromedriver)" || exit 1
    [[ -n "$driver_path" ]] || exit 1
    export CHROMEDRIVER="$driver_path"
    ;;
  *) echo "Unsupported browser: $browser" >&2; exit 2 ;;
esac

profile_args=()
case "$profile" in
  debug) ;;
  release) profile_args+=(--release) ;;
  *) echo "Unsupported profile: $profile" >&2; exit 2 ;;
esac

echo "Browser: $browser; profile: $profile"
echo "$binding_version"
echo "$runner_version"
"$driver_path" --version

manifests=(
  Cargo.toml
  examples/counter/Cargo.toml
  examples/cell_state/Cargo.toml
  examples/refcell_state/Cargo.toml
)
for manifest in "${manifests[@]}"; do
  echo "Testing $manifest"
  if [[ "$manifest" == Cargo.toml ]]; then
    capability_log="$(mktemp)"
    trap 'rm -f "$capability_log"' EXIT
    cargo test --locked --manifest-path "$repo_root/$manifest" \
      --target wasm32-unknown-unknown "${profile_args[@]}" -- --nocapture | tee "$capability_log"
    if grep -Fq 'native move capability: supported' "$capability_log"; then
      echo "Native move capability: supported; assertions exercised"
    elif grep -Fq 'native move capability: unsupported' "$capability_log"; then
      echo "Native move capability: unsupported; assertions not exercised"
    else
      echo "Native move capability: no result from root suite" >&2
      exit 1
    fi
    rm -f "$capability_log"
    trap - EXIT
  else
    cargo test --locked --manifest-path "$repo_root/$manifest" \
      --target wasm32-unknown-unknown "${profile_args[@]}"
  fi
done
