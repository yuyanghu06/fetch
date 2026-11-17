# Check how Codex and Claude choose a subagent model and limit writes

- Local timestamp / acting role / milestone: 2026-11-17T17:36:22-05:00 / Researcher / Workspace agent actions
- Requested scope: Verify what happens with an omitted model and what Claude Code provides adjacent to Codex read-only sandbox mode.
- Starting repository state: The editor supported omitting model and manually refreshing its catalog. Read-only source settings were flagged during conversion.
- Work performed / changed files: Read the applicable OpenAI docs skill, official Codex custom-agent config and Claude Code subagent documentation. No application changes in this pass.
- Decisions: Omitting model is supported by both apps. Codex uses explicit spawn, configured default and parent fallback. Claude uses its model resolution order and supports explicit inherit. Claude Plan mode is adjacent and may yield to parent permissions. User selected mapping Codex read-only to Claude Plan.
- Checks and evidence: [Codex model and sandbox docs](https://learn.chatgpt.com/docs/agent-configuration/subagents) and [Claude Code subagents](https://code.claude.com/docs/en/sub-agents).
- Resulting state / acceptance: Model question answered. Plan mapping approved for implementation.
- Remaining work and handoff: Apply the narrow conversion and validate menu loading and size.
