# Persistent Fetch-only Codex

Create one reusable test Codex client in ~/Library/Application Support/Fetch-Development/clients/codex-test, outside the checkout. Its authoritative .mcp.json supplies only the development Fetch endpoint to scoped config generation. launch.command uses separate settings/session state and an empty workspace, disables apps/plugins/remote plugins, loads Fetch's public certificate for this process and avoids the shared daemon. Normal Codex settings, existing credentials and Fetch services remain untouched.

Completion requires the files to exist privately, launcher syntax to pass, Codex's configured MCP list to contain only Fetch, and the three additional tool sources to be disabled. Sign-in and actual MCP tool calls are user follow-up; no claim of filesystem or desktop isolation.

Status: Complete: permanent client and portable setup guide ready.
