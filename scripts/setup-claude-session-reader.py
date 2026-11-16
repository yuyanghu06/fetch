#!/usr/bin/env python3
"""Explicit installation of Fetch's private pinned Claude session reader."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path,
                        help="The absolute session-readers folder under Fetch's actual data folder.")
    parser.add_argument("--data-dir", type=Path,
                        help="Fetch data folder (alternative to --root).")
    args = parser.parse_args()
    if (args.root is None) == (args.data_dir is None):
        parser.error("Provide --root or --data-dir, once.")
    root = args.root.expanduser().resolve() if args.root else args.data_dir.expanduser().resolve() / "session-readers"
    root.mkdir(parents=True, exist_ok=True, mode=0o700)
    uv = shutil.which("uv")
    if not uv:
        parser.error("uv is required. Install uv before setting up this reader.")
    env = os.environ.copy()
    env.update(UV_CACHE_DIR=str(root / "setup-cache"),
               UV_PYTHON_INSTALL_DIR=str(root / "python"))
    target = root / "claude"
    stage = Path(tempfile.mkdtemp(prefix=".claude-setup-", dir=root))
    backup = root / ".claude-previous"
    try:
        subprocess.run([uv, "venv", "--python", "3.12", "--managed-python",
                        "--relocatable", str(stage)], env=env, check=True)
        subprocess.run([uv, "pip", "install", "--python", str(stage / "bin/python"),
                        "claude-agent-sdk==0.2.181"], env=env, check=True)
        source = Path(__file__).resolve().parent.parent / "readers/claude/helper.py"
        shutil.copy2(source, stage / "helper.py")
        subprocess.run([str(stage / "bin/python"), "-I", "-c",
                        "import importlib.metadata; from claude_agent_sdk._internal.sessions import "
                        "_parse_transcript_entries, _build_conversation_chain, _is_visible_message; "
                        "assert importlib.metadata.version('claude-agent-sdk') == '0.2.181'"],
                       env=env, check=True)
        (stage / "provenance.json").write_text(json.dumps({
            "package": "claude-agent-sdk", "version": "0.2.181", "license": "MIT",
            "source_commit": "22b8eb9c19adfa78759b79e22bd664a57843bb06",
            "source": "https://github.com/anthropics/claude-agent-sdk-python"}, indent=2) + "\n")
        if backup.exists():
            parser.error(f"Preserve or remove the previous setup backup at {backup} first.")
        if target.exists():
            target.rename(backup)
        try:
            stage.rename(target)
        except BaseException:
            if backup.exists():
                backup.rename(target)
            raise
        shutil.rmtree(backup, ignore_errors=True)
        print(f"Installed Claude session reader: {target / 'helper.py'}")
    finally:
        shutil.rmtree(stage, ignore_errors=True)


if __name__ == "__main__":
    main()
