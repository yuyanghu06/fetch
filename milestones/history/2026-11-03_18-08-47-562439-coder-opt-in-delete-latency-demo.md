# Prepare an opt-in real MCP deletion diagnostic

- Local timestamp / acting role / milestone: November 3, 2026 / Coder; inspected and serialized by Orchestrator / delete-server.
- Requested scope: Install and demo deletion of one real disposable MCP and explain latency.
- Starting repository state: Existing dirty application/UI/docs files and concurrent command-install manifest/source changes preserved. Shared role files absent; session team instructions followed.
- Work performed / changed files: Added tests/delete_latency_demo.rs, ignored by default and gated by an explicit disposable /private/tmp root marker. It installs official Everything2026.8.31 through Manager, verifies catalog/echo, samples package cleanup, times deletion and checks preservation. Key provider is disposable and never uses Keychain. Updated tests/info.md and this journal.
- Decisions: Production deletion behavior remains unchanged. Backend diagnostic is distinct from protected native UI/Keychain acceptance. Existing public pinned Node/npm runtime can be copied as non-secret executable data into the disposable root.
- Checks and evidence: Locked no-run compile initially encountered a concurrently changed manifest/lock, then succeeded after that owner updated the lock (23.75 seconds; final diagnostic-event compile1.72 seconds). Orchestrator inspected harness source. No package/MCP process was executed by Coder.
- Resulting state / acceptance: Compiled opt-in diagnostic ready for independent OS-confinement validation and actual execution. No runtime acceptance yet.
- Remaining work and handoff: Tester must verify allowed/denied canaries and execute only inside the task sandbox; record measured outcomes, clean owned packages/processes and independently evaluate evidence.
