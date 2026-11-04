# Implement common authentication routing and web requests

- Local timestamp / acting role / milestone: 2026-11-03T16:49:58.499042-05:00 / Coder, inspected by Orchestrator / shared-mcp-authentication.
- Requested scope: Shared setup/authentication routing from imported command/arguments/environment; common Railway/Linear OAuth; Google pending by user choice; surface safe selected-flow errors.
- Starting repository state: Existing public Authenticate/GET status and encrypted saved formats; unrelated uncommitted documentation preserved.
- Work performed / changed files: Added src/manager/authentication.rs; refactored mod.rs, remote_auth.rs and railway.rs. Orchestrator inspected actual source/diff and refreshed manager guide and project journal. Independent Tester owns router_tests.rs; independent Evaluator reviewing source.
- Decisions: WebPlan centralizes registration bodies/requests, authorization URL and code/refresh token POSTs. Keep Railway native callback/scopes/rotation and Linear allowlist/resource compatibility. Generic discovery, issuer and DNS checks retained. Manager keeps ownership/revision/deadline, encrypted save/readback, service verification and per-server renewal. No new public contract, dependency, helper bridge or package-global account state. Unknown helpers are not guessed or executed; Google has precise pending guidance.
- Checks and evidence: Source inspection of all four implementation files only. Coder ran no tests/build/providers. Prior checks do not establish this source passes; independent tests/build/evaluation remain.
- Resulting state / acceptance: Implementation source integrated for independent verification. Google support is not claimed; no public route or saved-document migration.
- Remaining work and handoff: Run focused router/existing backend/raw fixtures and native build; resolve material findings; complete Evaluator decision and user walkthrough. Broader local-MVP/native/provider/client acceptance remains pending.
