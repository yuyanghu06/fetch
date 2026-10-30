# Provide safe host construction for independent Desktop update tests

- Local timestamp / acting role / milestone: 2026-10-30T16:20:04.162342-04:00 / Orchestrator / parallel-authentication.
- Requested scope: User approved the authentication plan and requested remaining MVP blockers after implementation.
- Starting repository state: main fe916e7 plus preserved October30 plan documents and concurrent exclusively owned source/test work.
- Work performed / changed files: Added cfg(test)-only ApplicationHost::fixture in src/application.rs. It accepts the test-owned CredentialStore, creates only disposable encrypted OAuth state, an in-memory certificate and a fixture lock, and supplies a host without a listener, gateway or started services. Tester can exercise actual Desktop messages without ApplicationHost::prepare or a native window.
- Decisions: Preserve POST authentication's D25 check and D31 checked saves, per-server coordination, native Settings authority and existing fixed provider endpoints. No deletion-only success or new provider support selected.
- Checks and evidence: Inspected constructor calls and generated diff. Compilation/tests for this new seam are pending the independent Tester. No real app launch, OS-store access or network service startup.
- Resulting state / acceptance: Fixture seam implemented; independent compilation and update-path tests remain pending.
- Remaining work and handoff: Tester runs current saved-key, batch, parallel and stale-result fixtures; Evaluator independently reviews final code/evidence. Orchestrator updates the remaining-MVP/testing handoff. Real protected native/provider/client acceptance and no-release boundaries remain.
