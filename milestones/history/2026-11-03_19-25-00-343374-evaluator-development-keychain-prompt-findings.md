# Accept scoped Keychain prompt findings

- Local timestamp / acting role / milestone: November 3, 2026 / independent Evaluator; serialized by Orchestrator / keychain-prompts.
- Requested scope: Independently review source, signing metadata, likely cause and possible prompt reduction.
- Starting repository state: Read-only research/options documented; no Keychain/signing/source change for this task. Existing unrelated work preserved.
- Work performed / changed files: Evaluator read native bridge/key provider/startup and codesign metadata only. Orchestrator clarified research limitations and updated canonical docs/journal.
- Decisions: ACCEPT scoped findings. Ad-hoc designated requirement uses exact code hash; rebuilt identity plus repeated uncached reads explains likely renewed requests. Same label hides item distinctions. Stable signing with stable identifier plus successful-key process cache is recommended, not selected or implemented. A wrapping key is one possible larger reviewed redesign, not the only possible design.
- Checks and evidence: Source and Apple ACL/code identity/TN3137 references reviewed. Exact five events untraced. Encrypted-file count is not Keychain-item count; obsolete items uninspected. Existing UI-fail flag is no guarantee for traditional store. No new native prompt count or one-dialog result claimed.
- Resulting state / acceptance: Research task complete with uncertainty and tradeoffs explicit; documentation only. No credentials, signing identity, Keychain/trust settings or running app changed.
- Remaining work and handoff: Explain findings/options plainly; obtain a specific implementation choice before signing/cache/storage migration, then use protected native validation. Current code remains unchanged for this request.
