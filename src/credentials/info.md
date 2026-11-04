# Encrypted server credentials

Deletion targets only one server's encrypted file and exact Keychain service/account under write serialization. Retired IDs reject future writes; application/internal identities are excluded. Missing keys are absent, never newly created. Cleanup errors are truthful and retain the association when possible for explicit retry.

This module saves and reads server-local encrypted bytes using AES-256-GCM with fresh random nonces. The authenticated data binds the file format and exact server ID, so copying ciphertext between servers fails verification even if a test key is shared. Corrupt data and missing keys are not replaced. Secret documents clear their owned buffers on drop and have a redacted debug representation.

Keys come from an injected KeyProvider; production uses MacKeychainKeys and tests use disposable fixtures. There is no plaintext-key fallback. No HTTP endpoint exposes secret bytes. Successful encrypted write/readback proves storage only; the separate required authenticated provider test is still needed before any API can report login or credential-save success.

The production `MacKeychainKeys` provider uses per-server random keys in the traditional Mac Keychain. It limits each key to the running Fetch application and fails noninteractively when access is unavailable. Reading or replacing an existing encrypted file requires its existing key; a missing key is never silently replaced. App authorization and TLS identity use a separate encrypted application tree. No Keychain or provider operation was executed while writing this source.


## Shared helper storage channel

bridge.rs binds a private Unix socket/capability to one server/adapter owner and
process lease. load/update/verify operate only on the helper subsection of the
existing encrypted document, with revision comparisons and readback before ACK.
Runtime updates require the trusted saved-access verifier before ACK; initial
login may save refresh-only unverified state. Revocation permanently fences the
owner and waits for in-flight disk work; failed/closed processes lose their lease.
No request supplies a server/file destination or writes a manual credential.
bridge_client.mjs provides serialized private requests without retries/stdout.
Runner owns normal-process leases; Manager owns authentication contexts and their
trusted verifier. This channel is not a public API or another encryption store.


## Successful keys during one Fetch run

keychain.rs keeps successful per-server/purpose keys in Zeroizing memory, shared
by server and application stores within one data-root namespace. OS read/create/
delete remain behind the existing native bridge. Concurrent first reads share
one successful lookup. Creation requires direct OS readback before caching;
missing/denied/failed lookups are never cached. Deletion evicts its key first,
even on OS deletion failure. Quit clears and permanently closes the cache, so
retained tasks cannot refill it. Keys remain usable in memory until Quit; this
does not merge keys, change encrypted files or replace native Settings approval.
keychain_tests.rs uses a fake backend and never accesses the user's Keychain.
