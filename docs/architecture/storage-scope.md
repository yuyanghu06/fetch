# Initial MVP storage scope

October 26, 2026. Latest user decision: **sideline retention; the initial MVP has no PostgreSQL functionality**. This supersedes the earlier requirement to provision/connect PostgreSQL for the initial MVP. PostgreSQL schema/migrations, database-host Tailscale setup and PostgreSQL-backed result retention are deferred, not implementation prerequisites.

The user subsequently selected **local files for server configurations and the existing encrypted store for all Fetch-managed tokens/credentials**. D26 subsequently specifies each server’s own encrypted credential configuration files. Non-secret launch/package/path configuration may contain references; actual secret values belong in those encrypted files. Backend APIs own reads/writes; UI/agents keep using the APIs. D28 places durable per-server configuration/credentials under `~/Library/Application Support/Fetch/servers/<server-id>/`, outside replaceable `packages/servers/` installs. File encoding/names, safe-write coordination and encryption-key protection remain open. Harness clients still retain their own Fetch login credential as previously agreed.

No application source has been implemented. There are no actual Fetch database dependencies to remove; this is a change to the plan. Do not install a substitute database or silently choose a specific file encoding/crypto implementation. A requirement to save data is not automatically a requirement for PostgreSQL or any database server.

## Planned data and whether it needs PostgreSQL

| Feature/data | What must be kept | Dependency at this checkpoint |
| --- | --- | --- |
| Installed-server definitions | Stable server IDs, original source/launch settings, paths, package/version selections and credential references; enough to work after original harness entries are removed. | Local configuration files selected. Include non-secret server/launch/package metadata and credential references; D28 selects the separate durable server-data directory; encoding/filenames still need review. |
| API keys, environment entries and provider OAuth | Per-server values, newly issued tokens, refresh/expiry information where supported. | Per-server encrypted credential configuration files selected (D26); ordinary non-secret records carry references. PostgreSQL is unnecessary; D28 selects outside-package placement; exact filenames/format/key protection/integration remain open. |
| Harness-to-Fetch OAuth | Client/grant/login/renewal state as required by the selected flow. | Fetch-held secret token/key material belongs in the encrypted store. OAuth flow and non-secret client/grant metadata representation remain open; no PostgreSQL required. |
| Installation/restart progress | Running operation IDs, stages and terminal result needed by callers currently monitoring. | Backend live state is needed. D36 guarantees installation-job/terminal-result lookup throughout the current Fetch run, including UI closure; no cross-Quit recovery promise. Restart-attempt lookup lifetime and concrete state/storage mechanisms remain open; long-term retention/PostgreSQL recovery stay deferred. Deferral does not remove the accepted `202` plus status API. |
| Shared-memory prompt/history/undo | Current preference prompt and revisions if that capability is included in this release. | D73 explicitly defers shared memory from this first release. Preserve future prompt/history/undo direction; no memory storage/schema choice blocks the MCP build. |
| Discovery, package preparation, wrapper processes and tool routing | Config-file reads, private package files, live process/call information and upstream tool discovery. | No inherent database-server dependency. Saved server definitions and authentication are still needed for reopening the app, but process control/tool forwarding themselves do not require PostgreSQL. |

## Approved server data placement — D28

On October 27 the user accepted the outside-package recommendation with "Sounds good, let’s do next steps." Keep durable per-server configuration and encrypted credential files in `~/Library/Application Support/Fetch/servers/<server-id>/`, separate from replaceable software in `packages/servers/`. Package updates, reinstalls and dependency cleanup must preserve the separate saved configuration/credentials. Removal/uninstall retention still needs its own decision.

```text
~/Library/Application Support/Fetch/
  packages/
    runtimes/...
    servers/<server-installation>/...  # Installed software/dependencies
  servers/
    <server-id>/
      config                          # Non-secret settings/references
      credentials.enc                 # This server's encrypted credential configuration
```

The directory split is approved. `config` and `credentials.enc` illustrate separate non-secret and encrypted files; exact filenames, encoding, stable-ID representation and encryption/key protection remain open. The package installation identifier need not equal the stable server ID; identity/version mapping belongs to step 5. No runtime directories or credentials have been created.

This is Fetch's engineering choice, not an MCP/Apple-prescribed internal layout. [Application Support](https://developer.apple.com/documentation/foundation/url/applicationsupportdirectory) is Apple's normal app-support location. The separation keeps secret retention independent of replacing downloaded software. It does not approve the GitHub CLI credential mechanism or any plaintext fallback.

## Consequences for the restart contract

- The accepted restart path/body, `202` response, restart ID, monitoring query and result/loading behavior remain unchanged.
- Remove the proposed 24-hour PostgreSQL result-retention rule from the MVP contract. Long-term job history, expiry policy and database-backed recovery are deferred. This does not select memory-only storage or discard all operation records.
- Do not promise that historical attempt IDs survive Quit/crash until the remaining local-state/lifetime behavior is decided. Quit still stops managed servers. Current-operation progress must remain available as needed for the selected polling flow.
- On October 27 the user separately approved the remaining restart policies, including deadlines, overlap, permissions, readiness and error/envelope rules (D20). That approval preserves this storage deferral and does not select local operation-state lifetime or cross-launch recovery.

## Provider-token ownership — October 27 requirement

D24 requires actual server authentication/storage to be defined before automatic integration support is claimed. All **Fetch-managed** secrets retain the selected encrypted per-server store. A wrapper's own OAuth memory/cache is not automatically readable by Fetch or protected by that store; no plaintext-cache exception, token scraping or blanket token-export capability is approved.

For wrapper-owned login, review a compatible storage/provisioning interface and document whether login survives restart and who refreshes it. A documented memory-only login does not establish persistent credentials. A separately selected supported manual/provider integration may supply assigned values through its proper process interface; this is not credential passthrough through ordinary MCP responses. [The API requirements](api-contract.md#proposed-provider-authentication-and-credential-storage-contract) and [research](research.md#actual-mcp-auth-and-token-ownership--rechecked-october-27) describe the distinction. D25 additionally requires a successful encrypted-store write/readback and `2xx` service-authenticated test using that stored credential before reporting credential-based auth success. The provider callback’s in-memory token and a public health check do not meet it. Exact hooks, test endpoints, recheck/cleanup policy, exact filenames, crypto/key protection and wrapper-owned-store compatibility remain open. No plaintext-cache exception is selected. PostgreSQL/retention remain deferred.

## Future database work

The earlier local PostgreSQL/Tailscale and common migrations across local/Supabase/cloud PostgreSQL remain historical future guidance if database features are reintroduced. They are not initial-MVP setup or acceptance requirements. Normal source Git remains required.

This concerns **Fetch's own storage**. It does not disable an imported MCP merely because that MCP calls a database-backed external service such as Supabase. Do not claim any such integration has been implemented or tested.

See [decisions.md](decisions.md), [build-readiness.md](build-readiness.md), [restart-contract.md](restart-contract.md) and the updated [handoff](../../milestones/mcp-mvp-architecture/handoff.md). Application implementation remains paused.

D36 selects installation-status availability through the current Fetch run only. It does not choose RAM/file/database storage, a history UI, restart-attempt lifetime, or recovery after Quit/crash/relaunch.

D71 now selects the presented app connection/token persistence and lifetime behavior across ordinary Quit/reopen, while incomplete approvals end on Quit. This selects no encryption/key/file mechanism; that concrete review follows. Fetch-owned connection tokens and TLS identity stay distinct from each upstream server's provider credentials.

The next [Settings/key protection proposal](secret-storage-settings.md) now reviews concrete login-Keychain per-server keys, config.json/credentials.enc names, authenticated encryption, native Settings authority and unlock/recovery. These mechanisms are proposed; D26/D28 alone did not select them. Exact app-owned OAuth/TLS layout, serialization and code/library identity remain explicit implementation dependencies.

D72 now selects Keychain-held keys for encrypted per-server files and the shown native Settings/Mac-confirmation boundary. It does not yet select classic versus modern Keychain, exact algorithm/filenames/envelope or recovery/header/bootstrap/signing mechanics from the unshown draft. Those remain bounded implementation candidates, not grounds to re-review encrypted storage.

The next [credential verification amendment](credential-verification.md) proposes retaining newly committed encrypted values after a failed provider check, truthful persistence metadata, and a narrow storage-only success exception for removal/known non-auth/empty batches. D72 does not approve these unseen failure/retention/exception policies; D25/D31 remain unchanged until the user answers.

D73 current scope: shared memory is deferred from this release, and the unaccepted verification/storage-only exception draft is withdrawn from active review. Existing approved file/Keychain direction stays fixed; all exact implementation choices are gathered once in build-readiness.md.
