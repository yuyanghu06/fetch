# Implement shared input resolution and trusted verification descriptions

- Local timestamp / acting role / milestone: 2026-10-30T17:13:48.867235-04:00 / Coder / generic-authentication.
- Requested scope: Implement D82 shared imported inputs and reusable trusted-description verification while retaining D25/D31 and existing remote OAuth.
- Starting repository state: main36d0f47 plus preserved research/plan/docs and exclusively owned concurrent source/tests.
- Work performed / changed files: Added bindings.rs, verification.rs and verification_profiles.json; integrated Manager and removed obsolete Railway verifier helpers. Old credential-name/request execution switches are removed. Existing four service request facts are data. Generic checks support individual/joint inputs, headers/JSON, response predicates and current encrypted readback. Review prompted complete HTTP-request preflight and explicit overlapping-JSON destination rejection.
- Decisions: No public verification route/URL schema or saved-record migration; no readiness substitute or automatic unknown-provider proof. Every submitted auth value must participate in its current saved proof.
- Checks and evidence: Process-scoped CLT cargo check --locked --lib --message-format short exited0 in8.23s with the existing four Security API deprecations and unread MCP error field. No native/provider/MCP/personal state actions.
- Resulting state / acceptance: Source compiles; no final acceptance. Independent regression fixtures and evaluator findings remain part of this pass.
- Remaining work and handoff: Fix lifecycle findings, add production-path regression fixtures, rerun affected/full fixtures and native binary build, obtain final independent acceptance, refresh docs and deliver. Parent local working MVP remains incomplete.
