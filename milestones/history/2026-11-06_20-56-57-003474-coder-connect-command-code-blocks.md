# Implement copyable connection command blocks

- Local timestamp / acting role / milestone: 2026-11-06T20:56:57.003474-05:00 / Coder, inspected and recorded by Orchestrator / connect-command-code-blocks.
- Requested scope: Two separate copyable command blocks for Claude Code and Codex.
- Starting repository state: Clean source plus accepted plan/journal updates.
- Work performed / changed files: Added shared code_block in src/ui/components.rs and replaced two plain-text elements in src/ui/mod.rs. Orchestrator updated src/ui/info.md, docs/SHARED_UI_COMPONENTS.md, docs/UI_STATE_COVERAGE.md and milestone guides/tracking.
- Decisions: Reuse rounded_box, shared padding/gap/text size and ordinary button. A single owned command string supplies both monospace display and Message::Copy. No command/backend/configuration change.
- Checks and evidence: Orchestrator inspected actual diff; Coder scoped git diff --check passes. No runtime or tests. Build tools available; selected local development identity found with matching private-key identity metadata. Strict signing verification remains for build.
- Resulting state / acceptance: Both blocks implemented; build and independent review next.
- Remaining work and handoff: Build with signed --build-only launcher; review exact clipboard wiring and bounded scope. Native appearance and clipboard observation pending deliberate user relaunch.
