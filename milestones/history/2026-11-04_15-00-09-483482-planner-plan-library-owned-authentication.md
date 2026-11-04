# Plan library owned authentication

- Local timestamp / acting role / milestone: 2026-11-04T15:00:09.483482-05:00 / planner / wrapper-authentication.
- Requested scope: Read-only bounded plan for removing custom issuer and neighboring pre-login gates.
- Starting repository state: Existing source/logging/console and documentation changes preserved; active wrapper-authentication scope authorized.
- Work performed / changed files: Inspected pinned rmcp metadata resolution and callback validation; identified four redundant prepare gates and retained Manager callback delegation, actual-request network checks, storage and consent. Orchestrator serialized this history from report and inspected evidence.
- Decisions: Official MCP library owns remote discovery/sign-in compatibility. Existing Fetch request-time networking, consent, instance/storage identity, state/deadline and no-silent-deletion contracts remain.
- Checks and evidence: Pinned rmcp 3.8.0 auth.rs optional issuer handling/legacy fallback; actual Fetch source inspection. No live-provider tests, app launches, credential/config changes or release work.
- Resulting state / acceptance: No files changed by Planner. Plan accepted for removal; unit regression/build/evaluation remain.
- Remaining work and handoff: Independent fake-HTTP/Keychain tests, appropriate regressions, locked native build and final evaluator verdict. No real Vercel account success claimed.
