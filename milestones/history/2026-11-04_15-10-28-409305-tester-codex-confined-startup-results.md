# Codex confined startup results

- Local timestamp / acting role / milestone: 2026-11-04T15:10:28.409305-05:00 / tester / codex-startup-environment
- Requested scope: Verify actual shared environment in confined ordinary startup and recorded setup plus relevant regressions.
- Starting repository state: Focused source accepted; independent library compilation still awaiting concurrent terminal test file/initializer.
- Work performed / changed files: Independent Tester ran the strengthened fixture suites and an isolated temporary rustc harness for the new pure helper tests. Harness was deleted. Root inspected actual helper and confined fixture assertions/canaries.
- Decisions: Keep tests independent of host mutation and provider accounts; no unrelated source repair.
- Checks and evidence: cargo test --locked --test program_paths --test environment_paths --test service_startup_logs --test standard_authentication --test isolated_native_mcp:24/24 pass (2+4+6+10+2). Both real confined ordinary/setup child processes received expected arguments, cwd, private HOME/XDG, computed PATH, LANG and helper lookup. New pure helper tests8/8 pass via temporary rustc wrapper; this alone is not native crate acceptance.
- Resulting state / acceptance:32 exercised checks pass so far. Native build and focused library regressions still pending; no provider authentication or application launch.
- Remaining work and handoff: Finish native build, retry library tests after concurrent owner files settle and obtain final evaluation.
