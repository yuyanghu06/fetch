# Development identity and key reuse — November 3, 2026

The user approved the recommended stable development identity and process key
cache, and requested pulling latest main. Shared wrapping keys, encrypted-file
migration, release signing and packaging were not selected.

## Implemented behavior

MacKeychainKeys now reuses successful individual server/purpose keys within its
fixed data-root namespace. Buffers are Zeroizing; concurrent first reads share
one successful native lookup. Missing/denied/failed lookups are not cached. New
keys still require direct OS readback and comparison before use. Deletion evicts
its selected key even if the native deletion fails. Quit clears and closes the
provider after manager shutdown; retained tasks cannot refill it. File encryption,
native Settings confirmation and stored-credential verification stay unchanged.

The development launcher builds normally, signs/verifies a staging copy with
fixed identifier com.fetch.local.development and an exact configured certificate
fingerprint, then atomically publishes target/debug/fetch-development. Build-only
stops before launching. Failure preserves the prior signed executable and never
falls back to ad-hoc signing. Cargo's original binary, global config, release
scripts and existing running Fetch are untouched.

## Actual identity setup and verification

Certificate Assistant was prepared for Fetch Development Local Signing, Self
Signed Root, Code Signing, with login Keychain selected. The user completed its
final Create action and confirmed Done. Native password/approval remained human
owned. Only the new identity's public fingerprint was saved at the documented
Fetch-Development settings path; no private key was exported, no password saved,
and no certificate trust/other Keychain access list was changed by the agent.

Actual signed build-only launch preparation passed. The resulting Fetch binary
has identifier com.fetch.local.development and authority Fetch Development Local
Signing. Two different tiny C executables were then signed with that same task
identity/identifier. Their code hashes differ but their designated requirements
match exactly. Both strict verification and cross-requirement checks passed.
The first probe omitted codesign's required '=' prefix for an inline test
requirement; the corrected probe passed without changing product code. Both
probe roots were removed; no probe binary was executed.

This proves stable signing requirements across changed code, not the actual
number of login-Keychain dialogs after rebuilding Fetch. Existing items may need
one final human approval for the new identity. An already unlocked cached key
remains usable until Quit, even if Keychain is locked externally. Returned key
copies cannot be retroactively erased by clearing the provider.

## Independent checks

- Credential suite:30passed (22existing+8new fake-backend cases), exit0;26.77s
  compilation,30.04s tests including an existing deliberate30-second timeout.
- Retained ApplicationHost shutdown integration:1new case passed;6.09s compile,
  0.01s test. This brings new cache coverage to9cases.
- Pulled UI/auth/discovery integration:18passed, exit0;5.62s compile/0.01s tests.
  Initial17/18 found an old fixture expecting Select all to be idempotent. The
  preserved user-selected Unselect-all toggle correctly clears selected rows;
  the amended fixture now checks clear then reselect, with installed rows excluded.
- Launcher fixture checks:4passed using harmless fake Cargo/codesign processes
  in a disposable copy. Build/sign/verification failures preserve output and
  clean staging; build-only success never launches. This is control-flow proof,
  separate from the real signature proof above.
- Native binary compilation passed18.42s; signed build-only preparation passed
  with a0.45s up-to-date Cargo build. The exact normal launcher command also
  passed with the default Xcode toolchain (9.20s rebuild), and Fetch retained its
  same signing requirement afterward. Existing native/MCP warnings remain.
- Independent cross-source reviews and final scoped evidence evaluation ACCEPT.

Source ownership was exclusive. Test fixtures never called real Keychain APIs;
no native Fetch, provider or MCP was started. Only the dedicated user-created
signing key was used for authorized signing. The launcher fixture root and both
signature-probe roots were removed; no task process remains.

## Pull integration and limits

ff-only pull initially refused divergence. Local helper commit3023750 and remote
d19d891 had identical helper source; duplicate local commit was skipped during
rebase. Autostashed notes/UI/startup work was restored alongside new automatic
folder discovery. Additive docs conflicts preserved both sides; the selection
toggle stayed. Tests cover the merged UI behavior. A recovery autostash remained
until final verification; it is task-created and may be removed after delivery.

Existing everyday app/services and encrypted credential files were preserved.
Use ./scripts/run-development.sh after explicitly quitting the old Fetch process.
Actual native prompt count, real Keychain lock/denial, and provider/client
acceptance remain separate protected native checks. No one-dialog result is claimed.

## Retained artifacts

- [Test/build results](verification.json)
- [Launcher control-flow results](launcher-fixture-results.json)
- [Real signature identity proof](signature-evidence.json)
