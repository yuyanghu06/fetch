# How Fetch connects and checks login

November3,2026. The scoped refactor is implemented and independently accepted;151 checks,
separate confined original Everything check and locked native build pass.
[Exact contracts and current evidence](native-server-storage.md).

Local MCP servers and their existing helpers own login, callbacks, token files and
renewal. Fetch runs an exact recorded setup command with the same instance's HOME
and XDG locations as startup. It does not read native tokens or patch dependencies.
Unknown commands or state folders return a clear unsupported error. Normal imports
cannot yet supply those records; do not invent per-server setup recipes.

For online MCP connections, official rmcp3.8.0 code handles discovery, registration,
login, token expiry and refresh. Fetch supplies the callback/browser connection,
protected network access, instance-bound Keychain saving and existing API authority.
The button and background requests enter the actual authenticate HTTP route.
Quiet requests never open a browser or run a local setup helper.

GET servers reports cached status and nullable expiry/last-verified times. It does
not read tokens, contact a provider or renew access. The shared scheduler and a
before-use check call authenticate when known expiry is due. Unknown expiry stays
unknown; failed actions are not automatically replayed.

Saving manual keys reports stored, verified:false. For a remote endpoint, Fetch
compares rejection without private headers against successful MCP setup with them
before claiming connection access. Public setup alone and successful local setup
command exit do not prove account access. This does not prove every downstream
service account or invent a credential-test tool for each server.

Fetch's own app authorization, caller permissions, callback validation, HTTPS,
queues and raw upstream definitions/arguments/results remain protected.

Earlier decisions and implementation evidence remain in [the historical record](../../milestones/native-server-storage/historical-authentication-engine.md); conflicting encrypted-provider requirements are superseded.
