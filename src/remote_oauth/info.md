## November 4 private auth error details

sdk.rs attaches the shared startup capture to the existing HTTP/library path. Actual response status and failed body plus error kind/message reach the native console after sanitization; successful bodies and request payloads are never logged. Request-time nofollow/destination/size rules and library issuer validation remain; their failure reason is visible. Token/Keychain ownership and refresh behavior are unchanged.

# Shared Keychain access and standard remote OAuth

This module stores Fetch's remote HTTP MCP client grants and explicitly entered
fixed keys directly in macOS Keychain. It does not create provider encryption keys
or encrypted provider files. Native server/helper OAuth remains with that server.
The separate application OAuth/TLS store is unchanged.

`GrantStore` names items by the selected Fetch root and server instance. Native
Keychain operations run on blocking workers. Writes require exact readback; errors
never fall back to plaintext or migrate legacy data. The memory/backend interfaces
are explicit disposable-test seams.

The document has independent manual and SDK sections. Section updates preserve
the other section. Manual values are delivered only through assigned saved
references and never returned by public APIs. Removal of a fixed key updates only
that manual section after native Settings authority; there is no automatic item
cleanup during server removal.

`sdk.rs` connects the official Rust MCP library to the store. The library handles
standard login/refresh/token fields. The store binds its saved SDK credentials to
the exact instance endpoint, saved issuer and OAuth client identity,
and supplies a shared per-instance refresh lock. SDK metadata mismatch cannot
silently delete account state. Failed refresh preserves the previous stored grant;
the library distinguishes transient failure from rejected login. There is no
custom persisted refresh state machine and no provider-specific adapter.

Fetch keeps browser/callback presentation, deadlines, caller authority, scheduling
and applicable protected-access checks. The raw MCP runner remains separate from
SDK protocol models. Real Keychain/provider/Finder acceptance is not established
by fake backend fixtures; actual results belong in milestone evidence.


## November 4 — Library-owned compatibility decisions

Preparation passes rmcp's resolved metadata into its session without Fetch requiring discovered-only metadata, mandatory issuer/support flags, or upfront validation of every unused endpoint. The library decides discovery fallback and callback validation. Manager forwards optional callback issuer to rmcp; no library checks are disabled. Harmless metadata changes no longer reject saved login, while endpoint/issuer/client identity checks remain. Saved records keep their existing shape and explicit no-silent-deletion behavior. 43 independent fake-HTTP/Keychain/route checks and final locked native build pass; final Evaluator ACCEPT. No real-provider sign-in claimed. Evidence is tracked in milestones/wrapper-authentication/.

## November 4 — Public Fetch client identity

`sdk.rs` supplies `https://www.yuyang-hu.org/oauth-client.json` as Fetch's public
client identity document to rmcp. The library chooses whether the service supports
this document or uses dynamic registration; Fetch adds no separate probe or gate.
The document describes Fetch, not the user's account, and contains no secrets.
Browser sign-in callbacks still return to the local Fetch callback address.
Changing the document's domain or URL later changes Fetch's OAuth identity and may
require a fresh sign-in. The canonical `www` address is intentional because the
apex domain redirects. Implementation is written; focused verification and site
publication evidence are recorded separately by the orchestrator.
