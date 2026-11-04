# Shared router fixtures

- Local timestamp / acting role / milestone: 2026-11-03T16:54:04.913350-05:00 / tester / shared-mcp-authentication.
- Requested scope: Independently verify routing, shared OAuth requests, ownership/proof, errors and same-package account isolation.
- Starting repository state: Coder source complete; Google explicitly pending. No known auth test failure.
- Work performed / changed files: Added 11 src/manager/router_tests.rs fixtures. Narrowly corrected tests/local_mvp_acceptance.rs persistent-alias setup after initial deterministic failure, configuring stopped disposable routes before alias lookup. Production runner/source unchanged.
- Decisions: Preserve existing configured-target alias protection rather than changing runtime behavior. Separate server IDs retain distinct credentials/outcomes. Google remains pending with no helper execution.
- Checks and evidence: DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked --lib manager:: =>53 passed. Existing backend16, development_https7, environment_paths4 and import_compatibility7 passed. Initial local_mvp_acceptance10/11 failed at stale alias assertion; targeted reproduction confirmed same guard in HEAD. After fixture-only correction, affected target11/11 and previously unrun raw_json1/1 passed. Total99 unique passing checks. No actual provider/helper/MCP-client/native/Keychain operation.
- Resulting state / acceptance: Independent Tester ACCEPT for scoped router/refactor; no material authentication findings.
- Remaining work and handoff: Native compilation and final evaluation recorded separately; real provider/native/client acceptance remains pending.
