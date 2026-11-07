# Research pre-login agent guidance

- Local timestamp / acting role / milestone: 2026-11-06T21:11:27.802276-05:00 / Primary agent directly, read-only source/research / pre-login guidance proposal.
- Requested scope: Determine whether pre-auth documentation can tell the coding agent to run a command that starts coding-app-to-Fetch login; user confirmed this login direction.
- Starting repository state: Approval403/Connect apps source changes and passing38checks/signed build remain pending real user retry.
- Work performed / changed files: Inspected existing MCP authorization ordering/public resource metadata; read official MCP authorization, RFC9728 and official OpenAI MCP documentation; ran installed codex mcp login --help only. Recorded findings in docs/architecture/research.md and current tracking.
- Decisions: None selected. Recommend separately delivered client-readable setup instruction/skill for reliable pre-login visibility, optionally complemented by resource_documentation link. Do not promise metadata is surfaced to model or executed. Preserve normal human browser/Mac approval and correct client account/configuration.
- Checks and evidence: Current Fetch authorization precedes MCP initialization/tools. Public metadata lacks documentation link. RFC permits human documentation URL but does not mandate model visibility. Official docs and local CLI confirm login command. No tests/builds/logins/services/configuration changes in this pass.
- Resulting state / acceptance: Research answer ready, implementation remains proposed. All work direct, no sub-sessions or independent checks.
- Remaining work and handoff: User chooses delivery mechanism before any new route/settings/client-instruction installation. Actual client visibility/reconnection requires focused isolated verification if selected.
