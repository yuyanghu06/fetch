# Review per-server file and CLI proposals

- Local timestamp / acting role / milestone: 2026-10-27T10:20:17.391775-04:00 / Evaluator (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Evaluate and integrate per-server encrypted-file requirements, reviewed general auth approval, proposed directory placement and GitHub CLI feasibility.
- Starting repository state: D26/D27 documented; separate durable server-state placement and CLI implementation choices pending; implementation paused.
- Work performed / changed files: Read-only independent review of D26/D27 approval fidelity, D25 gate, CLI adapter/storage/refresh limitations, Apple-guided layout recommendation and current record consistency. No specialist writes.
- Decisions: No new directory, CLI/auth mechanism or storage exception approved; preserve D25 verified stored credential and D26 per-server encryption.
- Checks and evidence: ACCEPT, no material findings. 58 Markdown files, 12 JSON examples, 236 local links, 39 anchors and 34 linked history entries passed with fences/whitespace/required fields. No integration tests.
- Resulting state / acceptance: Documentation ACCEPT; placement and CLI mechanisms remain proposals.
- Remaining work and handoff: Obtain user choice on durable state placement and concrete GitHub CLI/tool/API/install/auth/renewal design; complete other contracts without starting implementation.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
