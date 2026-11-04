# Figma registration diagnosis — Researcher

- Local timestamp / acting role / milestone: 2026-11-04T18:01:51.215696-05:00 / Researcher / client-identity follow-up
- Requested scope: Explain supplied Figma registration403 and check current Figma docs/callback requirements.
- Starting repository state: Large pre-existing modified/untracked worktree; preserved. Public Fetch identity and temporary HTTP loopback callback already implemented. Shared role/workflow files absent locally; specialist used supplied role instructions.
- Work performed / changed files: Independently checked current Figma developer docs, staff forum policy, public OAuth metadata and relevant Fetch callback/registration source. Read-only specialist; no owned files or side effects. Documentation is serialized by Orchestrator; no application source change.
- Decisions: No new product decision. Recommend official Fetch approval; callback workaround and another client's name are not selected.
- Checks and evidence: Current Figma docs https://developers.figma.com/docs/figma-mcp-server/; staff May1 policy https://forum.figma.com/report-a-problem-6/figma-s-approach-breaks-the-core-promise-of-mcp-52507; read-only public metadata https://api.figma.com/.well-known/oauth-authorization-server; src/remote_oauth/sdk.rs, src/manager/remote_callback.rs, installed rmcp3.8.0 auth.rs inspected. No builds/tests/registration/MCP connections/app launches/credentials or client configuration changes.
- Resulting state / acceptance: Diagnosis complete: approved-client restriction strongly explains403; exact rejection rule undisclosed, callback rejection unproven. Public identity method unadvertised by Figma, so library falls back to registration. This is not provider runtime acceptance.
- Remaining work and handoff: User can request Fetch approval through current documented waitlist or use an already supported client. Preserve existing source, accounts and pending work.
