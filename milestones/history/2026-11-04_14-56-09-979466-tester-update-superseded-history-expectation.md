# Update the test that previously required clearing logs

- Local timestamp / acting role / milestone: 2026-11-04T14:56:09.979466-05:00 / Tester (recorded by Orchestrator) / continuous-server-console
- Requested scope: Verify the user-selected cumulative current-run service history.
- Starting repository state: Review correction implemented; final library and integration runs active.
- Work performed / changed files: 130 library checks, two confined native checks, two private-program checks and six startup-log checks passed. Standard authentication passed nine of ten; the remaining test asserted that a new attempt removes the previous FATAL line, which is superseded by the new retention requirement. Tester receives exclusive ownership of the focused assertion/comment in tests/standard_authentication.rs.
- Decisions: Replace only the obsolete clearing expectation with retained sanitized fatal/observed exit exactly once. Preserve checks for fresh capture IDs, empty current exit metadata and no secrets/URLs. No product authentication behavior change.
- Checks and evidence: Failure at standard_authentication.rs around line503; actual output correctly retained prior logs. Relevant target will be rerun, with final native build and whitespace check.
- Resulting state / acceptance: Remaining mismatch is a superseded test expectation; final verification still pending.
- Remaining work and handoff: Rerun the changed standard-auth target, build and obtain evaluator acceptance. Keep separate failed-Restart admission observation unchanged.
