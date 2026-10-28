# Research complete account-free OAuth requirements

- Local timestamp / acting role / milestone: 2026-10-28T02:08:51.614475-04:00 / researcher / mcp-mvp-architecture
- Requested scope: Reuse completed auth research and verify concrete endpoint/client/rotation constraints.
- Starting repository state: Documentation-only repository; D69 account-free direction selected, D70 approval received this turn; application implementation and live testing remain paused.
- Work performed / changed files: Checked official MCP/RFC discovery, public DCR defaults, CIMD coverage, PKCE/redirect/resource/issuer responses, token/revocation/refresh behavior and current Codex/Claude/OpenCode evidence. Used OpenAI Docs skill for official Codex facts. No files changed.
- Decisions: DCR is demonstrated across all three; CIMD-only is unsafe for inspected coverage. DCR-first deferral needs explicit input. Defaults/race behavior and human authority are Fetch proposals, not standards-imposed values.
- Checks and evidence: Primary links captured in client-authorization.md; maintained ToolHive/Fosite/C# evidence reused without dependency selection. No live clients or credentials.
- Resulting state / acceptance: Documentation/research/plan pass inspected; architecture active/incomplete; no new user approval beyond D70, no runtime compatibility claim.
- Remaining work and handoff: Design explicit human authority; preserve exact RFC registration defaults and standard envelopes.
