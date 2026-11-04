# Preserved history must not hide a new error

- Local timestamp / acting role / milestone: 2026-11-04T14:52:54.031826-05:00 / evaluator (recorded by Orchestrator) / continuous-server-console
- Requested scope: Permanent expanded console and current-run startup-to-auth history.
- Starting repository state: Four source files implemented; independent checks and review active.
- Work performed / changed files: Independent review found that old startup_lines/startup_exit still count as current service output after begin_authentication. A real API/Keychain request failure before new capture is hidden by old history. Source owner is adding a small request-output baseline; Tester adds old-GET/new-output regression.
- Decisions: Preserve exact service history and visible genuine new Fetch failures. Do not change authentication, credentials, or issuer policy while repairing log display.
- Checks and evidence: Actual specialist results inspected through reports/current source. No real provider account or native app executed.
- Resulting state / acceptance: REQUEST CHANGES for this concrete continuity regression before final acceptance; no broader auth feature selected.
- Remaining work and handoff: Complete initial startup fixture, source correction, focused retests/build and final independent review.
