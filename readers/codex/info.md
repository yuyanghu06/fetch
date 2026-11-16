# Official Codex session reader

This small, separate Rust executable reads conversations using Codex's own stored
history code. It never starts Codex's app server, model clients, MCP servers,
configuration loader or authentication manager.

`src/main.rs` accepts one JSON request on stdin and returns one JSON response on
stdout. `list` preserves each candidate's ID and original file path, excludes
`SessionSource::is_non_root_agent()`, and uses the official current-rollout lookup.
When `source_index` is supplied it reads the existing native index with SQLite's
read-only option and the exact official metadata query and decoder. It never
migrates or initializes that index. Without an index it uses Codex's own filesystem
fallback; that fallback cannot override a selection saved only in a native index.

`conversation` copies only the selected rollout and its native `history_base`
ancestors into the caller's fresh `work_dir`. Native filename and compressed-file
readers find ancestors; original session bytes are never rewritten. The native
legacy reader builds legacy turns. For paginated history, the native projector
rebuilds a fresh private history database, honors inherited ordinal cutoffs and
subagent history starts, and the native item reader follows every result page.
Manual user messages, assistant messages and named tool results are mapped from
native UI items; model-context records, hook prompts, reasoning and settings are
not interpreted as user input. Text and output are not truncated by this helper.

Run `python3 scripts/setup-codex-session-reader.py --root /absolute/readers-root`.
The default is `~/Library/Application Support/Fetch-Development/session-readers`.
The pinned source, private Cargo downloads, build output and installed executable
live inside `<root>/codex`. A failed build leaves the prior executable installed.
`provenance.json` records source and local wrapper hashes. Source roots must be
native `sessions` / `archived_sessions` directories, separate from `work_dir`.
The application additionally confines the process so it can only read those
original roots and an explicitly allowed metadata index, and write `work_dir`.

## Pin, license and narrow source exposure

- Upstream: <https://github.com/openai/codex>, Apache-2.0.
- Tag: `rust-v0.167.2`.
- Commit: `ce21072c90d32c40af7a55b60b2fb35c84a43a6a`.
- Source archive SHA-256: `11e3321cd1a158dcfd920f8d24b07a08e896b9eaa7f4800f3042227b56342170`.
- Upstream `LICENSE` and `NOTICE` stay with the downloaded source.
- The standalone manifest repeats upstream's exact websocket fork patches. All
  Codex crates come from the same pinned source directory.
- `materialize-entrypoint.rs` is inserted into the private source copy's
  `LocalThreadStore` implementation. It only calls the existing lineage resolver
  and existing per-segment projector. It also exposes the existing native name
  selectors. No reconstruction algorithm or naming algorithm is changed.
- The setup script adds `StateRuntime::fetch_get_thread_readonly`: it reuses the
  exact upstream `get_thread` query and decoder, substituting a read-only SQLite
  pool for the initialized writable runtime. No migration or cleanup runs there.
- Setup checks every other upstream file against the hash-verified source archive;
  it rejects unexpected modifications to either exposed file too.

`tests/fixtures.py` exercises actual executable requests on disposable fixtures:
legacy full messages and injected-context exclusion, paginated history beyond
one page, native fork ancestry/cutoff, subagent list exclusion, native read-only current-rollout/name selection, explicit
missing-ancestry errors, and unchanged
original source hashes. It never reads personal sessions or starts services.

If the native index selects a rollout that no longer exists, listing skips that thread. It never substitutes an older surviving rollout. Other read failures and allowed-root checks still fail explicitly; direct conversation/ancestry reads are unchanged.
