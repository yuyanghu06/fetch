# Compare local OAuth certificate trust options

- Local timestamp / acting role / milestone: 2026-10-28T01:26:09.039947-04:00 / researcher / mcp-mvp-architecture
- Requested scope: Dedicated read-only research for the unresolved local HTTPS issuer/listener/browser/client-trust mechanism.
- Starting repository state: D10/D47 require standard OAuth and localhost access; broad auth/renewal/storage/verification already selected; actual issuer/trust still open.
- Work performed / changed files: Researcher checked MCP/RFC/Apple/Firefox/Node/Let's Encrypt sources and Caddy/ToolHive/mkcert maintenance, licensing, dependency and issue context. Root recorded inspected findings in research.md and the proposed local-oauth-trust.md; no researcher writes.
- Decisions: Recommend unique per-install local HTTPS certificate and explicit, preferably certificate/hostname-limited trust, subject to isolated proof. A broader CA fallback needs explicit input; no universal trust or CA:false isolation claim.
- Checks and evidence: Report includes primary links, Apache-2.0 Caddy/ToolHive and BSD-3 mkcert comparison, closed ToolHive issue6522, and client trust limitations. No services, keys, trust-store edits, personal client changes or live tests.
- Resulting state / acceptance: Research evidence accepted; recommendation remains proposed and compatibility unverified.
- Remaining work and handoff: Obtain user direction; resolve dependent exact OAuth, Settings/key, onboarding and test/release choices before authorized runtime work.
