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
