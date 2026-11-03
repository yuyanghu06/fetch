# Implement safe idle-server deletion

- Local timestamp / acting role / milestone: November 3, 2026, 15:27:24 America/New_York / Backend Coder, inspected and serialized by Orchestrator / delete-server.
- Requested scope: Shared delete lifecycle, contained file/key cleanup, truthful busy/failure behavior.
- Starting repository state: Adapter frozen; accepted plan and unrelated concurrent documentation preserved.
- Work performed / changed files: Added manager/deletion.rs; changed module wiring, installer, runner/scheduler/transport, storage, credentials and native bridge. Root read actual source and refreshed affected guides.
- Decisions: Keep deletion in an Arc-owned worker through cleanup after request loss; idle-only, stop-before-files, no-force, config-last retry, exact package/key targeting, application keys excluded. No actual servers/files/keys deleted by agents.
- Checks and evidence: Source checks cover package ID/sibling ownership, descriptor-relative NOFOLLOW recursion, exact SecItemDelete identity, retired credentials/runner aliases, bounded retained-child close. Cleanup limits128 depth/4096 entries per-directory fail honestly. No builds/tests/checks/runtime/apps/provider/MCP/personal-state actions.
- Resulting state / acceptance: Source implemented. Initial cancellation guard issue fixed via owned task; shutdown fenced-close details being finalized for source acceptance.
- Remaining work and handoff: Tester/Evaluator final verdict, scoped delivery; then queued authentication work after fresh instructions/source read.
