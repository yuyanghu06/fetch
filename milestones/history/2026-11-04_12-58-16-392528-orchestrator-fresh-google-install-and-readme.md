# Fresh Google MCP install and README check

- Local timestamp / acting role / milestone: 2026-11-04T12:58:16.392528-05:00 / orchestrator / google-connector-setup
- Requested scope: Check a fresh Google MCP installation for developer app settings; explain its own documentation.
- Starting repository state: Existing extensive uncommitted application/documentation work preserved.
- Work performed / changed files: Installed @a-bonus/google-docs-mcp@1.11.3 into a unique OS temporary directory with empty HOME/npm config and private cache, --ignore-scripts --no-audit --no-fund. Inspected installed auth source without launching server. Removed entire task directory in finally. Read upstream tagged README following user correction. Documentation synchronization only.
- Decisions: No application/import-policy changes or credential copying. Do not infer who originally supplied app settings.
- Checks and evidence: npm exit 0 in 7.69s; version 1.11.3; credentials.json absent; source reads supplied GOOGLE_CLIENT_ID/GOOGLE_CLIENT_SECRET then external package-root credentials.json. No account login or MCP execution; this is an install/source check, not runtime authentication proof. Existing Google definitions already provide app values, so no evidence user needs a new project.
- Resulting state / acceptance: Bounded research/install inspection complete; fresh authentication untested.
- Remaining work and handoff: Existing Fetch omission of supplied app configuration remains unresolved; no requirement to recreate an already configured Google project.
