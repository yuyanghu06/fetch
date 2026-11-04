# HTTP application routes

## November 3 requested organization

The user requires raw HTTP route handlers here, grouped in subfolders matching
URL paths, while reusable backend work may stay in separate service folders.
Current `mod.rs`, `setup/mod.rs` and `runtime.rs` only partially follow that
layout; OAuth and MCP also currently define HTTP endpoints outside this folder.
No routes or source files were moved in this explanation pass. Future extraction
must preserve URLs, behavior and shared backend authorization.

D81: provider callbacks now retain one optional iss value and reject duplicates;
generic OAuth validates its exact saved issuer before exchanging codes. Shared
browser HTML is supplied by src/browser.rs without changing API authority.
Independent fixture HTTP/MCP/TLS checks pass within the combined68-case suite;
actual native/provider/client operation remains unproved. Older no-execution
paragraphs below describe their original source/compile passes.

HTTP setup discovery and runtime.rs's MCP discovery adapter use the same installed-aware scan. Exact saved source path/name associations produce installed flags and existing server IDs; saved-record corruption is a safe discovery error.

Handlers use shared backend services. Application JSON responses include `next_steps` and are never cached. Host (or HTTP/2 authority) and optional Origin must match the configured local address; proxy headers grant no authority. The fixture-only developer mode keeps authorization-server metadata unavailable; the native application wires the implemented OAuth service. Protected reads authorize before query validation or filesystem metadata. No developer token bypass exists. HTTP mock-authority tests live outside production code.

runtime.rs adapts installation/server/config/credential/auth/restart/tool routes to shared backend services. Config edits are Fetch execution settings only. Native Settings authority is checked before manual-key parsing and request-bound Mac confirmation; direct Keychain storage does not imply verified account access. Authenticate accepts optional interactive (default true); a per-run background header authorizes only quiet authenticate requests and no other route. Installed-tool arguments/results use original RawValue JSON. The management-tool adapter calls the same services and preserves application JSON guidance. All new handlers are unverified source in D77; no HTTP requests have been made.

D79 removes an unused import during native binary compilation. The binary builds successfully; HTTP/native/client behavior remains untested and no listener was launched.

The outer router layer logs every incoming application/OAuth/MCP HTTP request,
including boundary/fallback failures, through diagnostics.rs. Fetch-owned error
codes and shared authentication outcomes are logged with generated request
context; response streams and raw provider data remain unchanged.

The new DELETE /api/v1/servers/{server_id} handler is in
v1/servers/server_id/delete.rs. It reuses the existing scope/target/error helpers
and delegates to shared Manager deletion; existing route families remain in place.

## Server-owned state changes

POST /api/v1/servers/{server_id}/open-folder lives in the URL-shaped open_folder.rs
module. It accepts no query/body/path override, requires a native Settings session
and calls shared Manager folder resolution/opening. Unknown native locations fail
clearly; bearer grants cannot open a desktop window. The credentials PATCH remains
for native-confirmed manual fixed keys through existing bindings. Config PATCH and
the matching MCP tool reject former launch/environment/header editing fields.
GET server authentication projects safe expiry/last-verified metadata without
performing renewal. Raw MCP values and the common no-store/next_steps boundary stay
unchanged. New fixture/native acceptance is recorded in the active milestone.
