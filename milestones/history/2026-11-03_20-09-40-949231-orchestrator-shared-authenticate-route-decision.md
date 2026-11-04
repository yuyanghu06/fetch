# Both callers use the authenticate route

- Local timestamp / acting role / milestone: 2026-11-03T20:09:40.949231-05:00 / Orchestrator / native-server-storage.
- Requested scope: one shared implementation; user explicitly says button and background both call Fetch authenticate route.
- Starting repository state: paused incomplete refactor, credentials route retention scope still under discussion.
- Work performed / changed files: update native-storage plan, architecture decisions/API contract, context/notes and milestone tracking; no application edits.
- Decisions: existing authenticate POST is the shared entry point; scheduler timing only; no new refresh route or server-specific refresh engine.
- Checks and evidence: prior inspected route/manager source establishes existing login/renewal responsibilities. Documentation only; no builds/tests/runtime/credentials accessed.
- Resulting state / acceptance: routing requirement recorded; browser-mode/default and internal request authorization details remain for planning; source pause intact.
- Remaining work and handoff: present those precise route details in plain language alongside unresolved credential/folder/native setup contracts before source work.
