# Google MCP developer setup research

- Local timestamp / acting role / milestone: 2026-11-04T12:58:16.392527-05:00 / researcher / google-connector-setup
- Requested scope: Check a fresh Google MCP installation for developer app settings; explain its own documentation.
- Starting repository state: Existing extensive uncommitted application/documentation work preserved.
- Work performed / changed files: Read-only primary-source comparison of @a-bonus/google-docs-mcp 1.11.3 and OpenAI plugin metadata. User clarified the requested target is the local Google MCP, not the built-in plugin. Documentation synchronization only.
- Decisions: No application/import-policy changes or credential copying. Do not infer who originally supplied app settings.
- Checks and evidence: Published archive has no credentials.json or credential-generating install hook. README requires create/select Cloud project and supplied client ID/secret; hosted deployment puts these settings on its operator.
- Resulting state / acceptance: Bounded research/install inspection complete; fresh authentication untested.
- Remaining work and handoff: Existing Fetch omission of supplied app configuration remains unresolved; no requirement to recreate an already configured Google project.
