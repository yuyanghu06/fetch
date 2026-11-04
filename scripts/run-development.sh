#!/usr/bin/env bash
set -euo pipefail

repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$repo_dir"

build_only=false
if [[ "${1:-}" == "--build-only" ]]; then
  build_only=true
  shift
fi

fail() {
  printf '%s\n' "Fetch development: $*" >&2
  exit 1
}

identity=${FETCH_DEV_SIGNING_IDENTITY:-}
if [[ -z "$identity" ]]; then
  identity_file="${HOME:?Current home directory is required}/Library/Application Support/Fetch-Development/development-signing-identity"
  [[ -r "$identity_file" ]] || fail "Set FETCH_DEV_SIGNING_IDENTITY to the development certificate's 40-digit fingerprint, or save it in $identity_file."
  identity=$(< "$identity_file")
fi
[[ "$identity" =~ ^[0-9a-fA-F]{40}$ ]] || fail "The development signing identity must be exactly 40 hexadecimal digits."

target_dir=${CARGO_TARGET_DIR:-"$repo_dir/target"}
case "$target_dir" in
  /*) ;;
  *) target_dir="$repo_dir/$target_dir" ;;
esac

cargo build --locked --bin fetch
debug_dir=$(cd -- "$target_dir/debug" && pwd -P)
launch_binary="$debug_dir/fetch-development"
staged=$(mktemp "$debug_dir/.fetch-development.XXXXXX")
cleanup() {
  if [[ -n "$staged" ]]; then
    rm -f -- "$staged"
  fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

cp -- "$debug_dir/fetch" "$staged"
chmod 700 "$staged"
/usr/bin/codesign --force --sign "$identity" \
  --identifier com.fetch.local.development --timestamp=none "$staged" \
  || fail "Signing failed; the previous development executable was preserved."
/usr/bin/codesign --verify --strict "$staged" \
  || fail "Signature verification failed; the previous development executable was preserved."
mv -f -- "$staged" "$launch_binary"
staged=
printf '%s\n' "Signed Fetch development executable: $launch_binary"

if "$build_only"; then
  exit 0
fi
exec "$launch_binary" app "$@"
