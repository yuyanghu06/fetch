# Research the authentication MCP test roster

- Local timestamp / acting role / milestone: October 28, 2026, 13:40:10.512351 America/New_York / Researcher report serialized by Orchestrator / isolated-mcp-testing.
- Requested scope: Choose minimal real API-key and browser-OAuth providers for short Fetch tests, using recorded inventory where useful.
- Starting repository state: main at 1fae5b6 with prior planning docs; personal clients/providers untouched and live tests deferred.
- Work performed / changed files: Read-only specialist inspected official Tavily/Linear setup/auth/usage/pricing docs, Tavily source/issues and alternative provider costs/fixtures. No specialist file/runtime changes. Sources and qualifications are recorded in servers.md by Orchestrator.
- Decisions: Recommend Tavily API key (local npm plus optional hosted HTTP bearer) and Linear hosted read-only OAuth in dedicated test workspace. No third auth provider needed for this batch.
- Checks and evidence: Tavily /usage is an authenticated GET; keyless search is not auth proof and issue211 reports misleading isError status. Public npm metadata resolved published Tavily0.2.22 versus source-main0.2.23; installed pin behavior remains untested. Linear documents read-only HTTP endpoint/OAuth and deprecated SSE; general API refresh docs do not prove hosted MCP token lifecycle. No account login, credential use or provider tool call occurred.
- Resulting state / acceptance: Auth candidates cover distinct small checks with ownership/verification/credential/account prerequisites explicit. No universal token export/refresh or Fetch auth success claimed. Specialist spawned no agents.
- Remaining work and handoff: Combine with the local trio, verify plan/status consistency and finalize per-source private credential handoff before implemented/authorized auth tests. Keep provider tokens and raw login/log data outside Git.
