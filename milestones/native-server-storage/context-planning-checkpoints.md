> **November 3 current implementation:** User resumed the scoped refactor.
> Official Rust MCP login/renewal, direct Keychain storage, native-owned credentials,
> retained manual-key PATCH and shared authenticate-route callers are implemented.
> Initial141 fixture checks and native build pass; final review/real confined MCP
> verification are underway. Native setup/folder metadata authoring and real
> provider/Finder/Keychain acceptance remain limits. See [the current plan](docs/architecture/native-server-storage.md).

> **November 3 setup decisions:** Manual fixed keys go through retained PATCH
> with shared Keychain saving/delivery. Authenticate runs recorded native setup
> commands; no reading prose and guessing commands. Exact command-entry/state-path
> and route outcomes remain to review. No code/testing resume.

> **November 3 latest decision:** Use the official Rust MCP library for shared
> online login/renewal, preserving native server-owned login and raw forwarding.
> Exact remaining Fetch work and open details are in [the updated plan](docs/architecture/native-server-storage.md).
> Further application edits/builds/tests remain paused at the user's request.

> **November 3 wrapper priority:** Use existing native server/helper behavior
> before adding Fetch code. Full lifecycle research recommends the official Rust
> auth library for shared hosted login/renewal; dependency adoption is proposed,
> not selected. Preserve Keychain and the common authenticate route. Source remains
> paused; [ownership and necessity](docs/architecture/native-server-storage.md).

> **November 3 shared login decision:** Both Authenticate and automatic renewal
> call Fetch's existing authenticate POST. That route owns the shared login/renewal
> work; the background job only decides when to call. Browser behavior and safe
> internal requests still need planning. Coding remains paused.

> **November 3 follow-up:** Keep PATCH /api/v1/servers/{server_id}/credentials
> in the plan for Keychain storage; explain what native servers already handle.
> The proposed manual-key-only scope and exact save/delivery behavior remain under
> discussion. Coding remains paused; see [the current plan](docs/architecture/native-server-storage.md).

> **November 3 latest checkpoint:** Keychain remote-client OAuth is selected.
> Further source edits/builds/tests are paused at the user's request for the full
> change plan. Partial source edits are incomplete and untested. Discuss any
> individual MCP/provider-specific code before writing it. See
> [the full route/change plan](docs/architecture/native-server-storage.md).

