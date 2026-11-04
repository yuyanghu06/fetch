# Current storage scope

November3,2026. [Exact ownership and route contracts](native-server-storage.md).

The initial MCP MVP requires no PostgreSQL server or retention database. Shared
memory, its history/undo, graph work and future database choices remain deferred.

| Data | Who keeps it |
| --- | --- |
| Installation/launch records and Fetch execution settings | Fetch's private local records, outside replaceable packages. Public config editing is execution settings only. |
| Native provider accounts/tokens/refresh | Unmodified local server/helper, using its supported files/authentication process. Separate HOME/XDG locations where honored; no guarantee for servers that ignore them. |
| Online-client OAuth grants and manually entered fixed keys | Shared direct Keychain store, separate root/instance and SDK/manual sections; no plaintext fallback. |
| Fetch TLS identity and coding-app authorizations | Existing protected application files/keys, restricted to the two exact app purposes. |
| Installation/restart progress and owned process/queue state | Existing backend records/live state and same-run monitoring contracts. No new durable history or cross-Quit job-recovery promise. |

Removing a server stops its idle owned context and removes only registration.
Native state, packages, old encrypted files/keys and online grants are preserved.
Existing state is not automatically migrated, decrypted, deleted or exposed.
No file watcher, automatic file-change restart, substitute database or personal
client configuration edit is introduced.

A recorded native state folder is required for Open server folder. A package or
HOME path is not assumed to contain the files the server actually uses. Ordinary
imports have no approved command/path authoring field yet; unsupported metadata
returns a clear error.

Earlier decisions and implementation evidence remain in [the historical record](../../milestones/native-server-storage/historical-storage-scope.md); conflicting encrypted-provider requirements are superseded.
