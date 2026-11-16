# Separate session errors

- Local timestamp / acting role / milestone: 2026-11-16T13:11:56.768164-05:00 / coder / workspace-session-error
- Requested scope: isolate history failures from file editing.
- Starting repository state: existing extensive changes preserved.
- Work performed / changed files: Coder added sessions_error to src/ui/workspace.rs and documented it in src/ui/info.md.
- Decisions: preserve real messages and eager loading; show history failures only in Sessions.
- Checks and evidence: Coder git diff --check passes; inspected resulting state/error handlers.
- Resulting state / acceptance: implementation ready for independent test.
- Remaining work and handoff: independent test and final UI rendering follow-ups.
