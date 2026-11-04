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
the exact instance endpoint, published metadata, issuer and OAuth client identity,
and supplies a shared per-instance refresh lock. SDK metadata mismatch cannot
silently delete account state. Failed refresh preserves the previous stored grant;
the library distinguishes transient failure from rejected login. There is no
custom persisted refresh state machine and no provider-specific adapter.

Fetch keeps browser/callback presentation, deadlines, caller authority, scheduling
and applicable protected-access checks. The raw MCP runner remains separate from
SDK protocol models. Real Keychain/provider/Finder acceptance is not established
by fake backend fixtures; actual results belong in milestone evidence.
