# omit missing startup bindings

- Local timestamp / acting role / milestone: 2026-11-04T11:23:56.027418-05:00 / coder / startup-pass-through.
- Requested scope: User: just start/report outcome, remove that missing-input check for all startups.
- Starting repository state: Prior authentication/console code and all unrelated dirty work preserved.
- Work performed / changed files: Shared bindings.rs now omits missing assigned Environment and Header values, passes saved present values unchanged. Removed SERVER_INPUT_REQUIRED dispatch/recognition from local_auth.rs/manager mod. Normal/app/postinstall/restart/auth/helper transport share this path. No provider code, imports, personal state, runtime or Cargo runs by Coder.
- Decisions: Treat configured references as possible inputs, not declared prerequisites. Actual server decides and Fetch reports existing startup outcome; no raw-output feed or new route selected.
- Checks and evidence: Root inspected shared resolver/callers and confirms no SERVER_INPUT_REQUIRED/missing_inputs production references remain. Existing relevant source researched already, no new package/runtime/credentials.
- Resulting state / acceptance: Source patch ready; independent tests/build/final review pending.
- Remaining work and handoff: Verify actual confined child fallback/failure and loopback HTTP acceptance/rejection without missing-value preflight; update current contracts/results.
