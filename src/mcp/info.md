## November 12 — project-bound access and history

The same gateway serves Shared /mcp and /mcp/projects/{id}. Exact verified
resource, legacy sessions and control replies must match the endpoint. Server
metadata/content/calls and change notifications filter current project membership;
dispatch rechecks the resolved server/tool. Only fetch__list_sessions and
fetch__read_session expose project history, behind a separate permission.
History-only grants can use those tools. File/subagent APIs remain native-only.
Upstream results and the server-first tool interface remain unchanged.

## November 12 — persistent server on/off implementation

fetch__restart_server advertises and forwards optional action:stop to the same management backend. Stop preserves raw upstream data contracts and requires existing server-management authority. Locked library compilation, 30 focused checks, final signed build and independent Evaluator acceptance pass; live user/provider acceptance remains separate.

## November 6 — initial version negotiation

An authenticated initialize request must supply a protocol version, but its initial offer need not already equal Fetch's supported handshake version. Fetch returns2025-11-25and the client decides whether to continue. This corrects standard negotiation for the real Codex0.166.0offer2025-06-18; it does not implement an extra operating version. Existing negotiated headers/session/authentication and modern mode checks remain. tests/local_mvp_acceptance.rs exercises this offer, initialized notification and actual management-tool listing.

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

This module exposes the one protected `/mcp` endpoint. It keeps legacy initialization/session ownership separate from modern per-request capability metadata, wraps tool calls in native SSE messages, and sends quick reads as JSON. The default catalog contains Fetch-owned discovery/call tools and permitted management tools. Selected upstream definitions are returned as discovery data; the generic caller sends the original name/arguments and returns the native upstream result through existing revision rules. Historical direct aliases remain callable under existing permissions but are not exported by default.

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

The restart management tool accepts interactive:true for the existing start/login
API. Its description explains pending host consent and account-proof limits. The
authenticate tool stays available but is described as deprecated compatibility.
No provider tool definitions/results are rewritten by this change.

November 4 correction: fetch__discover_servers now describes a selected `.codex` folder as reading only its config.toml, matching the shared service used by HTTP discovery. Parent Claude/OpenCode configurations are excluded for this selection.

## Server-first tool discovery — November 12

The default native tools/list exposes Fetch's fixed list-servers, selected-server
tool-list, and call-tool helpers, plus permitted management helpers. It no longer
requests or publishes every installed upstream tool during startup. Both legacy
initialize.instructions and modern server/discover.instructions include the full
permitted installed server names/IDs and the discover/select/inspect/call sequence;
the Fetch list-servers tool description repeats that information for hosts that
only expose tool definitions to their model. Startup metadata reads do not contact
upstream servers.

management.rs owns only Fetch helper descriptions and input schemas. Selected
fetch__list_tools requires a nonempty server_id array; its ordinary HTTP equivalent
retains its existing optional filter. fetch__call_tool unwraps only routing fields
and forwards the inner raw arguments through the existing Runner and native
result adaptation. Upstream results are not wrapped as management JSON/text.
Outer continuation state, input responses, progress, cancellation and permission
checks are preserved. The generic caller has no upstream-specific outputSchema,
and client approval displays the Fetch caller rather than an individually
registered upstream tool. This fixed catalog does not depend on per-connection
side effects or dynamic tool registration. Source is implemented; root coordinates
isolated verification and no real-client acceptance is inferred here.
