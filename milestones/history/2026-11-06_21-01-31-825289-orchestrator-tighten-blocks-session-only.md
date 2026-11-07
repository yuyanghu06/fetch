# Remove command-block gap and correct session scope

- Local timestamp / acting role / milestone: 2026-11-06T21:01:31.825289-05:00 / Orchestrator directly / connect-command-code-blocks.
- Requested scope: Remove blank top row in command blocks; limit no-sub-session instruction to this session; confirm global configuration untouched.
- Starting repository state: Earlier blocks had separate Copy header; unintended lasting project instruction was added by the prior pass.
- Work performed / changed files: Put command text and Copy in one centered row in src/ui/components.rs. Removed only this session's additions to AGENTS.md and NOTES.md; both now match HEAD. Corrected rolling milestone/to-do/guide wording. Earlier immutable history remains and is superseded by this correction.
- Decisions: No persistent delegation preference. No global/client config edits; read-only metadata and TOML parse check performed. No source or client configuration was changed during that check.
- Checks and evidence: git diff -- AGENTS.md NOTES.md is empty. Global Codex TOML parses. Signed build-only launcher exits 0; compile9.75s, development signing/strict verification pass. git diff --check passes. User screenshot shows original code blocks; revised native row not observed.
- Resulting state / acceptance: Gap fix implemented and built. No app relaunched or personal settings edited. This pass used no sub-sessions.
- Remaining work and handoff: New reported Codex-to-Fetch /oauth/authorize403 is under direct investigation. User subsequently requested removing the cleanup checkbox/prompt; implement alongside the scoped sign-in repair.
