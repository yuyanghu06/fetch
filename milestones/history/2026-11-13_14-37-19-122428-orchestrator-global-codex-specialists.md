# Create global Codex specialists

- Local timestamp / acting role / milestone: 2026-11-13T14:37:19.122428-05:00 / Orchestrator / global-codex-specialists.
- Requested scope: user explicitly requests creating global definitions corresponding to the shared team.
- Starting repository state: no global agents directory/registrations/shared workflow; other personal settings and large dirty project tree preserved.
- Work performed / changed files: personal ~/.codex/agents five role TOMLs plus info.md; ~/.codex/AGENT_WORKFLOW.md; appended agents registrations to ~/.codex/config.toml. Original private config backup under ~/.codex/backups; no secret/config body in repository. Project journal/notes updated.
- Decisions: five specialists only; root stays Orchestrator. Models/reasoning inherit. Read-only roles use native read-only sandbox; writers obey inherited scope. No personal MCP registration changed.
- Checks and evidence: official matching rust-v0.167.2 config schema fields checked; TOML parsed and unrelated semantic configuration compared equal. Installed codex features list returns0. Five role paths validated under global root; no model/reasoning overrides.
- Resulting state / acceptance: requested global configuration complete. No providers/MCP sessions, app launches, personal session execution, commits/pushes/releases. Existing running chat reload behavior not claimed.
- Remaining work and handoff: refresh Fetch Global/Codex list for files; continue new full-history reader task.
