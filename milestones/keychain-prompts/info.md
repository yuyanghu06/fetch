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
