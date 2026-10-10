#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

browser="${1:?Specify firefox or chrome}"
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
  chrome)
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

has_wasm_test_dependency() {
  awk '
    /^[[:space:]]*\[/ {
      header = $0
      sub(/[[:space:]]*#.*$/, "", header)
      gsub(/^[[:space:]]*\[[[:space:]]*|[[:space:]]*\][[:space:]]*$/, "", header)
      in_dev_dependencies = (header ~ /(^|\.)dev-dependencies$/)
      if (header ~ /(^|\.)dev-dependencies\.wasm-bindgen-test$/) found = 1
      next
    }
    in_dev_dependencies && /^[[:space:]]*wasm-bindgen-test([[:space:]]*\.[^=]*)?[[:space:]]*=/ { found = 1 }
    END { exit !found }
  ' "$1"
}

manifests=(Cargo.toml)
for example_manifest in examples/*/Cargo.toml; do
  if has_wasm_test_dependency "$example_manifest"; then
    manifests+=("$example_manifest")
  fi
done
if ((${#manifests[@]} < 2)); then
  echo "No example manifest lists wasm-bindgen-test under dev-dependencies" >&2
  exit 1
fi

webdriver_files=()
cleanup() {
  if ((${#webdriver_files[@]})); then
    rm -f "${webdriver_files[@]}"
  fi
  rm -f "${capability_log:-}"
}
trap cleanup EXIT

if [[ -n "${BROWSER_BINARY:-}" ]]; then
  [[ -f "$BROWSER_BINARY" ]] || {
    echo "BROWSER_BINARY is not a file: $BROWSER_BINARY" >&2
    exit 1
  }
  case "$browser" in
    firefox)
      capabilities="$(printf '{"moz:firefoxOptions": {"binary": "%s"}}' "$BROWSER_BINARY")"
      ;;
    chrome)
      capabilities="$(printf '{"goog:chromeOptions": {"binary": "%s", "args": ["--no-sandbox"]}}' "$BROWSER_BINARY")"
      ;;
  esac
  for manifest in "${manifests[@]}"; do
    webdriver_file="$repo_root/$(dirname "$manifest")/webdriver.json"
    if [[ -e "$webdriver_file" ]]; then
      echo "Refusing to overwrite existing $webdriver_file. Remove it or unset BROWSER_BINARY." >&2
      exit 1
    fi
    printf '%s\n' "$capabilities" > "$webdriver_file"
    webdriver_files+=("$webdriver_file")
  done
  echo "Browser binary: $BROWSER_BINARY"
else
  echo "Browser binary: not set; webdriver.json files are not written"
fi

for manifest in "${manifests[@]}"; do
  echo "Testing $manifest"
  if [[ "$manifest" == Cargo.toml ]]; then
    capability_log="$(mktemp)"
    cargo test --locked --manifest-path "$repo_root/$manifest" \
      --target wasm32-unknown-unknown ${profile_args[@]+"${profile_args[@]}"} -- --nocapture | tee "$capability_log"
    if grep -Fq 'native move capability: supported' "$capability_log"; then
      echo "Native move capability: supported; assertions exercised"
    elif grep -Fq 'native move capability: unsupported' "$capability_log"; then
      if [[ "${ALLOW_MISSING_MOVE:-}" == 1 ]]; then
        echo "Native move capability: unsupported; allowed by ALLOW_MISSING_MOVE=1; assertions not exercised"
      else
        echo "Native move capability: unsupported; set ALLOW_MISSING_MOVE=1 to allow this" >&2
        exit 1
      fi
    else
      echo "Native move capability: no result from root suite" >&2
      exit 1
    fi
  else
    cargo test --locked --manifest-path "$repo_root/$manifest" \
      --target wasm32-unknown-unknown ${profile_args[@]+"${profile_args[@]}"}
  fi
done
