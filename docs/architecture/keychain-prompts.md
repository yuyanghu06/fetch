# Development Keychain prompts — November 3, 2026

Status: user subsequently approved implementation of the recommended stable
development identity and successful-key process cache, and requested latest main
be pulled. Shared wrapping-key/storage migration remains unselected. The initial
read-only investigation below is retained as evidence.

Original investigation: User reported approximately
five identical macOS login-Keychain access dialogs for the development build and
asked whether access can be combined. No signing, Keychain/trust, encryption,
application-source or personal-state changes were made for this investigation.
The later user instruction authorizes development signing/cache work only;
release packaging/signing and credential migration remain excluded.

## Current scope after the server-storage refactor

The successful-key process cache now applies only to Fetch's own TLS identity and
coding-app authorization keys. Old provider files/keys remain untouched; provider
key reads/writes and deletion APIs are removed. New manually entered keys and
online-client grants use direct Keychain storage, while native servers keep their
own files. No new Keychain prompt-count result is claimed for this refactor.
[Current storage contract](native-server-storage.md).

## Earlier implementation after user approval

Stable development signing and successful-key caching are implemented and tested.
Use scripts/run-development.sh for normal development; plain cargo run still
uses Cargo's linker-signed executable. The user created the dedicated local Code
Signing identity in Certificate Assistant; only its public fingerprint is stored
in development settings. Private keys remain in login Keychain; global trust and
existing item permissions were not changed. Signature requirements match across
two changed binaries; actual signed Fetch preparation succeeds.

Successful per-ID keys now remain in zeroizing memory until Quit, with direct OS
creation readback, failure exclusion, delete eviction and permanently closed
shutdown. Separate server/purpose keys and encrypted file formats stay unchanged.
Independent tests:30credential+1host+18UI cases,4launcher fixtures; native build and
cross-source/final evaluations pass. Actual prompt counts across rebuilt native
sessions remain unmeasured. See [implementation evidence](../../milestones/keychain-prompts/evidence.md).

## Original observations and source before implementation


- The screenshot shows a traditional login-Keychain item-access dialog for the
  label Fetch encrypted local state; it is not the native Settings confirmation.
- Read-only codesign metadata on target/debug/fetch reported Signature=adhoc,
  linker-signed and no TeamIdentifier. The exact binary identity was inspected,
  not any signing private key or installed certificate inventory.
- Metadata-only checks under the current development root found one application
  encrypted record and one server encrypted record. No encrypted bytes, tokens,
  key values or credential/config contents were read. This is not a Keychain
  item inventory; obsolete/orphan items were not inspected. Five dialogs therefore
  cannot be equated with five distinct current credential files.
- src/native_auth/bridge.m creates every key with the same label. It creates
  per-item trusted-application access using the executable creating the item.
- MacKeychainKeys in src/credentials/mod.rs has no key cache. Every encrypted-file
  read requests its existing Keychain key; save and encrypted readback request
  keys again. Application startup loads its HTTPS identity and app authorization
  separately, then manager startup reads required server secrets across refresh,
  routing, connection and renewal setup. Missing encrypted files return without
  requesting keys. Exact lookups vary by stored state and selected flow.

These facts strongly support repeated access to a small number of keys plus
rebuild-related identity reapproval. The exact five dialogs were not traced or
reproduced; do not claim five distinct keys or a measured prompt-count reduction.

## Apple guidance

[Code-signing requirements](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/RequirementLang/RequirementLang.html)
explain that ad-hoc signing identifies only that exact program.
[Code identity and Keychain](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/AboutCS/AboutCS.html)
explain that the creating application's stable designated requirement controls
Keychain recognition. Self-signed development identities can provide stable
identity without establishing public distribution/Gatekeeper trust. This is not
a reason to grant all applications access or change system certificate trust.

[SecItemCopyMatching](https://developer.apple.com/documentation/security/secitemcopymatching(_:_:))
disallows combining password data return and match-all for generic passwords,
because individual items may need their own authentication. A bulk lookup cannot
promise one OS approval across these independent items.

[TN3137](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains)
distinguishes traditional file-based and data-protection stores. Current code sets
kSecUseAuthenticationUIFail, but the screenshot still reports legacy item-access
UI. The installed Apple SecItem.h explicitly warns that the older no-auth-UI
option applies to data-protection items and does not suppress legacy-item UI.
Treat background-prompt prevention as unresolved native behavior; do not assume
an LAContext or a current SecItem flag alone solves traditional ACL dialogs.
The legacy [interaction-control API](https://developer.apple.com/documentation/security/seckeychainsetuserinteractionallowed(_:))
needs isolated validation, including setting scope/concurrency/restoration and
truthful denied/locked behavior. It must not become an automatic retry loop,
plaintext fallback, or silent success when protected storage is unavailable.

## Original options (1 and 2 subsequently selected; 3 unselected)

1. **Recommended first: stable development signing.** Give successive local
   builds the same protected developer identity and explicit app identifier.
   Preserve per-server keys and all encrypted files. Existing items may require
   a final approval for the new identity; ordinary rebuilt launches should then
   be recognized. This requires a deliberately selected development identity,
   not ad-hoc codesign again, and isolated native proof. No identity was created,
   selected from personal Keychain or used in this investigation.
2. **Remember successfully authorized keys during this Fetch process.** Avoid
   requesting the same key repeatedly. Keep server/purpose isolation, verify
   newly created keys through actual OS readback before caching, evict on delete,
   never cache missing/denied as success, and clear owned memory on Quit. Keys
   remain usable in memory longer; Keychain lock does not revoke already cached
   copies. This reduces duplicate accesses, not the number of distinct items.
3. **One Keychain-protected wrapping key.** Keep separate server data keys but
   protect their storage using one Fetch unlock key. Normal startup would need
   one Keychain item; loss/compromise of that key affects all wrapped keys. This
   changes storage, migration, failure/recovery and memory-lifetime decisions.
   Access to existing separate items may still require approvals during initial
   migration. It is a major unapproved design change, not a small dialog fix.

Prefer stable development identity, then consider per-process caching. Neither
promises exactly one dialog when adopting the change with existing items. A
literal single-item unlock model requires an explicitly reviewed storage
redesign, such as option3, with a complete migration/recovery decision. Other
consolidated-item or different-store designs also need separate review. Switching to the provisioned data-protection store
is not selected by this investigation either.

## Existing implementation comparisons

Research reused the already-inspected maintained
[apple-native-keyring-store](https://github.com/open-source-cooperative/apple-native-keyring-store)
(MIT/Apache-2.0, security-framework dependency; researcher rechecked activity
November3). It separates file-based and protected stores. Its
[shared-authentication issue25](https://github.com/open-source-cooperative/apple-native-keyring-store/issues/25)
concerns protected-store biometric prompts, not evidence for a traditional-store
fix here. Its [error issue43](https://github.com/open-source-cooperative/apple-native-keyring-store/issues/43)
reinforces preserving denied/unavailable distinctions. No source copied or
new library selected. No native prompt/Keychain/certificate experiment was run.

Independent Evaluator ACCEPTed the scoped source/research explanation, while
confirming exact dialog events are untraced and no one-prompt result is measured.
