# Research GitHub CLI as the operations backend

- Local timestamp / acting role / milestone: 2026-10-27T10:18:14.298065-04:00 / Researcher (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Investigate proposed GitHub CLI support, record per-server encrypted credential files, and advise whether they belong inside package-install folders.
- Starting repository state: Documentation-only repository; D24/D25 auth ownership/store/readback/service-test required, general auth review underway, no implementation authorized.
- Work performed / changed files: Read-only current primary docs/source for gh v2.103.3 install/auth/store/refresh/API/status and maintained MCP bridges. Checked licenses/maintenance/dependencies and direct-wrapper limitations. No specialist writes.
- Decisions: CLI operations feasible; no built-in general MCP server or automatic compliance with Fetch-owned per-server encrypted files. All integration/tool/install/auth/refresh designs remain proposals.
- Checks and evidence: Official login has OS-store/plaintext fallback; env tokens override stored tokens and are not refreshed; token export omits refresh lifecycle; GH_CONFIG_DIR does not namespace host/user keyring state. Reviewed actual authenticated /user HTTP response option. Active shuymn bridge launches official MCP; gh-aw shows named gated tools; inactive wrappers not recommended. Sources recorded in research.md; no app tests.
- Resulting state / acceptance: Research inspected and source-linked proposal prepared; no implementation.
- Remaining work and handoff: Obtain placement and GitHub CLI tool/API/install/fresh-auth/renewal decisions; continue remaining contracts. No package installation, login, credential migration or personal config changes authorized/performed.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
