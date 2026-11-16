# Confined reader and library feasibility limits

- Local timestamp / acting role / milestone: 2026-11-13T15:05:48.536107-05:00 / Tester test_codex_reader_snapshot, serialized by Orchestrator / official-session-readers.
- Requested scope: safe official reader proof with disposable synthetic sessions; no user config/auth/data.
- Starting repository state: installed Codex0.167.2, no reader dependency change selected.
- Work performed / changed files: temporary fixture/caches only, all removed after completed probes; no app or personal settings changed.
- Decisions: never bypass confinement with normal HOME. Snapshot service proof failed before initialization.
- Checks and evidence: denied outside synthetic reads/writes/network; confined version passes. app-server configLoad EPERM persisted after private paths/file-only auth and41literal ancestor metadata allowances; no thread/read reply. Source/copy hashes unchanged. First pure Rust helper used only installed1.94.1 and pinned ce21072c90d32c40af7a55b60b2fb35c84a43a6a; resolver failed because codex-otel needs tokio-tungstenite proxy feature from upstream workspace patches, before Rust compiler.93.96s/~0.88GB task cache cleaned.
- Resulting state / acceptance: no runtime full-reader support proven, no Rust1.95 conclusion, no appsource changes.
- Remaining work and handoff: later patched build/source-only storage proof ongoing; user API/dependency decision remains pending.
