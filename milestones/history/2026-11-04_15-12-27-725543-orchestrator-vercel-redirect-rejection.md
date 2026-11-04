# Explain Vercel return-address rejection

- Local timestamp / acting role / milestone: 2026-11-04T15:12:27.725543-05:00 / Orchestrator / Vercel authentication diagnosis; wrapper-auth removal remains complete.
- Requested scope: Explain supplied registration failure and what the return address means.
- Starting repository state: Six custom gates removed and accepted; user now supplies actual Vercel registration HTTP400 invalid_redirect_uri.
- Work performed / changed files: Read sdk.rs begin, Manager callback construction and ApplicationHost origin/default port; read official Vercel MCP docs/changelog; updated canonical research and current records.
- Decisions: No new login route, callback method, public hostname or hosting selected. Existing SDK only receives redirect and Fetch client name; no CIMD URL or pre-registered client supplied.
- Checks and evidence: src/application.rs builds https://localhost:port, development default53672; Manager uses origin/oauth/provider/callback. Exact running port not proven by logs, so53672 is described as default. Supplied HTTP400 says Vercel rejects registered redirect URIs. Official https://vercel.com/docs/agent-resources/vercel-mcp and https://vercel.com/changelog/vercel-mcp-now-supports-client-id-metadata-documents describe approved clients or HTTPS-hosted client identity document support. No live account/network service testing or source edits.
- Resulting state / acceptance: Meaning of return address established; registration-stage provider rejection now concrete, rather than earlier generic hypothesis. Completing browser login not established.
- Remaining work and handoff: Explain the local browser return URL plainly. Supporting Vercel identity requires a separate bounded configuration/hosting decision; do not invent a hostname, deploy or copy another client's identity.
