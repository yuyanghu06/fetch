# Project-bound connections

Build the user-requested project-scoped MCP server support and session reading
using one binding. Packages stay central; membership lives in existing server
config.json. Native project registry, scoped installation/management and separate
project grants/profile/resource URLs enforce Shared+own visibility. Session tools
require an explicit project-history permission. Workspace file APIs stay native.

Approved route/auth contract:
[project-connections.md](../../docs/architecture/project-connections.md).
Status: implemented and independently **ACCEPT**ed. Registry, scoped operations,
OAuth/connector bindings, bounded session APIs/tools and native API wrappers are
complete.57unique named focused checks pass, including existing compatibility
checks. Signed development build-only/strict signature verification passes22.95s.
See [evidence](evidence.md) for commands, screenshots, corrections and actual limits.

Completion criteria are satisfied for this bounded source/fixture/build slice.
Actual Fetch/coding clients, personal projects, Keychain approvals and providers
were not exercised. Project-specific notification/completion boundaries were
source-reviewed without dedicated runtime scenarios. No personal files/settings,
credentials, trust, running apps/services, commit/push or release changes.

Next user action: Quit/reopen the signed development executable, choose a folder
in Workspace, register it for MCP, then choose that project in Connect apps and
apply the shown snippet. Each project/client approval is separate; history is
optional and initially unchecked.
