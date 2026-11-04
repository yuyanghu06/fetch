# Preserve new request errors alongside old service history

- Local timestamp / acting role / milestone: 2026-11-04T14:55:24.348500-05:00 / Coder (recorded by Orchestrator) / continuous-server-console
- Requested scope: Resolve the independent review finding introduced by retained history.
- Starting repository state: Console/history source implemented; old output could hide a real error from the next request.
- Work performed / changed files: src/ui/authentication.rs records the prior capture ID, retained line count and observed exit when starting a request. Current-output checks exclude unchanged old snapshots and carried old exit lines. Old backend summaries no longer overwrite the new request error before new service output arrives. Existing history and state actions remain.
- Decisions: This changes display evidence only; no authentication, credential, permission or issuer behavior.
- Checks and evidence: Orchestrator inspected the written OutputBaseline/current-output logic. Coder reports whitespace pass; Tester has strengthened the exact regression and is running final library/integration checks. Actual unpolled app startup through auth fixture passed separately.
- Resulting state / acceptance: Material source correction complete; final retest/build and evaluator acceptance pending.
- Remaining work and handoff: Finish focused final evidence and journal. Explicit failed-Restart admission observation remains a separate unchanged limitation.
