# Clarify model arguments versus MCP request framing

- Local timestamp / acting role / milestone: 2026-10-27T13:57:30.375966-04:00 / planner / mcp-mvp-architecture
- Requested scope: Apply input/rawresult correction without broad route approval.
- Starting repository state: Prior call proposal placed Fetchserver_id/name/arguments in body; user asks standard raw input and rawresult.
- Work performed / changed files: Read-only Planner mapped input layers and proposed targetoutsidebody; user subsequently clarified model-supplied arguments only; root incorporated latestclarification.
- Decisions: Arguments-only object selected; internalMCPclient constructs method/name/arguments envelope. ExactHTTPpath and othercallpolicies unapproved; rawresult means terminalToolResult.
- Checks and evidence: Existing currentMCP request research supplies protocol facts; no repeat live research/SDKtests.
- Resulting state / acceptance: Bounded amendment applied after user resolved input-layer ambiguity.
- Remaining work and handoff: Independently verify/evaluate D39/currentdocs; answer with one simple forwarding example.
