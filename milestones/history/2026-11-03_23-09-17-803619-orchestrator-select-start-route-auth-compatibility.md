# Select start route for interactive login

- Local timestamp / acting role / milestone: 2026-11-03T23:09:17.803619-05:00 / Orchestrator / standard-authenticate.
- Requested scope: User steering: call start API and deprecate but retain auth route.
- Starting repository state: Runner/Manager implementation and root UI changes in progress; other dirty changes preserved.
- Work performed / changed files: Sent steering to Coders/Tester; root switched UI request paths to /restart, added compatible auth deprecation headers and successor Link, restart response/polling handling and MCP interactive schema/forwarding. Updated current contracts and notes.
- Decisions: Existing /restart owns start/connect explicit interactive option; no new /start route, no auth removal/Sunset. Missing native URL elicitation contract remains unclaimed.
- Checks and evidence: Inspected existing route registration/start implementation; RFC9745 structured Date deprecation syntax checked at https://www.rfc-editor.org/rfc/rfc9745.html . Whitespace check passed so far, no build/runtime result yet.
- Resulting state / acceptance: User route choice reflected in source-in-progress/contracts. Full compilation/tests/review pending.
- Remaining work and handoff: Manager steering integration, independent isolated checks/evaluation, fix/retest and documentation.
