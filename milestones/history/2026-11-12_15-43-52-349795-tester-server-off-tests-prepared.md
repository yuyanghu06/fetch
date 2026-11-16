# Prepare focused server off tests

- Local timestamp / acting role / milestone: 2026-11-12T15:43:52.349795-05:00 / Tester, parent-serialized journal / server-on-off.
- Requested scope: prepare a few isolated real-feature checks while Coder implements production files; do not execute yet.
- Starting repository state: approved contract/plan; Coder holds all production ownership.
- Work performed / changed files: new tests/server_power.rs (3 confined HTTP/MCP scenarios), new src/manager/power_tests.rs (2 private quiet-Off/conflict checks), one actual toggle case in src/ui/neumorphic_tests.rs, and agreed power-field initializers in src/ui/auth_tests.rs. No production writes.
- Decisions: use existing OS-confined synthetic child fixtures, in-memory grants and no-network remote stub. Parent inspected sandbox canaries, dedicated fixture homes, panic cleanup and shutdown paths.
- Checks and evidence: source review only; no test execution or compilation during preparation. No personal accounts, files or MCP/provider services used.
- Resulting state / acceptance: test preparation accepted; execution waits for Coder's compile checkpoint.
- Remaining work and handoff: test actual stop/on, persistence, busy preservation, management permission/schema and native pending/failure/state; then independent evaluation.
