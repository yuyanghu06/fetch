# Implement independent authentication request tracking in the UI

- Local timestamp / acting role / milestone: 2026-10-30T16:20:04.162341-04:00 / Coder / parallel-authentication.
- Requested scope: User approved the authentication plan and requested remaining MVP blockers after implementation.
- Starting repository state: main fe916e7 plus preserved October30 plan documents and concurrent exclusively owned source/test work.
- Work performed / changed files: src/ui/mod.rs, src/ui/authentication.rs and new src/ui/auth_requests.rs track explicit auth, inline saves and Settings saves per server without setting or clearing general busy. Unique request tokens and status-read revisions/sequences reject old responses. Failed manual saves cannot be promoted by older authenticated evidence.
- Decisions: Preserve POST authentication's D25 check and D31 checked saves, per-server coordination, native Settings authority and existing fixed provider endpoints. No deletion-only success or new provider support selected.
- Checks and evidence: cargo check --locked --bin fetch --message-format short passed in 4.75s; coder diff check passed. Orchestrator inspected source wiring. No native window, provider, MCP or Keychain execution.
- Resulting state / acceptance: UI source pass complete; independent update-path fixtures and real native interaction remain pending.
- Remaining work and handoff: Tester runs current saved-key, batch, parallel and stale-result fixtures; Evaluator independently reviews final code/evidence. Orchestrator updates the remaining-MVP/testing handoff. Real protected native/provider/client acceptance and no-release boundaries remain.
