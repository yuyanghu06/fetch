#!/bin/sh
# Explicitly prepare both saved-history readers under one Fetch-private root.
set -eu
if [ "$#" -ne 2 ] || [ "$1" != "--root" ]; then
    echo 'Usage: scripts/setup-session-readers.sh --root "/absolute/Fetch-data/session-readers"' >&2
    exit 2
fi
reader_root=$2
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
python3 "$script_dir/setup-codex-session-reader.py" --root "$reader_root"
python3 "$script_dir/setup-claude-session-reader.py" --root "$reader_root"
