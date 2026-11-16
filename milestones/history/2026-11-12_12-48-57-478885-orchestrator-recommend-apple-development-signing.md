# Recommend development signing remedy

- Local timestamp / acting role / milestone: 2026-11-12T12:48:57.478885-05:00 / primary agent read-only research / keychain-prompts.
- Requested scope: explain how to stop recurrent Keychain passwords.
- Starting repository state: earlier UI/native/docs work retained; recurring native prompt issue unresolved.
- Work performed / changed files: Apple primary-source recheck, read-only security find-identity valid candidate count, current finding/journal.
- Decisions: User asks how to stop recurring Keychain password prompts. Read-only valid code-signing identity check finds no valid Apple Development/Mac Developer/Developer ID Application identity. Recommend an Apple-issued development identity for rebuild-related approvals: user creates it through Xcode account certificate management; then select its public fingerprint for the existing Fetch launcher and verify actual repeated launches. This is a recommendation, not an approved signing change or guaranteed fix for unchanged-binary prompts. Apple securityd source distinguishes qualifying team identities from per-build code hashes; prior self-signed approval assumption was insufficient. No certificate creation, private-key access, Keychain permission/storage change, app launch or rebuild.
- Checks and evidence: security find-identity -v -p codesigning, output filtered to candidate count only; Apple clientid.cpp and Xcode certificate documentation.
- Resulting state / acceptance: recommendation explained; no remedy implemented or runtime success claimed.
- Remaining work and handoff: user-owned Apple Development certificate creation if selected; approve signing-identity change before applying. Same-binary recurring prompt diagnosis remains unresolved.
