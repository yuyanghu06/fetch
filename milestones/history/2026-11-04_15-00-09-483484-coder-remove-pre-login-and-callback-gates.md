# Remove pre login and callback gates

- Local timestamp / acting role / milestone: 2026-11-04T15:00:09.483484-05:00 / coder / wrapper-authentication.
- Requested scope: Implement accepted removal in two exclusively owned source areas.
- Starting repository state: Existing source/logging/console and documentation changes preserved; active wrapper-authentication scope authorized.
- Work performed / changed files: Inspected sdk.rs prepare: direct SDK metadata resolution/storage/session. Inspected runtime.rs callback parser: ignores unrelated fields, separately rejects duplicate state/code/error/iss. Preserved existing logs and unrelated dirty source. Orchestrator serialized this history from report and inspected evidence.
- Decisions: Official MCP library owns remote discovery/sign-in compatibility. Existing Fetch request-time networking, consent, instance/storage identity, state/deadline and no-silent-deletion contracts remain.
- Checks and evidence: Pinned rmcp 3.8.0 auth.rs optional issuer handling/legacy fallback; actual Fetch source inspection. No live-provider tests, app launches, credential/config changes or release work.
- Resulting state / acceptance: Source written in src/remote_oauth/sdk.rs and src/api/runtime.rs; no coder tests. Independent tests/review pending.
- Remaining work and handoff: Independent fake-HTTP/Keychain tests, appropriate regressions, locked native build and final evaluator verdict. No real Vercel account success claimed.
