# Separate session errors from the instruction editor

- Local timestamp / acting role / milestone: 2026-11-16T13:07:58.701748-05:00 / Planner, integrated by Orchestrator / workspace-session-error
- Requested scope: user-reported missing-rollout error while opening Codex AGENTS.md.
- Starting repository state: extensive existing modified/untracked work; preserved. Official reader scope previously accepted.
- Work performed / changed files: read-only Planner traced eager session loading and the shared error field; no application edits. Orchestrator creates scoped journal.
- Decisions: give session-list failures a separate field, show only under Sessions, retain the real message. Do not alter native current-file selection or add fallback behavior.
- Checks and evidence: inspected src/ui/workspace.rs, reader canonicalize failure and focused widget fixtures. No runtime acceptance yet.
- Resulting state / acceptance: bounded plan accepted; Coder next.
- Remaining work and handoff: implement, independently test/review, build/sign; personal-definition Markdown formatting awaits user answer.
