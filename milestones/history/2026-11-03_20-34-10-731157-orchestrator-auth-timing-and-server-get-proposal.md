# Propose auth timing and server GET metadata

- Local timestamp / acting role / milestone: 2026-11-03T20:34:10.731157-05:00 / Orchestrator / native-server-storage.
- Requested scope: explain check scheduling/method/harness behavior; include auth status and expiry in existing server GET.
- Starting repository state: paused partial refactor; official library/common authenticate route/recorded commands/manual-key purpose selected.
- Work performed / changed files: primary-source inspection; plan/API/decision/notes and milestone documentation only.
- Decisions: requested visibility recorded; exact existing-state plus access_expires_at/last_verified_at and expiry/use/rejection timing remain proposed. GET stays read-only.
- Checks and evidence: official rmcp-v3.8.0 get_access_token/prepare_request/REFRESH_BUFFER_SECS source and Claude Code docs; no tests/builds/library or MCP execution.
- Resulting state / acceptance: complete bounded GET/timing proposal, known-vs-hidden expiry and protected-check evidence distinguished; no application acceptance.
- Remaining work and handoff: user reviews timing/metadata and existing pending background/manual-key/native-record contracts before coding.
