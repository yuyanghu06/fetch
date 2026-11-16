# Clarify Codex project add command versions

- Local timestamp / acting role / milestone: 2026-11-12T13:42:46.483958-05:00 / Orchestrator / project-scoped-servers.
- Requested scope: Is there a single project add command, and which Codex versions support it?
- Starting repository state: Prior explanation could imply newer Codex versions have a project flag; no such release had been established.
- Work performed / changed files: Read installed codex --version and add --help; searched and opened current official CLI reference. Updated proposal and milestone records.
- Decisions: No product changes. A Fetch setup helper is possible but remains unselected/unimplemented.
- Checks and evidence: Installed 0.167.1 has no scope/project output flag. Official reference describes global MCP entry management and no project output option. No exhaustive historical release audit claimed. Read-only help/version only; no config, login, server, installation or trust changes.
- Resulting state / acceptance: Correct misleading implication about newer versions; distinguish project config support from an add command. No independent review needed for this brief factual clarification.
- Remaining work and handoff: Project feature remains proposed; Apple app signing versus HTTPS trust question and actual Claude certificate acceptance remain pending.
