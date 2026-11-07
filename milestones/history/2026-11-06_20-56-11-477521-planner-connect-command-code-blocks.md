# Copyable connection commands plan

- Local timestamp / acting role / milestone: 2026-11-06T20:56:11.477521-05:00 / Planner, recorded by Orchestrator / connect-command-code-blocks.
- Requested scope: Make Claude Code and Codex connection commands two copyable code blocks.
- Starting repository state: Clean working tree; Connect apps renders both as ordinary text. Shared role/workflow files were not found at the referenced locations; specialist follows roles stated in AGENTS.md.
- Work performed / changed files: Read-only independent plan inspected connect_view, shared components/theme and existing clipboard action. This record and rolling tracking updated by Orchestrator.
- Decisions: Shared fitted-height monospace block and Copy button; caller uses identical display/clipboard strings. Preserve commands, endpoint and existing clipboard path. Reuse existing rounded container, button, spacing and text size.
- Checks and evidence: Message::Copy already invokes iced::clipboard::write. No execution or native observation.
- Resulting state / acceptance: Plan accepted for bounded UI implementation; no new API, dependency or configuration.
- Remaining work and handoff: Implement, build and independently review. Native appearance/copy observation remains separate; do not relaunch existing app automatically.
