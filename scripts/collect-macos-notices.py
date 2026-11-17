#!/usr/bin/env python3
"""Collect the notices for a Fetch binary built from this checkout's Cargo.lock."""

import argparse
from concurrent.futures import ThreadPoolExecutor
from functools import cache
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
from urllib.error import HTTPError
from urllib.request import urlopen


REPO = Path(__file__).resolve().parent.parent
NOTICE_NAME = re.compile(r"(licen[cs]e|copying|notice|copyright|unlicense|ofl)(?:$|[._-])", re.I)
UPSTREAM_NOTICES = {
    "aclysma/profiling": ("LICENSE-APACHE", "LICENSE-MIT"),
    "etemesi254/zune-image": ("LICENSE.md", "LICENSE-ZLIB"),
    "gfx-rs/rspirv": ("LICENSE",),
    "iced-rs/iced": ("LICENSE",),
    "madsmtm/objc2": ("LICENSE.md", "LICENSE.txt"),
    "nical/lyon": ("LICENSE-APACHE", "LICENSE-MIT"),
    "nical/rust_debug": ("LICENSE",),
    "rusticata/asn1-rs": ("LICENSE-APACHE", "LICENSE-MIT"),
    "zakarumych/gpu-alloc": ("COPYING", "license/APACHE", "license/MIT"),
    "zakarumych/gpu-descriptor": ("COPYING", "license/APACHE", "license/MIT"),
}
# These exact published revisions declare a license in Cargo.toml and have no
# standalone license text in either the crate archive or upstream repository.
MANIFEST_ONLY = {
    ("block", "0.1.6"): "https://github.com/SSheldon/rust-block/tree/47178790cfc9d4a8b092051d8b413b78bd31254a",
    ("dispatch", "0.2.0"): "https://github.com/SSheldon/rust-dispatch/tree/82d6c7a5b75dc0c71c3f46f87bb6c16a476f7748",
    ("hexf-parse", "0.2.1"): "https://github.com/lifthrasiir/hexf/tree/4225763d744183d720f575ae96d04161b4d08ea0",
    ("malloc_buf", "0.0.6"): "https://github.com/SSheldon/malloc_buf/tree/a7811e5f4c6f9685dd57ad9bfe34e9bf6b0ba4f9",
}
SPDX = "https://raw.githubusercontent.com/spdx/license-list-data/v3.27.0"


def release_dependencies(target):
    data = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--offline", "--format-version", "1", "--filter-platform", target],
        cwd=REPO,
    ))
    resolve = data["resolve"]
    nodes = {n["id"]: n for n in resolve["nodes"]}
    packages = {p["id"]: p for p in data["packages"]}
    todo = [resolve["root"]]
    seen = set()
    while todo:
        package = todo.pop()
        if package in seen:
            continue
        seen.add(package)
        todo.extend(dep["pkg"] for dep in nodes[package]["deps"]
                    if any(kind["kind"] != "dev" for kind in dep["dep_kinds"]))
    seen.remove(resolve["root"])
    return sorted((packages[ident] for ident in seen), key=lambda p: (p["name"], p["version"]))


@cache
def download(url):
    try:
        with urlopen(url, timeout=20) as response:
            return response.read()
    except HTTPError as error:
        if error.code != 404:
            raise
        return None


def local_notices(directory):
    for path in sorted(directory.rglob("*")):
        relative = path.relative_to(directory)
        if path.is_file() and not path.name.endswith(".rs") and (
            NOTICE_NAME.match(path.name) or {part.lower() for part in relative.parts[:-1]} & {"license", "licenses"}
        ):
            yield relative


def copy_dependency(package, directory):
    name, version = package["name"], package["version"]
    label = f"{name}-{version}"
    source = Path(package["manifest_path"]).parent
    destination = directory / "crates" / label
    destination.mkdir(parents=True)
    paths = set(local_notices(source))
    if package["license_file"]:
        paths.add(Path(package["license_file"]))
    for path in sorted(paths):
        output = destination / (path.name if path.is_absolute() or ".." in path.parts else path)
        output.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source / path, output)

    urls = []
    if not paths:
        manifest_only = MANIFEST_ONLY.get((name, version))
        if manifest_only:
            shutil.copyfile(source / "Cargo.toml", destination / "Cargo.toml")
            urls = [manifest_only]
        else:
            repository = package["repository"] or ""
            match = re.match(r"https?://github\.com/([\w.-]+/[\w.-]+)", repository)
            repo = match[1].removesuffix(".git") if match else ""
            if (name, version) == ("zune-core", "0.4.12"):
                repo = "etemesi254/zune-image"
            vcs = source / ".cargo_vcs_info.json"
            revision = json.loads(vcs.read_text())["git"]["sha1"] if vcs.is_file() else ""
            if not re.fullmatch(r"[0-9a-f]{40}", revision) or repo not in UPSTREAM_NOTICES:
                raise ValueError(f"No audited upstream license source for {label}")
            for path in UPSTREAM_NOTICES[repo]:
                url = f"https://raw.githubusercontent.com/{repo}/{revision}/{path}"
                content = download(url)
                if content:
                    output = destination / path
                    output.parent.mkdir(parents=True, exist_ok=True)
                    output.write_bytes(content)
                    urls.append(url)
            if not urls:
                raise ValueError(f"Pinned upstream license files could not be found for {label}")

    attribution = [label, f"Declared license: {package['license']}"]
    if package["authors"]:
        attribution.append(f"Published authors: {', '.join(package['authors'])}")
    if package["repository"]:
        attribution.append(f"Repository: {package['repository']}")
    if urls:
        attribution.extend(["", "Upstream reference:", *urls])
    if (name, version) in MANIFEST_ONLY:
        attribution.append("The pinned revision has no separate license text. See the declared SPDX terms bundled here.")
    (destination / "FETCH-NOTICE-SOURCES.txt").write_text("\n".join(attribution) + "\n")
    return f"- [{label}](crates/{label}/FETCH-NOTICE-SOURCES.txt) — {package['license']}"


def collect(directory, target):
    packages = release_dependencies(target)
    license_ids = set()
    for package in packages:
        expression = package["license"] or ""
        license_ids.update(token for token in re.split(r"[\s()/]+", expression)
                           if token and token not in {"AND", "OR", "WITH"})
    (directory / "spdx").mkdir()
    with ThreadPoolExecutor(max_workers=12) as executor:
        index = list(executor.map(lambda package: copy_dependency(package, directory), packages))
        urls = {license_id: f"{SPDX}/text/{license_id}.txt" for license_id in sorted(license_ids)}
        for (license_id, url), content in zip(urls.items(), executor.map(download, urls.values())):
            if not content:
                url = f"{SPDX}/exceptions/{license_id}.txt"
                content = download(url)
            if not content:
                raise ValueError(f"SPDX text unavailable for {license_id}")
            (directory / "spdx" / f"{license_id}.txt").write_bytes(content)

    (directory / "assets").mkdir()
    for name in ["Inter-OFL.txt", "RobotoMono-OFL.txt"]:
        shutil.copyfile(REPO / "src/ui/assets/fonts" / name, directory / "assets" / name)
    shutil.copyfile(REPO / "src/ui/assets/info.md", directory / "assets" / "asset-provenance.md")
    for name in ["LICENSE", "THIRD_PARTY_NOTICES.md", "LICENSE-CODEX-APACHE-2.0", "NOTICE-CODEX"]:
        shutil.copyfile(REPO / name, directory / name)
    sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], cwd=REPO, text=True).strip())
    rust_docs = sysroot / "share/doc/rust"
    (directory / "rust").mkdir()
    shutil.copyfile(rust_docs / "COPYRIGHT-library.html", directory / "rust/COPYRIGHT-library.html")
    shutil.copytree(rust_docs / "licenses", directory / "rust/licenses")

    (directory / "INDEX.md").write_text("\n".join([
        "# Fetch release notices",
        "",
        f"Cargo.lock release dependencies for {target} ({len(packages)} Rust crates).",
        "This inventory includes build dependencies and excludes development-only dependencies.",
        "Package-provided texts are in the crate folders. Standard terms are in `spdx/`.",
        "Embedded fonts and asset provenance are in `assets/`. Rust standard-library notices are in `rust/`.",
        "The separately set up session readers and downloaded runtimes are outside this app bundle.",
        "",
        *index,
        "",
    ]))
    return len(packages)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="Fresh notice directory to create")
    parser.add_argument("--target", default="aarch64-apple-darwin", choices=["aarch64-apple-darwin", "x86_64-apple-darwin"])
    args = parser.parse_args()
    output = args.output.resolve()
    if output.exists():
        parser.error(f"Output directory already exists: {output}")
    output.parent.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix=".fetch-notices-", dir=output.parent) as temporary:
        stage = Path(temporary) / "notices"
        stage.mkdir()
        count = collect(stage, args.target)
        stage.rename(output)
    print(f"Collected notices for {count} Rust release dependencies and embedded assets in {output}")


if __name__ == "__main__":
    main()
