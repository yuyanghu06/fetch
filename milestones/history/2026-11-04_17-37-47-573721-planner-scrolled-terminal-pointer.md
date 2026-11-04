# Plan consistent scrolled pointer coordinates

- Local timestamp / acting role / milestone: 2026-11-04T17:37:47.573720-05:00 / Read-only Planner / terminal-pointer-coordinates.
- Requested scope: normal clicking sometimes causes large gray selection in embedded Terminal.
- Starting repository state: prior release-outside/pixel-wheel fix passed10focused checks; those did not exercise input through an actual scrolled parent. Existing dirty work preserved.
- Work performed / changed files: Primary installed Iced scrollable.rs790–825 translates child cursor_position +scroll offset while forwarding event unchanged. Terminal press uses translated passed position but CursorMoved branch reads raw event position. Proposed match CursorMoved{..} and use existing cursor_position, matching press. One actual outer Scrollable click/tiny-move and deliberate-drag regression before source change.
- Decisions: no cursor-style/selection-policy change, drag threshold or broad rewrite. Reuse existing translated pointer/selection implementation. Coder holds source for independent reproduction.
- Checks and evidence: source inspection only; realistic reproduction/tests/build pending. No actual user login/settings/credentials/processes touched.
- Resulting state / acceptance: bounded plan/source cause confirmed, not yet implementation acceptance.
- Remaining work and handoff: original failure → two-line fix → focused UI/render tests/native build → independent Evaluator; root serializes journal.
