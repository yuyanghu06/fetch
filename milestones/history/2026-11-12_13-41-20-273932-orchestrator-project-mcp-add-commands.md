# Check project MCP registration commands

- Local timestamp / acting role / milestone: 2026-11-12T13:41:20.273932-05:00 / Orchestrator / project-scoped-servers.
- Requested scope: Exact Claude/Codex commands for project-scoped MCP registration from a repository.
- Starting repository state: Existing source/docs changes preserved; project connection filtering remains proposed.
- Work performed / changed files: Read official client documentation and ran only installed CLI add --help. Added command findings to project-scoped-servers.md and journal/snapshot/to-do.
- Decisions: No new product behavior selected; no server/client registrations performed.
- Checks and evidence: Claude installed help supports --scope project (default local). Installed Codex add help has no scope/project flag. Official OpenAI docs confirm project config support. No package installation, MCP connection, auth, certificate/trust or personal/project configuration edits.
- Resulting state / acceptance: Commands explained with installed-version limit; current Fetch URL registration does not implement upstream project filtering. Direct factual review, no independent specialist pass.
- Remaining work and handoff: Project support implementation still unapproved; certificate issue still needs actual user retry/verification.
