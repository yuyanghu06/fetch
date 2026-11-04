# Remove full metadata reuse gate

- Local timestamp / acting role / milestone: 2026-11-04T15:00:09.483485-05:00 / coder / wrapper-authentication.
- Requested scope: Follow-up remove overbroad saved metadata equality.
- Starting repository state: Existing source/logging/console and documentation changes preserved; active wrapper-authentication scope authorized.
- Work performed / changed files: Inspected Store::load: endpoint match remains; full metadata JSON equality removed. Saved issuer/client identity validation and stored document shape retained; endpoint mismatch error corrected. Orchestrator serialized this history from report and inspected evidence.
- Decisions: Official MCP library owns remote discovery/sign-in compatibility. Existing Fetch request-time networking, consent, instance/storage identity, state/deadline and no-silent-deletion contracts remain.
- Checks and evidence: Pinned rmcp 3.8.0 auth.rs optional issuer handling/legacy fallback; actual Fetch source inspection. No live-provider tests, app launches, credential/config changes or release work.
- Resulting state / acceptance: Only src/remote_oauth/sdk.rs changed in follow-up. No migration/deletion or tests. Tester owns regression files.
- Remaining work and handoff: Independent fake-HTTP/Keychain tests, appropriate regressions, locked native build and final evaluator verdict. No real Vercel account success claimed.
