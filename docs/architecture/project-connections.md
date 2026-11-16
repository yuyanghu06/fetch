## November 13 — automatic native registration selected

The user selected automatic registration whenever a project folder is opened in Workspace. After successful folder discovery, the UI submits the existing POST /api/v1/projects operation if the folder is not registered; it resumes this check after project-list readiness or another pending registration. The selected scopes in Servers, Install MCPs and Connect apps remain independent. The manual registration button/status row is removed. Existing input/result/permission/storage rules are unchanged. Previously browsed folders without a saved registration become registered when opened/refreshed again; no historical folder scan was added.

# Project connections, servers and session access

November12,2026. User explicitly approved this complete route/authentication
contract and requested implementation ("Approve and build this contract").
Workspace's existing native-only file/config APIs remain unchanged.

## Everyday behavior

Register a project folder in Fetch. Install servers for Shared or for that
project, or move an idle existing server's assignment in native Settings. Connect
Claude/Codex with the project's generated Fetch command. Its connection sees
shared servers plus that project's servers. Shared connections see shared servers
only. Same-named records stay distinct by their actual server IDs; neither name
silently replaces the other.

Session history requires an additional displayed permission and a project-bound
connection. That connection can list/read only its project's retained sessions,
using the existing bounded readers. Shared connections never get all histories.
Instruction files and subagent definitions have no agent-facing APIs/MCP tools.
Workspace browsing remains independent of installed/running client connections.

Packages stay centrally installed. Scope is metadata in each server's existing
config.json. Native server credential/account behavior and process isolation stay
as implemented; project membership is not an OS file sandbox or a new account.

## Saved data

- A native-owned registry under `<Fetch root>/projects/` stores opaque
  `project-<32 hex>` IDs, names and canonical absolute `folder_path` values.
  Registering the same canonical folder returns its existing record; a supplied
  name is used only for a newly registered folder. There is no automatic search,
  rename/move/removal or client-file editing in this slice.
- Each server's `config.scope` is `{ "project_id": "project-…" }` or
  `{ "project_id": null }`. A missing scope is Shared for legacy records.
  Installation reuse includes scope, so A and B get distinct logical instances
  even when imported from the same definition. Existing records are not migrated.
- The approved grant family's existing immutable `resource` is the exact global
  or project MCP URL. Caller project identity is derived from that registered
  resource. Do not trust tool/body/header project claims as grant authority.
- Project connector profiles/refresh locks use `(installation, client, project)`;
  existing global profile keys remain unchanged. New project profiles require
  fresh human approval, not copied global credentials.

## Approved API contract

Application responses keep `status`, `next_steps`, and no-store. Native-only
operations use the existing native-window capability and deny normal bearer or
background tokens before resolving paths. No new personal-client file writes.

| Operation | Input and permission | Result/effects |
| --- | --- | --- |
| GET `/api/v1/projects` | Native-only; no body/query | 200: `projects` with IDs/names/folders and project MCP URLs. Read only. |
| POST `/api/v1/projects` | Native-only JSON `{folder_path, name?}` | Existing absolute directory required; canonicalize. 201 for new, 200 existing; return project and project URL. No server/login/startup. |
| Existing GET `/api/v1/servers` | Native may add `project_id=<registered ID>` for Shared+project, or `project_id=global` for Shared only; absent means all native records. Non-native selection comes only from its verified grant. | Existing response plus scope in config; enforce visibility even with guessed `id`/restart ID. Native filter never changes a client connection. |
| Existing PATCH `/api/v1/servers/{server_id}/config` | Existing execution fields; new `scope:{project_id:ID|null}` requires native authority. | 200 persisted scope; absent scope retains assignment. Project must exist. Scope changes require idle server/no active, queued, uncertain, auth, install or restart work. No restart, cwd, credential or power change. |
| Existing GET `/api/v1/setup/discovery` | Native optional `project_id`; otherwise normal Shared discovery. Project agents derive their destination from their grant; cannot select another project or source outside it. | Read selected project's supported `.mcp.json`/`.codex/config.toml` using existing parsers; exact explicit sources remain visible. Installed flags match target scope. No recursive repo search. |
| Existing POST `/api/v1/setup/installations` | Existing `servers`; native optional `project_id` (null/absent Shared). Project agents inherit their project; explicit different destinations forbidden. | Existing 202/progress lifecycle, one scope per group. Scope participates in reuse. For new local project servers, missing cwd defaults to project folder; explicit upstream cwd is preserved/resolved using source rules. Preparation/connect/power behavior unchanged. |
| Existing GET `/api/v1/setup/installations/{id}` | Existing management permission plus matching group project; native may view any. | Existing progress; guessed other-project group returns 404 without its contents. |
| GET `/api/v1/sessions` | Project-bound bearer with `fetch:sessions:read`; optional `app`, `cursor`, `limit`. No folder/file/project arguments. | 200 lightweight `sessions` names/IDs/app/time and next cursor. Default50,max200; omit private source file paths from this agent response. |
| GET `/api/v1/sessions/{session_id}` | Same project/history permission; optional byte `offset`,`limit`. | 200 raw original stream chunk, `next_offset`,`eof`, permitted session metadata. Default32KiB,max64KiB; no full-context loading or file writes. |

Global/native Workspace keeps its separately approved private folder/path APIs;
these new session operations do not turn them into agent routes. All installed
server operations intersect grant scopes/targets with current Shared+project
membership, including direct HTTP calls and management operations.

Errors:400 invalid body/path/query or unsupported selection;403 missing permission
or forbidden scope edits;404 unknown projects/servers/sessions/groups without
cross-project contents;409 busy scope change or existing installation conflict;
500/503 actual registry/storage/authorization unavailability. Existing failure
envelopes and process/job outcomes remain. Registry and scope saves are atomic;
old data stays on failed writes. No hidden retries, force scope moves or migration.

## Connection/authentication contract

- Keep existing `POST/GET/DELETE /mcp` for Shared access.
- Add `POST/GET/DELETE /mcp/projects/{project_id}` using the same protocol code.
  The registered project and exact verified grant resource must match before
  initialization, stream/control replies, continuation, cancellation or calls.
  A global grant cannot become a project grant by changing URL. Cross-project
  session IDs/resources fail closed. Query project selectors remain unsupported.
- Add GET `/.well-known/oauth-protected-resource/mcp/projects/{project_id}` with
  that exact project URL as `resource`, existing issuer, and supported scopes.
  Unknown project returns404. Existing global metadata remains compatible.
- Existing authorization-code/browser endpoints accept either the exact global
  resource or a registered project resource, and display its project name/path.
  The code and eventual family retain the requested exact resource. Native human
  confirmation and existing PKCE/redirect/token rules remain.
- Existing `/oauth/token` compares code/refresh family against the exact supplied
  resource. Refresh may narrow scopes but never change resource/project or widen
  access. Old persisted/global grants retain their old meaning.
- Add `fetch:sessions:read` as a separately displayed permission only for project
  resources. Existing global/default grants gain no history permission.
- Existing POST `/api/v1/connections/requests` accepts optional `project_id`.
  Validate registered ID; return project name/folder/resource in pending/safe UI
  responses. Existing native decision/claim retains exactly that requested
  project, selected scopes and Mac confirmation. Claim includes binding so the
  connector verifies it before saving. No agent can register a project this way.
- `fetch connect --project <project_id> --client …` selects that registered
  project's separate profile and endpoint for all POST/GET/DELETE/refresh paths.
  Existing automatic Fetch launch/approval/pending-proof handling carries project
  identity too. Reconnection/restart preserves it. No CLI user settings are
  automatically edited; show/copy project snippets through the native UI.

Both registered native connector setup and direct HTTP browser OAuth are covered;
do not publish a project address whose authentication remains global-only.

## Session MCP contract

Add only these session tools; no instruction/subagent tools:

- `fetch__list_sessions`: optional `app` (`claude`/`codex`), `cursor`, `limit`.
  Returns the same lightweight data as GET sessions, with no message contents.
- `fetch__read_session`: required `session_id`, optional `offset`,`limit`.
  Returns the same bounded raw chunk/continuation as GET session.

Both require the history permission and verified project binding. They derive the
folder from the registry, never model arguments. A history-only approved grant
can discover/call them without gaining upstream catalog/tool-call permission.
Global grants see neither tool. The current server-first tool interface and
unchanged raw upstream results remain. Filters apply to resources/prompts,
completion, install/progress, notifications and live permission rechecks too.

## Native interface

Add native project registration from the selected Workspace folder and a saved
project choice for server filtering, install destination and connection snippets.
These choices are explicit and independent of browsing/each running client.
Show scope in server rows/Settings; native Settings can reassign idle servers.
Approval shows the actual project folder and a clear history-read permission.
Use current shared controls/theme, frontend API wrappers and existing chooser.

## Verification and scope limits

Implementation accepted November12:57unique focused named checks, inspected
native-widget captures, signed build-only/strict verification22.95s and independent
Evaluator ACCEPT. [Detailed evidence and limits](../../milestones/project-connections/evidence.md).
The following fixture plan was executed within its documented limits; no real
Fetch/client/account launch. Project notification/completion boundaries were
reviewed in source without dedicated runtime scenarios.

Use two disposable projects, one Shared and one same-named server per project,
separate session streams and test-only grants/credentials. Prove independent
native registration/scope persistence; installation reuse/cwd; inventory/calls/
resources/prompts/job isolation; history listing/chunks and guessed-ID rejection;
resource/session/refresh replay denial; reconnect/restart/revocation. Include a
history-only grant and legacy global grant. Preserve server-first and Off behavior.

Reuse existing confined fixtures for any actual MCP child processes. Never use
personal client configs, services, credentials, real chats or desktop state.
Run focused real-widget controls and a signed build-only, independent evaluation,
and record actual acceptance limits. No real app relaunch, client-config write,
credential migration, packaging, release or cross-computer synchronization.
