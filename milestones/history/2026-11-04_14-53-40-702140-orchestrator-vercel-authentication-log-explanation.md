# Explain the Vercel authentication log

- Local timestamp / acting role / milestone: 2026-11-04T14:53:40.702140-05:00 / Orchestrator / explanation only.
- Requested scope: Explain the user-provided Vercel authentication messages; no fix requested.
- Starting repository state: Existing source/documentation edits preserved; current service-output logging work remains separate.
- Work performed / changed files: Read src/remote_oauth/sdk.rs and searched related transport diagnostics; consulted official Vercel documentation search results; updated milestone snapshot/to-do and appended this entry.
- Decisions: No implementation or authentication-policy change selected. The log identifies Fetch's explicit metadata requirement as the observed stopping point, before client registration; earlier generic registration hypotheses do not explain this specific stop.
- Checks and evidence: sdk.rs prepare requires authorization_response_iss_parameter_supported to be true and returns NoAuthorizationSupport otherwise. It checks this before attaching saved-login storage or begin/client registration. First 401 says no authorization provided; two 200 responses show discovery requests succeeded, not sign-in. Final initialize 401 likewise says no authorization. No credentials or live-provider requests used. Official references consulted: https://vercel.com/docs/agent-resources/vercel-mcp and https://vercel.com/changelog/vercel-mcp-now-supports-client-id-metadata-documents .
- Resulting state / acceptance: Explanation grounded in the supplied log and current Fetch source. Missing or false advertised support is established; actual support is not established by this log. No bad-password or expired-token conclusion follows.
- Remaining work and handoff: Review Fetch's issuer-validation compatibility with Vercel before choosing a change. No code, tests, builds, sign-ins or personal-state changes. Existing work preserved; checks not independent.
