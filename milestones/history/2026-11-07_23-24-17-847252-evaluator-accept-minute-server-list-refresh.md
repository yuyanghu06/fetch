# Accept one-minute app refresh

- Local timestamp / acting role / milestone: 2026-11-07T23:24:17.847252-05:00 / independent Evaluator, recorded by root / periodic-server-refresh
- Requested scope: Final source/test/build evidence review for automatic app server list GET every minute.
- Starting repository state: Coder implementation and two focused fixture tests complete; locked build passed. Prior source/Figma/docs changes preserved.
- Work performed / changed files: Read-only Evaluator inspected src/ui/mod.rs/auth_tests.rs diff and test/build logs. Root records acceptance and final project status.
- Decisions: Open app UI behavior only; existing client/session lifecycle, pending-read ordering, manual Refresh and1sactive-work polling stay. No background Runner or management permission change.
- Checks and evidence: ACCEPT.60second subscription dispatches AutomaticRefresh through existing GET/reply path; pending/missing client guards correct.2focused tests and locked build pass. Source-checked exact interval/path; no real60second timing, HTTP/native launch/account/MCP/Keychain test.
- Resulting state / acceptance: Selected source change complete; source whitespace checks clean.
- Remaining work and handoff: Running app was not relaunched; later deliberate signed development launch/native observation remains separate. Parameter JSON-as-form suggestion is not yet applied to Figma or code.
