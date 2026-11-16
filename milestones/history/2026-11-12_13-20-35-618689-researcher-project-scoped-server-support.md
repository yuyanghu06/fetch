# Research project-scoped server support

- Local timestamp / acting role / milestone: 2026-11-12T13:20:35.618689-05:00 / Researcher (research_project_scope), serialized by Orchestrator / project-scoped-servers.
- Requested scope: Recommend how to add project-scoped MCP servers to Fetch; research only.
- Starting repository state: Extensive pre-existing source/docs/UI work; no project membership in installed server records or MCP sessions found. Shared role/workflow files were not available at their recorded local locations; the session's explicit role instructions were used.
- Work performed / changed files: Read repository discovery, installation, authorization and MCP connection code. Dedicated read-only Researcher checked current Claude Code and OpenCode documentation and OpenCode implementation/license/dependencies/issues. Findings recorded in docs/architecture/project-scoped-servers.md and milestones/project-scoped-servers/info.md. Shared journal writes serialized by Orchestrator.
- Decisions: None newly approved. Recommend explicit project registration, project-specific connection addresses, shared-plus-project access restricted by approved permissions, distinct logical server settings and unchanged server-owned login. Do not use a single active project or roots as authority.
- Checks and evidence: Current OpenCode V2 docs use mcp.servers whereas Fetch reads older mcp entries; Claude supports project .mcp.json and local/project/user precedence. Sources linked in proposal. No builds/tests/apps/installations/accounts/personal configuration changes. Independent research, not runtime verification.
- Resulting state / acceptance: Research complete; recommendation remains unapproved. Evaluator review requested separately.
- Remaining work and handoff: Review the proposed behavior with the user, then complete API/authentication contracts before implementation. Real protected two-project client acceptance remains unrun; existing acceptance tasks preserved.
