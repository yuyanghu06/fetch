## November 12 — Apple Development signing verified

The user created a fresh Apple Development certificate in Xcode. macOS lists it as a valid signing identity; its public certificate expires November 12, 2027. Selected its public fingerprint in the existing Fetch development settings file (no environment override). The existing ./scripts/run-development.sh --build-only succeeded in 25.71 seconds after waiting for another Cargo build. The resulting fetch-development executable passes strict signature verification, uses com.fetch.local.development, chains through Apple WWDR/Apple Root, and has TeamIdentifier 557D6QNLJV. Existing compiler/deprecation warnings remain. No Fetch launch, credential read/migration, Keychain access-rule/trust changes or release work occurred. Certificate/signing setup is complete; actual Keychain prompt behavior is still untested. Next: user explicitly Quits Fetch and runs ./target/debug/fetch-development app, approves any native prompts themselves, then Quits/repeats to observe behavior.

## November 12 — Apple Development setup now authorized

User authorizes proceeding with Apple Development signing for Fetch. Read-only checks find the existing Apple Development identity expired; Fetch still selects its self-signed local identity, with no environment override. A valid Apple Distribution identity also exists but is not selected for this development setup. Rust 1.94.1 meets Cargo.toml minimum 1.89; full Xcode, clang and codesign are available. Xcode was opened successfully. Next: user creates a fresh Apple Development certificate through Xcode Settings → Accounts/Apple Accounts → team → Manage Certificates → + → Apple Development. Then inspect its public identity, save the selected public fingerprint, and run the existing build-only launcher with strict signature verification. Apple login/password approvals remain user-owned. No signing selection, Keychain access rules/trust, saved credentials, app source or running Fetch was changed; no build or Fetch launch performed. Repeated-launch prompt behavior remains unverified.

## November 12 — startup fails without a prompt

User explicitly reports no Keychain dialog before protected HTTPS identity failure. Existing error reporting hid native status, read/decryption and format failure alike. Safe OSStatus/stage propagation is implemented and isolated tests pass; real cause remains unknown until a fresh failure is observed. Directory metadata was private/owned/non-symlink, but no real secret bytes or Keychain keys were read. No access-list, certificate/trust, signing-identity or saved-state changes. UI code is signed with the existing local signer; this is not a native access fix. See milestones/https-identity-startup-error/info.md.

## November 12 — recommended next step for Keychain prompts

User asks how to stop recurring Keychain password prompts. Read-only valid code-signing identity check finds no valid Apple Development/Mac Developer/Developer ID Application identity. Recommend an Apple-issued development identity for rebuild-related approvals: user creates it through Xcode account certificate management; then select its public fingerprint for the existing Fetch launcher and verify actual repeated launches. This is a recommendation, not an approved signing change or guaranteed fix for unchanged-binary prompts. Apple securityd source distinguishes qualifying team identities from per-build code hashes; prior self-signed approval assumption was insufficient. No certificate creation, private-key access, Keychain permission/storage change, app launch or rebuild. Sources: [Apple securityd](https://github.com/apple-oss-distributions/Security/blob/main/securityd/src/clientid.cpp), [Xcode certificates](https://help.apple.com/xcode/mac/current/en.lproj/dev154b28f09.html).

## November 11 — direct launch is already correct

User reports the Fetch encrypted local state dialog still appears on each direct ./target/debug/fetch-development app run. That command already starts the signed executable without building or re-signing; switching to scripts/run-development.sh invokes build/sign and does not fix access permission. Current on-disk executable passes strict signature verification, uses Fetch Development Local Signing and com.fetch.local.development, has no Apple TeamIdentifier, and was signed November11 at14:50:32. Multiple recent UI fixes rebuilt/replaced this executable, so the same path does not mean unchanged program bytes between those attempts. Existing metadata/source evidence supports build-specific approvals with the self-signed identity. Repeated prompts for a truly unchanged binary remain unresolved; the user's report is not dismissed as a wrong command or forgotten Always Allow. No new Keychain inspection, password/key read, permission/trust/storage change, rebuild or app launch.

Read scripts/run-development.sh and current native key access code; codesign -dv --verbose=4 and codesign --verify --strict on target/debug/fetch-development. Rechecked Apple securityd clientid.cpp partitionIdForProcess: qualifying Apple developer identities use teamid; other signed code uses cdhash. Primary-source context, not native reproduction. [Apple source](https://github.com/apple-oss-distributions/Security/blob/main/securityd/src/clientid.cpp).

## November 9 — correction: Always Allow already selected; issue unresolved

The user explicitly says Always Allow is already selected and prompts return every time they run the development launcher, including what they describe as launches without rebuilding. Do not repeat the earlier button-choice advice or claim source signing/cache proves the behavior works.

Read-only exact-item metadata confirms both Fetch internal keys already include fetch-development in the approved application list. The TLS key has 21 partition entries; app-authorization has eight. Every entry approves an individual code hash; the newest signed executable's hash is absent from both lists. Decrypt permission flags are zero. This confirms approvals exist and shows a build-specific approval boundary; it does not identify the third prompt or prove every identical-binary restart fails. The app exited before dynamic process-identity lookup, so running-vs-disk identity comparison was not completed.

The on-disk executable has the selected local certificate, com.fetch.local.development, no TeamIdentifier, and passes strict verification. Repeating run-development.sh --build-only with unchanged source succeeded with identical code hash and designated requirement; only signing time changed. Therefore automatic re-signing by itself is not established as the cause of prompts on an unchanged launch. No source/authentication/storage/access-list changes were made. One locally available Apple-issued development signing identity is expired; do not select it or change signing settings automatically.

[Apple's securityd identity source](https://github.com/apple-oss-distributions/Security/blob/main/securityd/src/clientid.cpp) chooses a team identifier for qualifying Apple-issued developer certificates and a code hash for other signed code. This supports the observed self-signed per-build permissions and corrects the earlier assumption that a stable designated requirement alone guaranteed Keychain approval across rebuilds. The public Apple source is research context, not proof of this Mac's exact securityd build. [Apple's Keychain tracking table](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/AboutCS/AboutCS.html) also distinguishes custom access rules. No code was copied; Apple source is APSL2.0.

Next controlled check is user-owned because it involves the actual app and native password dialogs: run ./target/debug/fetch-development app, approve, explicitly Quit, run the same command again without edits/rebuilds. Observe whether the second launch prompts and which labels appear. This was requested asynchronously and is pending. Agent probes disabled UI in their own process, requested only exact item references/permission rules, restored UI settings and removed all task-created probe sources/executables. No saved key/token/password bytes were requested, no Keychain changes made, and no actual Fetch/provider/MCP launched. Aggregate evidence: milestones/keychain-prompts/repeated-launch-metadata.json and unchanged-launcher-signature-check.json.

## November 9 — three prompts reported on launch

The user reports entering three Keychain prompts per launch, with one pictured item labelled Fetch encrypted local state. This is a user-reported count, not a traced list of item reads. Read-only source review confirms one shared MacKeychainKeys provider per host, separate TLS-identity/app-authorization encryption keys, and successful-key caching until Quit. Remote MCP logins/manual keys use separate direct-Keychain items. Stable development signing is configured and used by the signed launcher. These facts do not establish which three accesses prompted or whether Always Allow was selected.

Apple's [Allow apps to access your keychain](https://support.apple.com/en-gb/guide/mac-help/kychn002/26/mac/26) and [If you’re asked for access](https://support.apple.com/en-euro/guide/keychain-access/kyca1243/mac) distinguish this-time access from Always Allow, which remembers permission for the app/item. Recommend the user choose Always Allow themselves for Fetch items if they want remembered permission. Several separate items may need approval initially; do not promise exactly one approval or zero future prompts with locked/changed identities. Entering a password and pressing ordinary Allow does not grant persistent access.

A guaranteed single unlock remains a storage-design/migration decision, not a bulk-read switch. No shared-key consolidation, access-rule change, password entry, credential read/migration or new authentication source work is authorized by this question. Existing signing/cache implementation is retained. If prompts recur after Always Allow with the signed launcher, next investigate actual public signing identity and which nonsecret item identifiers request access; do not guess three distinct keys from the count.

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
