#!/usr/bin/env python3
"""Build the approved pinned native Codex reader in Fetch-private runtime storage."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import urllib.request

COMMIT = "ce21072c90d32c40af7a55b60b2fb35c84a43a6a"
ARCHIVE_SHA256 = "11e3321cd1a158dcfd920f8d24b07a08e896b9eaa7f4800f3042227b56342170"
URL = f"https://codeload.github.com/openai/codex/tar.gz/{COMMIT}"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.home() / "Library/Application Support/Fetch-Development/session-readers")
    parser.add_argument("--jobs", type=int, default=2)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parent.parent
    owned = repo / "readers/codex"
    runtime = args.root.expanduser().resolve() / "codex"
    runtime.mkdir(parents=True, exist_ok=True, mode=0o700)
    os.chmod(runtime, 0o700)
    archive = runtime / "codex-ce21072c.tar.gz"
    if not archive.exists():
        temporary = archive.with_suffix(".download")
        urllib.request.urlretrieve(URL, temporary)
        if digest(temporary) != ARCHIVE_SHA256:
            temporary.unlink()
            raise RuntimeError("official Codex source archive hash mismatch")
        temporary.replace(archive)
    if digest(archive) != ARCHIVE_SHA256:
        raise RuntimeError("cached Codex source archive hash mismatch")
    source_parent = runtime / "source"
    source = source_parent / f"codex-{COMMIT}"
    if not source.exists():
        source_parent.mkdir(exist_ok=True)
        with tarfile.open(archive) as tar:
            tar.extractall(source_parent, filter="data")
    # Check every shipped upstream file against the pinned archive. Only our two documented
    # entrypoint files may differ, and both are checked against exact expected patches below.
    exposed = {"codex-rs/thread-store/src/local/mod.rs", "codex-rs/state/src/runtime/threads.rs"}
    with tarfile.open(archive) as tar:
        for entry in tar:
            if not entry.isfile():
                continue
            relative = Path(entry.name).relative_to(f"codex-{COMMIT}")
            if str(relative) in exposed:
                continue
            actual = source / relative
            if not actual.is_file() or hashlib.sha256(actual.read_bytes()).digest() != hashlib.sha256(tar.extractfile(entry).read()).digest():
                raise RuntimeError(f"private upstream file differs from pinned archive: {relative}")
    # Deterministic visibility/entrypoint patch, with a pristine pinned-file comparison.
    local_path = "codex-rs/thread-store/src/local/mod.rs"
    with tarfile.open(archive) as tar:
        pristine = tar.extractfile(f"codex-{COMMIT}/{local_path}").read().decode()
    anchor = "impl LocalThreadStore {\n"
    assert pristine.count(anchor) == 1
    patched = pristine.replace(anchor, anchor + (owned / "materialize-entrypoint.rs").read_text(), 1)
    local = source / local_path
    if local.read_text() not in (pristine, patched):
        raise RuntimeError("private upstream entrypoint has unexpected changes; refusing to overwrite")
    if local.read_text() != patched:
        local.write_text(patched)
    # Expose the exact official metadata query/decoder over a read-only pool. No migrations.
    threads_path = "codex-rs/state/src/runtime/threads.rs"
    with tarfile.open(archive) as tar:
        original_threads = tar.extractfile(f"codex-{COMMIT}/{threads_path}").read().decode()
    start = original_threads.index("    pub async fn get_thread(")
    end = original_threads.index("\n    /// Permanently promote", start)
    official = original_threads[start:end]
    readonly = official.replace(
        "pub async fn get_thread(&self, id: ThreadId)",
        "pub async fn fetch_get_thread_readonly(path: &std::path::Path, id: ThreadId)",
    ).replace(
        "        let row = sqlx::query(",
        "        let options = sqlx::sqlite::SqliteConnectOptions::new().filename(path).read_only(true).create_if_missing(false);\n"
        "        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect_with(options).await?;\n"
        "        let row = sqlx::query(",
    ).replace(".fetch_optional(self.pool.as_ref())", ".fetch_optional(&pool)")
    assert readonly != official
    patched_threads = original_threads[:end] + "\n" + readonly + original_threads[end:]
    threads_file = source / threads_path
    if threads_file.read_text() not in (original_threads, patched_threads):
        raise RuntimeError("private upstream read-only metadata entrypoint has unexpected changes")
    if threads_file.read_text() != patched_threads:
        threads_file.write_text(patched_threads)
    build = runtime / "build"
    build.mkdir(exist_ok=True)
    manifest = (owned / "Cargo.toml.in").read_text().replace("@SOURCE@", str(source).replace("\\", "\\\\").replace('"', '\\"'))
    (build / "Cargo.toml").write_text(manifest)
    (build / "src").mkdir(exist_ok=True)
    shutil.copy2(owned / "src/main.rs", build / "src/main.rs")
    # A private build cache; neither normal Cargo state nor the Fetch app workspace is modified.
    env = os.environ.copy()
    rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
    cargo = subprocess.check_output(["rustup", "which", "cargo"], text=True).strip()
    env.update(CARGO_HOME=str(runtime / "cargo-home"), CARGO_TARGET_DIR=str(runtime / "target"), RUSTC=rustc)
    subprocess.run([cargo, "build", "--manifest-path", str(build / "Cargo.toml"), "--jobs", str(args.jobs)], cwd=build, env=env, check=True)
    executable = runtime / "fetch-codex-session-reader"
    new_executable = executable.with_suffix(".new")
    shutil.copy2(runtime / "target/debug/fetch-codex-session-reader", new_executable)
    new_executable.chmod(0o700)
    new_executable.replace(executable)
    provenance = {
        "upstream": "https://github.com/openai/codex", "tag": "rust-v0.167.2", "commit": COMMIT,
        "archive_sha256": ARCHIVE_SHA256, "license": "Apache-2.0",
        "upstream_license": str(source / "LICENSE"), "upstream_notice": str(source / "NOTICE"),
        "entrypoint_patch_sha256": digest(owned / "materialize-entrypoint.rs"),
        "helper_source_sha256": digest(owned / "src/main.rs"), "rustc": rustc,
        "executable": str(executable), "protocol": "one JSON stdin request / one JSON stdout response",
    }
    (runtime / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
    print(f"Installed {executable}")


if __name__ == "__main__":
    main()
