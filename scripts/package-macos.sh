#!/bin/sh
# Source-only in D77. The user must explicitly start packaging after verification.
set -eu
if [ "${FETCH_ALLOW_PACKAGING:-}" != "1" ]; then
  echo 'Packaging is disabled. After you authorize and finish testing, set FETCH_ALLOW_PACKAGING=1 to run this script.' >&2
  exit 1
fi
case "$(uname -s)" in Darwin) ;; *) echo 'macOS is required to create Fetch.app and its DMG.' >&2; exit 1;; esac
repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
fetch_binary=${FETCH_BINARY:-"$repo_dir/target/release/fetch"}
output_dir=${FETCH_PACKAGE_OUTPUT:-"$repo_dir/dist"}
if [ ! -f "$fetch_binary" ] || [ ! -x "$fetch_binary" ]; then
  echo 'Build the release binary yourself first. This script never starts a build or test.' >&2
  exit 1
fi
if [ -e "$output_dir/Fetch.app" ] || [ -e "$output_dir/Fetch.dmg" ]; then
  echo 'The output already contains Fetch.app or Fetch.dmg. Choose a fresh FETCH_PACKAGE_OUTPUT directory.' >&2
  exit 1
fi
mkdir -p "$output_dir"
staging_dir=$(mktemp -d "$output_dir/.fetch-package.XXXXXX")
trap 'rm -rf -- "$staging_dir"' EXIT HUP INT TERM
app_dir="$staging_dir/Fetch.app"
mkdir -p "$app_dir/Contents/MacOS" "$app_dir/Contents/Resources"
cp "$fetch_binary" "$app_dir/Contents/MacOS/fetch"
cp "$repo_dir/packaging/Info.plist" "$app_dir/Contents/Info.plist"
cp "$repo_dir/LICENSE" "$repo_dir/THIRD_PARTY_NOTICES.md" "$app_dir/Contents/Resources/"
cp "$repo_dir/docs/testing/HANDOFF.md" "$app_dir/Contents/Resources/TESTING-HANDOFF.md"
if [ -n "${FETCH_SIGNING_IDENTITY:-}" ]; then
  codesign --force --options runtime --timestamp --sign "$FETCH_SIGNING_IDENTITY" "$app_dir"
fi
ln -s /Applications "$staging_dir/Applications"
hdiutil create -volname Fetch -srcfolder "$staging_dir" -format UDZO -ov "$output_dir/Fetch.dmg"
cp -R "$app_dir" "$output_dir/Fetch.app"
echo "Created $output_dir/Fetch.app and $output_dir/Fetch.dmg. No release, notarization or publication was performed."
