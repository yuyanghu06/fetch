# Plan automatic Console and combined error output

- Local timestamp / acting role / milestone: 2026-11-11T14:37:11.941376-05:00 / Planner (independent read-only agent) / console-on-expansion.
- Requested scope: open Console on expansion and show Fetch errors alongside server output.
- Starting repository state: existing dirty changes preserved; Console defaults Hidden; standalone auth/runtime errors below tabs; Flow log suppresses Fetch failures when service output exists.
- Work performed / changed files: read-only inspection/report; root wrote this journal and milestone status.
- Decisions: presentation-only assembly of existing log plus current errors; preserve auth history internals; direct Console selection on each expansion; Copy uses displayed lines; remove standalone error rows.
- Checks and evidence: source inspection of src/ui/mod.rs, authentication.rs, server_terminal.rs and fixture tests. No tests/build/launch in Planner pass.
- Resulting state / acceptance: bounded plan accepted by root; Coder assigned exclusive ownership of mod.rs, server_terminal.rs and neumorphic_tests.rs.
- Remaining work and handoff: actual-widget failed-server check, repeat status/no duplicates, re-expansion after Tools/Settings, retained terminal session, focused tests and independent review.
