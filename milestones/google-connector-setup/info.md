# Google MCP setup check

Goal: determine whether the Google MCP used in existing local Codex definitions includes Google developer app settings. Scope: fresh disposable installation, package/source inspection and upstream README; no live account or server execution.

Complete: npm installation of @a-bonus/google-docs-mcp 1.11.3 succeeded; app credentials are external, not bundled. README says create or select a Cloud project, enable APIs, configure consent and supply a desktop client ID/secret, then authorize. Existing local definitions already supply those settings. The earlier built-in plugin comparison was not the user’s requested target. Temporary install/cache/state removed.

Completion criteria: package contents checked, documentation read, cleanup verified, limits recorded. No Fetch implementation or personal settings changed.
