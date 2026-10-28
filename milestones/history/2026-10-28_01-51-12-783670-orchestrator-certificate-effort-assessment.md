# Estimate certificate setup and verification effort

- Local timestamp / acting role / milestone: 2026-10-28T01:51:12.783670-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Explain how hard it would be to issue a local certificate and verify that it works.
- Starting repository state: D69 no-account local browser-token direction selected; exact scheme/certificate/authority controls remain open; application implementation and live tests paused.
- Work performed / changed files: Reused inspected MCP/client/trust research and rechecked Apple self-signed certificate and current Claude trust documentation. Added a clearly qualified effort/proof-scope assessment to local-oauth-trust.md; refreshed snapshot/to-do and affected folder guides.
- Decisions: No new product choice. Estimate minutes to create, a few hours for browser/one-client certificate proof, roughly one to two days for first three-client trust compatibility pass after environment/test-service readiness; exclude VM provisioning and Fetch authorization implementation. Certificate proof is distinct from full token/renewal/tool/reconnect proof.
- Checks and evidence: Read back the added section; checked uniqueness, estimate prerequisites, retained D69 and proposed-HTTPS status. Primary Apple/Claude pages rechecked. Root documentation checks only, not independent or live compatibility tests.
- Resulting state / acceptance: Effort estimate documented with uncertainty and meaningful success criteria; no certificate/private key, trust entry, client configuration, service or environment created or modified. Milestone remains active/incomplete.
- Remaining work and handoff: Answer in plain English; exact scheme/trust and complete account-free browser/token contract remain open. Select and authorize isolated environment/test scope before any actual proof.
