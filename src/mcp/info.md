# Client-facing MCP

fetch__delete_server shares Manager's per-server deletion permission, busy checks,
connection shutdown and owned-state cleanup with the HTTP DELETE handler. The
management adapter does not delete files or credentials directly. Its metadata
describes destructive scope, preserved source data and partial/uncertain outcomes.

D83 installation management now invokes managed preparation plus normal Connect;
successful connections remain running. Its description distinguishes preparation
from runtime/authentication and directs clients to existing status reads. No
extra workflow is implemented in this protocol adapter.

The discovery management tool describes and returns installed-row metadata from the same backend projection as HTTP discovery. An installed source file/name pair references an existing server rather than another installation selection. This does not merge different accounts by package/name or claim authentication ready.

This module exposes the one protected `/mcp` endpoint. It keeps legacy initialization/session ownership separate from modern per-request capability metadata, wraps tool calls in native SSE messages, and sends quick reads as JSON. Imported definitions change only their routing selector; imported terminal results retain their raw values except the approved missing legacy completion marker and native content framing.

Fetch's eight management tools call a `ManagementBackend` supplied by the application, so installation, authentication, configuration and restart keep the same backend rules as HTTP and Settings. There is no manual credential writer or provider model in this module. Source is unverified; the user controls when any checks or tests start.

D79 corrected the gateway's Rust match-guard syntax and unused imports. The native binary compiles with the resolved lockfile; no MCP messages, sessions or native/runtime behavior were exercised. One unread decoded-error field warning remains.
