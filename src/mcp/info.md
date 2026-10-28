# Client-facing MCP

This module exposes the one protected `/mcp` endpoint. It keeps legacy initialization/session ownership separate from modern per-request capability metadata, wraps tool calls in native SSE messages, and sends quick reads as JSON. Imported definitions change only their routing selector; imported terminal results retain their raw values except the approved missing legacy completion marker and native content framing.

Fetch's eight management tools call a `ManagementBackend` supplied by the application, so installation, authentication, configuration and restart keep the same backend rules as HTTP and Settings. There is no manual credential writer or provider model in this module. Source is unverified; the user controls when any checks or tests start.
