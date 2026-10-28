# Verify resource prompt contract and required read framing

- Local timestamp / acting role / milestone: 2026-10-28T00:37:44.300024-04:00 / researcher / mcp-mvp-architecture
- Requested scope: Independently verify URI/rawpayload/completion/Task boundaries and actual schema requirements.
- Starting repository state: Full proposed native contract drafted from completed research/plan; all new clauses await input.
- Work performed / changed files: Read-only verifier found missing modern resource-read frame/cache fields; checked currentCacheableResult/ReadResourceResult/GetPromptResult/CompleteResult. Root explicitly proposed required native readframing/private0 hints while preserving data.
- Decisions: ReadResource requires ttlMs/cacheScope/resultType; promptget/completion do not require cache hints. No invalidmarker overwrite, InputRequiredcompletionguess or ToolResultRchange.
- Checks and evidence: Source/schema facts confirmed and fix incorporated for final evaluator. No files changed by verifier or live tests.
- Resulting state / acceptance: One material documentary gap fixed by root; remaining URI/prompt/deferral boundaries match research.
- Remaining work and handoff: Final evaluator checks corrected contract plus synchronizedrecords before user presentation.
