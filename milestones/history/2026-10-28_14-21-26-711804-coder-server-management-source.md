# Implement shared server-management source

- Local timestamp / acting role / milestone: 2026-10-28T14:21:26.711804-04:00 / coder / mcp-mvp-implementation.
- Requested scope: Full selected backend management source through testing handoff, no testing.
- Starting repository state: 9fc4eee runner/native gateway source; authority and installer source previously pushed.
- Work performed / changed files: Add Manager shared handlers for safe/scoped reads, atomic config, native credential saves, authenticated provider verification, Linear login/owned refresh, named normal/force restart, startup/shutdown and durable tool observations. Existing unknown/unsupported branches remain truthful; no new management controls. Source is integrated with existing storage/runner types.
- Decisions: Restrict non-secret imported endpoint paths to documented supported profiles; other layouts report unsupported. Unsupported/unreferenced/multi-entry/removal-only manual saves cannot claim a successful new-credential verification.
- Checks and evidence: Source reading and specialist reports only. No build/test/check/format/lint/validation/service/OS/package/provider/client execution.
- Resulting state / acceptance: Reviewed source ready for integration; first compilation and all runtime acceptance remain user-controlled.
- Remaining work and handoff: HTTP/native UI/application wiring, packaging source and explicit verification handoff.
