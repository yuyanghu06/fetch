# Investigate recurring large click selection

- Local timestamp / acting role / milestone: 2026-11-04T17:37:47.573720-05:00 / Orchestrator / terminal-pointer-coordinates.
- Requested scope: normal clicking sometimes causes large gray selection in embedded Terminal.
- Starting repository state: prior release-outside/pixel-wheel fix passed10focused checks; those did not exercise input through an actual scrolled parent. Existing dirty work preserved.
- Work performed / changed files: User reports large selection after clicks despite previous release/scroll fix. Inspected actual source and new screenshot; do not retain screenshot account/login/device-code contents. Assigned exclusive Planner/Coder/Tester. Found different coordinate sources for press versus move under outer page scrolling. Earlier release/trackpad fixes remain and are not evidence for this missing case.
- Decisions: no cursor-style/selection-policy change, drag threshold or broad rewrite. Reuse existing translated pointer/selection implementation. Coder holds source for independent reproduction.
- Checks and evidence: source inspection only; realistic reproduction/tests/build pending. No actual user login/settings/credentials/processes touched.
- Resulting state / acceptance: bounded plan/source cause confirmed, not yet implementation acceptance.
- Remaining work and handoff: original failure → two-line fix → focused UI/render tests/native build → independent Evaluator; root serializes journal.
