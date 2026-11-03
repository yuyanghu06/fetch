# Bind authentication errors to their actual connection

- Local timestamp / acting role / milestone: 2026-11-03_14-55-20 America/New_York / Tester, inspected and serialized by Orchestrator / install-connect D83.
- Requested scope: Independently source-review callback, status, reuse, retained connections, error handling and UI.
- Starting repository state: Frozen four-file D83 source with other work preserved.
- Work performed / changed files: Read source/diff and lifecycle/transport; Tester owned no files, Orchestrator records finding.
- Decisions: REQUEST CHANGES for one narrow new race; other inspected D83 paths sound.
- Checks and evidence: invoke_inner/list_native acquire connection, await, then read generation. Reconnect can pair old connection/new generation and wrongly flag a new context from an old401. Require actual connection identity as well as generation. Callback/locks, preparation outcomes, no-auth classification, retained connection, timeout/Quit guards and UI otherwise pass source reasoning. No tests/builds/checks/apps/providers/MCP/personal-state operations.
- Resulting state / acceptance: Acceptance withheld until correction; source review adds no runtime evidence.
- Remaining work and handoff: Coder fixes latch, Tester rechecks and Evaluator reviews final source/docs. Preserve newly verified pending credentials; no lifecycle lock across RPC.
