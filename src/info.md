workspace/ is the file-backed Workspace service: recognized native instruction/agent files, converted drafts, app model catalogs and bounded original session streams. Native API handlers enforce access; the UI wraps them. Session MCP is deferred.

connector/ is the stdio-to-existing-HTTPS transport and separate approved client profile store. The connect CLI does not own the app/server lifecycle; setup is the explicit launch/focus path. Native approval lives in oauth/connections.rs and ui/connector_setup.rs.

## Native terminal and saved console — November 4

terminal/ owns backend PTY sessions and account-shell/process cleanup. API route handlers and Manager provide native-only authority and the exact MCP launch environment; ui/server_terminal.rs wraps those APIs and the existing iced_term parser/renderer. runner/log_store.rs persists sanitized service output separately from terminal bytes. New terminal unit tests use confined disposable shells; UI fixtures do not open native windows. Current acceptance is tracked in milestones/server-console-terminal/.

# Fetch backend and native interface

Current wrapper implementation uses official rmcp3.8.0 auth-only functions for
online sign-in/renewal; manager/network keeps common destination protection,
remote_oauth connects its store to direct Keychain, and application wires restricted
actual self-HTTPS authenticate requests. Native servers/helpers keep account files
and refresh. Provider bridges/Google interception/custom token rules are removed.

credentials now protects only application TLS identity/coding-app authorization;
provider IDs are rejected without accessing old files/keys. storage keeps internal
launch records and registration-only removal. runner/mcp preserve raw messages,
permissions, queues/deadlines/cancellation and native forwarding. API/UI wrap shared
backend operations; native fixed-key Settings and known-folder actions are separate
from verified login. Native metadata authoring remains an explicit limitation.

Initial141 relevant fixtures and locked native binary build pass; final independent
review/real confined MCP evidence are tracked in milestones/native-server-storage.
No actual native window/Finder/Keychain/provider/client or universal compatibility
claim follows from these fixtures. Personal state and existing services remain.

Existing debug/release roots/ports are unchanged. HTTP handlers remain in api and
legacy oauth/mcp registration modules; new open-folder uses URL-shaped subfolders.
The separately recorded broader route-file reorganization is not part of this pass.

## Automatic first connector approval — November 12

application.rs accepts bounded nonsecret connector-approval intents on its existing private activation socket and cold app startup. Activation itself grants no access; native UI uses the existing pinned HTTPS approval routes.
## Project-bound access

`projects/` stores explicit native project registrations. `authority`, `oauth`,
`connector` and `application` retain one project resource across approval,
activation, profiles and transport. `manager`, `api` and `mcp` enforce current
Shared+own server membership. `workspace/project_sessions.rs` shares bounded
session-only reads between HTTP/MCP; instruction/subagent editing stays native.
See the project-connections architecture and milestone evidence for limits.
