# Explain remaining Vercel and Railway login failures

- Local timestamp / acting role / milestone: 2026-11-04T14:14:44.908353-05:00 / Orchestrator / private-program-paths follow-up
- Requested scope: Read-only diagnosis and concise explanation; user explicitly prohibits code.
- Starting repository state: Scoped helper lookup fix accepted; new user errors after retry.
- Work performed / changed files: Read current SDK/error mapping/shared local startup/runner source and official Vercel/Railway docs. Documentation synchronization only.
- Decisions: Do not infer an exact live rejection from generic grouped errors; do not implement or select a new login design.
- Checks and evidence: Vercel publishes client metadata URL/approved-client requirement; Fetch configures client name without that URL/preset ID. Error mapping groups several failures. Railway reuses CLI login; Fetch supplies distinct HOME and starts configured command. Original protocol error text is discarded. No code, provider requests, tokens or client configs changed.
- Resulting state / acceptance: Explanatory evidence recorded; exact live failure causes remain unconfirmed.
- Remaining work and handoff: Inspect exact underlying failure stages/messages before selecting fixes; no implementation authorized by explanation request.
