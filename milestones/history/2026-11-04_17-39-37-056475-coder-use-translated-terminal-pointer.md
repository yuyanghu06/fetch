# Use adjusted pointer consistently

- Local timestamp / acting role / milestone: 2026-11-04T17:39:37.056474-05:00 / Coder and Orchestrator / terminal-pointer-coordinates.
- Requested scope: recurring large selection on normal terminal clicks after outer page scrolling.
- Starting repository state: primary-source plan confirmed; Coder held edits until real-parent regression reproduced.
- Work performed / changed files: Changed only two lines in vendor/iced_term/src/view.rs: CursorMoved{..} no longer reads raw event coordinates; handle_cursor_moved receives &cursor_position, matching press. Root inspected actual source and git diff --check passes. No function signatures or other source behavior changed; independent retest/build pending.
- Decisions: reuse Iced-provided cursor; retain selection semantics/cursor style and prior release/wheel fixes. No new parser/API/process/authentication/PATH behavior.
- Checks and evidence: original regression failure and inspected source/whitespace as above. Corrected-source11focused terminalUI checks/native build/review next; no pass claimed yet.
- Resulting state / acceptance: concrete cause reproduced and two-line fix ready for independent retest.
- Remaining work and handoff: Tester/Evaluator own final verification; root serializes journal. No native app, provider, shell or credentials used.
