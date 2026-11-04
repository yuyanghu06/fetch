# Reproduce click selection through real scrolled page

- Local timestamp / acting role / milestone: 2026-11-04T17:39:37.056474-05:00 / Independent Tester and Orchestrator / terminal-pointer-coordinates.
- Requested scope: recurring large selection on normal terminal clicks after outer page scrolling.
- Starting repository state: primary-source plan confirmed; Coder held edits until real-parent regression reproduced.
- Work performed / changed files: One actual Iced Scrollable with spacer and500px page offset forwards real input to TerminalView. A1px visible mouse move after press produces SelectUpdate y=-474 instead of26. Original focused case fails exactly at this mismatch; log /private/tmp/fetch-terminal-scrolled-pointer-before.log inspected by root. Fixture contains generic text only, no screenshot login/account/code contents.
- Decisions: reuse Iced-provided cursor; retain selection semantics/cursor style and prior release/wheel fixes. No new parser/API/process/authentication/PATH behavior.
- Checks and evidence: original regression failure and inspected source/whitespace as above. Corrected-source11focused terminalUI checks/native build/review next; no pass claimed yet.
- Resulting state / acceptance: concrete cause reproduced and two-line fix ready for independent retest.
- Remaining work and handoff: Tester/Evaluator own final verification; root serializes journal. No native app, provider, shell or credentials used.
