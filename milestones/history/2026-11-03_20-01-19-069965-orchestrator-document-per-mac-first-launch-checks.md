# Document first-launch readiness on each Mac

- Local timestamp / acting role / milestone: November 3, 2026 / Orchestrator / keychain-prompts.
- Requested scope: User asks project AGENTS.md to tell other agents what to check before first launch and how to guide the user when setup is missing; user resumed after an interruption before any edit.
- Starting repository state: Clean working tree; signed development launcher/cache already delivered. No source change or runtime action needed.
- Work performed / changed files: Added first-launch checklist at top of AGENTS.md; linked DEVELOPMENT.md; dated NOTES entry; refreshed root/testing/milestone guides, context/snapshot/to-do and journal. Actual signing script, Cargo manifest and existing setup docs were read.
- Decisions: Per-Mac tool and signer readiness, exact environment/file precedence, public fingerprint only, local matching certificate/private key, human Certificate Assistant creation and OS confirmation, authorized build-only verification and signed launcher. Missing setup must produce concrete user steps. No automatic installs/trust changes/private-key or encrypted-state migration.
- Checks and evidence: Scoped whitespace check and referenced-file/launcher-setting inspection pass. Installed security help confirms valid-only identity filtering is optional; checklist does not demand global trust for self-signed development. No tests, app, Keychain or signing operation was run.
- Resulting state / acceptance: Requested instructions written; independent read-only review pending. Each Mac's identity/logins remain local, with source/checklist reusable.
- Remaining work and handoff: Inspect independent review, commit/push documentation and report AGENTS.md destination. Existing native prompt-count acceptance remains unchanged.
