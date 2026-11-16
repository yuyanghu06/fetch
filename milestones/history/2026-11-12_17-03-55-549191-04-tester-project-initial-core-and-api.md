# Initial disposable project acceptance

- Local timestamp / acting role / milestone: 2026-11-12 17-03-55-549191 America/New_York / Independent Tester / project-connections.
- Requested scope: Test real feature use in two disposable projects with Shared and project servers.
- Starting repository state: Core and route fixtures were prepared separately; the future route fixture was temporarily a milestone draft until APIs existed.
- Work performed / changed files: Restored tests/project_scope.rs and injected registered projects/Workspace. Core tests cover canonical persistence/reuse/cwd/discovery/progress; combined native HTTP/MCP scenarios cover isolation, stale caller denial, active+queued move rejection and bounded session reads.
- Decisions: No executable MCP package, personal logs/accounts/settings or real credentials. Synthetic owned loopback peers only. Draft promoted to the real test target, then removed as superseded.
- Checks and evidence: Initial cargo test --locked --test project_registry --test project_scope:2passed0.13s +4passed0.34s, build27.19s. No fixture changes were needed in that run.
- Resulting state / acceptance: Initial six scenarios pass; metadata races found by independent review required later source correction and final retest.
- Remaining work and handoff: Final extended session replay and project-grant reopen/revoke results are in final verification.

