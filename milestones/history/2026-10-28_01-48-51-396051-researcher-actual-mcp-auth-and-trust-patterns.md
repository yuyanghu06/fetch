# Compare actual MCP connection and certificate approaches

- Local timestamp / acting role / milestone: 2026-10-28T01:48:51.396051-04:00 / researcher / mcp-mvp-architecture
- Requested scope: Answer how local trust setup works and how actual MCP projects handle it; read-only dedicated research.
- Starting repository state: D68 and earlier selected contracts; D69 clarification arrived during actual-MCP trust research; application implementation paused.
- Work performed / changed files: Compared official Filesystem stdio, ToolHive local HTTP issuer allowance, GitHub hosted HTTPS and official C# development HTTPS authorization sample. Root added source-linked comparison/procedures to local-oauth-trust.md/research.md. Researcher changed no files.
- Decisions: Custom local certificates are one approach, not universal MCP practice. Preserve transport-versus-authorization/issuer/metadata requirements. Current Claude OS-store support has runtime conditions; generic Node defaults do not establish every client's behavior.
- Checks and evidence: Primary official sources and maintained project/license/dependency/issue context inspected; root separately checked Apple Keychain and Claude/Node docs. No live test, client/certificate/trust/credential/environment/service change.
- Resulting state / acceptance: Research accepted with compatibility and production-installer limitations explicit; no design or dependency selected.
- Remaining work and handoff: User subsequently selected no-account browser token connection as D69; exact scheme/trust/approval/token controls remain open.
