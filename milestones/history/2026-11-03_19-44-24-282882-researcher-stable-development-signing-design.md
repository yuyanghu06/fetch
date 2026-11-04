# Select a safe stable development-signing flow

- Local timestamp / acting role / milestone: November 3, 2026 / dedicated Researcher; serialized by Orchestrator / keychain-prompts.
- Requested scope: Implement stable DEVELOPMENT identity and successful-key cache after latest pull.
- Starting repository state: d19d891 integrated with previous local work. Earlier Apple/source investigation retained.
- Work performed / changed files: Primary Apple signing guidance reviewed, with concrete Certificate Assistant and explicit build/sign/launch workflow. Orchestrator records design.
- Decisions: Dedicated self-signed Code Signing identity created through Certificate Assistant in login Keychain, no private-key export/import password files or global trust change. Fixed identifier com.fetch.local.development and exact public certificate fingerprint. Sign staging copy then atomic rename; do not modify live Cargo executable inode. Leave Cargo/build.rs/tests/release signing unchanged. Existing items may need initial approval for new stable identity.
- Checks and evidence: Apple TN2206/Code Signing Guide references in keychain-prompts.md. Signature identity across two executable versions must be verified; actual prompt persistence requires protected native evidence. No certificate/Keychain operation in this research.
- Resulting state / acceptance: Bounded operational design selected under user approval; no wrapping-key migration.
- Remaining work and handoff: Implement launcher, independent stub checks, prepare native Certificate Assistant; final credential creation and OS password remain human-owned.
