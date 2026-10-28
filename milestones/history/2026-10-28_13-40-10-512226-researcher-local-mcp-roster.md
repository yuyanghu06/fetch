# Research the local MCP test roster

- Local timestamp / acting role / milestone: October 28, 2026, 13:40:10.512226 America/New_York / Researcher report serialized by Orchestrator / isolated-mcp-testing.
- Requested scope: Curate small local MCPs for Node/Python installation and basic protocol behavior without heavy workflows.
- Starting repository state: main at 1fae5b6 with earlier planning docs preserved; location proposed and no tests provisioned.
- Work performed / changed files: Read-only specialist compared official Everything, Filesystem, Time, Git and Fetch sources, dependencies, license/maintenance/security notes. No files changed by specialist; Orchestrator records sources/procedures in servers.md.
- Decisions: Recommend Everything and Filesystem npm 2.0.1 plus Python Time 2026.10.19. Time covers uvx without Git fixtures or web service calls. No server was installed or claimed compatible.
- Checks and evidence: Official Oct25 release identifies fixed npm 2.0.1 and date-version ordering issue. Public npm registry metadata independently confirmed both versions/Node>=24/integrity presence; PyPI confirms Time version. Documented license transition and Everything environment-exposing tool/Filesystem legacy-root limits. No package code executed, credentials read or live MCP tested.
- Resulting state / acceptance: Source-backed local trio ready for the curated plan, with clean environment/disposable files and all-not-run status. Specialist spawned no agents.
- Remaining work and handoff: Integrate two auth providers, review documentation and retain exact artifact/license/integrity/runtime checks before authorized installation. No source or product behavior changed.
