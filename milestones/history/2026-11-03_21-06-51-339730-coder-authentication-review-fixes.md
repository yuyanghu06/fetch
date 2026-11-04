# Authentication recovery and current documentation fixes

- Local timestamp / acting role / milestone: 2026-11-03T21:06:51.339730-05:00 / coder / native-server-storage
- Requested scope: fix independent review findings and update current records while preserving earlier decisions.
- Starting repository state: task changes uncommitted on main after pull515c7b1; initial141 checks and native build pass; evaluator requested changes.
- Work performed / changed files: Manager explicit login recovery and post-drain actual authenticate-route renewal; Runner retires stale rejection evidence with header-revision coordination; transport captures actual request headers before spawn and no longer treats bare403 as expired login; GET no longer persists derived rejection. Test-only Mac confirmation seam supports independently authored actual manual PATCH tests. Current auth/storage/open-question documents are concise, earlier records archived in full; current context and worklist updated.
- Decisions: no per-server refresh/compatibility code; no tool replay, old-state cleanup or new public metadata authoring contract.
- Checks and evidence: backend targeted8/8 passes; independent evaluator source rereview finds issues resolved. Final integrated test/build pending. See ../native-server-storage/evidence.md.
- Resulting state / acceptance: source frozen for final Tester run; scoped final ACCEPT pending.
- Remaining work and handoff: finish final integrated tests/native binary build, synchronize evidence/status, obtain ACCEPT and integrate. Native metadata authoring and real OS/provider/client acceptance remain limits.
