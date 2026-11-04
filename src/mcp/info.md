# Client-facing MCP

fetch__delete_server shares Manager's per-server deletion permission, busy checks,
connection shutdown and registration removal with the HTTP DELETE handler. The
management adapter does not delete files or credentials directly. Its metadata
describes preserved native state, credentials, packages and uncertain outcomes.

D83 installation management now invokes managed preparation plus normal Connect;
successful connections remain running. Its description distinguishes preparation
from runtime/authentication and directs clients to existing status reads. No
extra workflow is implemented in this protocol adapter.

The discovery management tool describes and returns installed-row metadata from the same backend projection as HTTP discovery. An installed source file/name pair references an existing server rather than another installation selection. This does not merge different accounts by package/name or claim authentication ready.

This module exposes the one protected `/mcp` endpoint. It keeps legacy initialization/session ownership separate from modern per-request capability metadata, wraps tool calls in native SSE messages, and sends quick reads as JSON. Imported definitions change only their routing selector; imported terminal results retain their raw values except the approved missing legacy completion marker and native content framing.

Fetch's management tools call a `ManagementBackend` supplied by the application, so installation, authentication, configuration and restart keep the same backend rules as HTTP and Settings. There is no manual credential writer or provider model in this module. Current verification evidence is recorded in the milestone; source alone does not prove real account authentication.

D79 corrected the gateway's Rust match-guard syntax and unused imports. The native binary compiles with the resolved lockfile; no MCP messages, sessions or native/runtime behavior were exercised. One unread decoded-error field warning remains.


November 3: fetch__discover_servers describes its optional config_path as a file or home/.codex base folder. The unchanged invocation handler uses the shared discovery service, so agents and native HTTP callers resolve the same known paths. fetch__install_servers still takes explicitly selected config_path/server_name pairs.

fetch__install_servers describes the same three POST input alternatives: a name
with config_path, command or url. Its schema enforces exactly one alternative;
the common backend owns parsing, preparation, reuse, permission and progress
rules. No separate MCP installer, new tool or UI is introduced.

## Native state and shared authentication

fetch__authenticate_server accepts optional interactive (default true); false only
checks/reuses/renews remote-client access without browser or native-helper startup.
It calls the same Manager implementation as HTTP. No package-specific OAuth or
manual-secret tool exists. Native Settings handles confirmed fixed-key saves and
actual folder opening; agents receive a human handoff for these operations.
fetch__update_server_config now accepts Fetch execution settings only, preserving
per-tool permission and merge/remove behavior. Imported raw definitions, arguments
and results stay in the existing runner/protocol path; the OAuth library does not
replace that path.
