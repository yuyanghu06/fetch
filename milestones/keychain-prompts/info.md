## November 12 — Apple Development signing verified

The user created a fresh Apple Development certificate in Xcode. macOS lists it as a valid signing identity; its public certificate expires November 12, 2027. Selected its public fingerprint in the existing Fetch development settings file (no environment override). The existing ./scripts/run-development.sh --build-only succeeded in 25.71 seconds after waiting for another Cargo build. The resulting fetch-development executable passes strict signature verification, uses com.fetch.local.development, chains through Apple WWDR/Apple Root, and has TeamIdentifier 557D6QNLJV. Existing compiler/deprecation warnings remain. No Fetch launch, credential read/migration, Keychain access-rule/trust changes or release work occurred. Certificate/signing setup is complete; actual Keychain prompt behavior is still untested. Next: user explicitly Quits Fetch and runs ./target/debug/fetch-development app, approves any native prompts themselves, then Quits/repeats to observe behavior.

## November 12 — Apple Development setup now authorized

User authorizes proceeding with Apple Development signing for Fetch. Read-only checks find the existing Apple Development identity expired; Fetch still selects its self-signed local identity, with no environment override. A valid Apple Distribution identity also exists but is not selected for this development setup. Rust 1.94.1 meets Cargo.toml minimum 1.89; full Xcode, clang and codesign are available. Xcode was opened successfully. Next: user creates a fresh Apple Development certificate through Xcode Settings → Accounts/Apple Accounts → team → Manage Certificates → + → Apple Development. Then inspect its public identity, save the selected public fingerprint, and run the existing build-only launcher with strict signature verification. Apple login/password approvals remain user-owned. No signing selection, Keychain access rules/trust, saved credentials, app source or running Fetch was changed; no build or Fetch launch performed. Repeated-launch prompt behavior remains unverified.

# Fewer development Keychain requests

Goal: preserve macOS approval across development rebuilds and avoid repeatedly
requesting the same successful key, while keeping separate encrypted server keys.
User approved stable development signing and process caching after investigation.

Status: implemented. Latest remote work and local UI/startup/docs are preserved.
New cache keeps successful Zeroizing keys, direct creation readback, no failure
caching, delete eviction and closed/cleared Quit lifecycle. User-created dedicated
local signing identity is configured by public fingerprint only. Signed Fetch
build and two changed-binary identity proofs pass.30credential+1host+18UI tests,
4launcher fixtures and native build pass; independent source/final review ACCEPT.

[Evidence](evidence.md), [canonical design](../../docs/architecture/keychain-prompts.md).
All temporary fixture/probe state removed. The signer and its public settings file
are intended persistent development setup. Existing app/services/accounts preserved;
user should Quit Fetch then run ./scripts/run-development.sh. Old Keychain items
may need final human approval for the new identity.

Limits: actual native prompt counts and lock/denial/provider/client workflows are
unmeasured. No wrapping key, credential migration, release packaging or global trust
change. Password entry/credential creation remains human-owned.


First-launch documentation follow-up: AGENTS.md now directs agents to check each
Mac's tools, local signer/fingerprint and signed build readiness, and to give
concrete Certificate Assistant/tool installation steps when missing. It does not
provision a second Mac or authorize secret/Keychain migration.

## November 9 prompt follow-up

User reports three prompts per launch. Source confirms stable signing and successful-key caching are already implemented; exact prompting accesses untraced. Read-only review and current Apple guidance support user-owned Always Allow per Fetch item as the first step. A guaranteed single unlock/storage migration remains unselected. No Keychain item/access change or native reproduction was performed. See the current canonical design above.

## November 9 — Always Allow does not stick for the user

User correction: they already choose Always Allow and report repeated prompts on each development launch. Runtime acceptance remains unresolved. Exact-item permission metadata confirms saved Fetch approvals and per-build code-hash lists, with the newest build absent. An unchanged build-only rerun kept its same code hash/identity, so signing alone does not prove the unchanged-launch failure. Actual running-process inspection could not finish because Fetch exited; user-owned same-binary-twice check requested and pending. No secret reads, permission/storage changes or agent-run app/provider/MCP launch. Current canonical record and aggregate metadata supersede earlier expectations of remembered permission.

## November 11 — direct launch is already correct

User reports the Fetch encrypted local state dialog still appears on each direct ./target/debug/fetch-development app run. That command already starts the signed executable without building or re-signing; switching to scripts/run-development.sh invokes build/sign and does not fix access permission. Current on-disk executable passes strict signature verification, uses Fetch Development Local Signing and com.fetch.local.development, has no Apple TeamIdentifier, and was signed November11 at14:50:32. Multiple recent UI fixes rebuilt/replaced this executable, so the same path does not mean unchanged program bytes between those attempts. Existing metadata/source evidence supports build-specific approvals with the self-signed identity. Repeated prompts for a truly unchanged binary remain unresolved; the user's report is not dismissed as a wrong command or forgotten Always Allow. No new Keychain inspection, password/key read, permission/trust/storage change, rebuild or app launch.

## November 12 — recommended next step for Keychain prompts

User asks how to stop recurring Keychain password prompts. Read-only valid code-signing identity check finds no valid Apple Development/Mac Developer/Developer ID Application identity. Recommend an Apple-issued development identity for rebuild-related approvals: user creates it through Xcode account certificate management; then select its public fingerprint for the existing Fetch launcher and verify actual repeated launches. This is a recommendation, not an approved signing change or guaranteed fix for unchanged-binary prompts. Apple securityd source distinguishes qualifying team identities from per-build code hashes; prior self-signed approval assumption was insufficient. No certificate creation, private-key access, Keychain permission/storage change, app launch or rebuild.
