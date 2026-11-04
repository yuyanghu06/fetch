# atomic login admission

- Local timestamp / acting role / milestone: 2026-11-03T23:47:25.206126-05:00 / coder / standard-authenticate.
- Requested scope: Resolve independent authentication review/test findings within selected startup/login change.
- Starting repository state: Source integrated with current dirty UI/research/docs preserved; first independent checks in progress.
- Work performed / changed files: Runner/scheduler follow-up adds an atomic idle guard rejecting active/queued/uncertain/restarting/deleting work without flushing it. Normal restart unchanged; no-process scheduler test written. Source whitespace passed; root binary build subsequently passed15.41s.
- Decisions: Preserve work safety/current-status truth, existing inputs/last-known schema and server-owned tokens. No broader provider features or personal-state operations.
- Checks and evidence: Actual specialist report inspected by root against code; first build PASS15.41s. Initial failing checks are retained here and not reported as final acceptance.
- Resulting state / acceptance: Fixes integrated; final retest/evaluation pending.
- Remaining work and handoff: Eight independent confined scenarios including >30second wait, lib/regression retest, final review and current-state documentation.
