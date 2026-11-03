# Bind late authentication failures to the active connection

- Local timestamp / acting role / milestone: 2026-11-03_14-57-40 America/New_York / Coder plus Orchestrator inspection / install-connect D83.
- Requested scope: Correct Tester's generation mismatch finding without holding locks over RPCs or changing unrelated work.
- Starting repository state: D83 source implemented; source acceptance withheld for one identity race.
- Work performed / changed files: Coder changed runner/mod.rs: published request failures check generation and active Arc identity under a brief connection read guard before atomic latching. Unpublished startup failure path retains lifecycle protection. Root refreshed guides and install-tool description, API/authentication docs and state coverage.
- Decisions: Keep header refresh behavior and explicit Connect rules; no arbitrary rejection resets. Preserve required-vs-runtime error distinction and pending verified credentials.
- Checks and evidence: Read actual diff and all three published failure call sites. No await between identity/generation check and latch. No tests/builds/checks/format/lint/apps/providers/MCP/personal-state actions; no tests added.
- Resulting state / acceptance: Source correction frozen and independently re-reviewed ACCEPT. Actual compilation/runtime remains unverified.
- Remaining work and handoff: Final Evaluator source/documentation review, scoped commit/push and delivery record; later authorized runtime/build/visual checks remain.
