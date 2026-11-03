# Retain deletion guards and fenced process references

- Local timestamp / acting role / milestone: 2026-11-03_15-32-49 America/New_York / Backend Coder and independent Tester, inspected and serialized by Orchestrator / delete-server.
- Requested scope: Correct cancellation lifetime and Quit behavior before delivery.
- Starting repository state: Initial deletion source; Tester requested owned cleanup lifetime.
- Work performed / changed files: Public Manager deletion uses Arc-owned Tokio worker holding complete guarded operation through blocking cleanup and finish. Quit enumerates raw registry and uses bounded retained-close for fenced contexts, preserving ordinary close behavior elsewhere. Tester independently rechecked both corrections; no files edited by Tester.
- Decisions: Caller loss detaches its waiter, not cleanup. Failed fenced closure retains connection/failed status and never resets capacity or claims stopped. No extra force/operation API added.
- Checks and evidence: Actual source inspected; final Tester ACCEPT with no further material findings. No tests/builds/checks/apps/providers/MCP/personal-state operations, native key changes or actual deletion performed.
- Resulting state / acceptance: Source findings resolved, deletion source accepted. Runtime and compilation remain unverified for this change.
- Remaining work and handoff: Final Evaluator, scoped commit/push; then shared authentication work per user ordering.
