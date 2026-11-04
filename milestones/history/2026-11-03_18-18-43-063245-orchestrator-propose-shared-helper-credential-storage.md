# Propose reusable helper credential storage

- Local timestamp / acting role / milestone: 2026-11-03T18:18:43.063245-05:00 / Orchestrator / shared-mcp-authentication.
- Requested scope: Explain whether the missing callback-to-encrypted-storage work can be reusable across servers.
- Starting repository state: Shared router/direct install body delivered; Google helper integration deferred by user choice. Concurrent UI/deletion investigation and documentation changes preserved.
- Work performed / changed files: Reused completed Google/helper research and current authentication records; documented bounded shared bridge proposal in architecture, NOTES, milestone snapshot/to-do/guide and architecture guide. No application source changes.
- Decisions: Proposed common per-server encrypted load/update acknowledgement, same helper/server renewal ownership, callback preservation, adapters for actual storage interfaces and required saved-access proof. No new public route/UI, plaintext fallback, universal binary interception or server-owned credential export. Not selected/implemented by this conceptual question.
- Checks and evidence: Source/record inspection only; known Google1.11.3 helper has no storage hook and persists refresh_token only. No new build/test/provider/helper/native/credential/personal-state execution.
- Resulting state / acceptance: Shared design ready for user discussion; Google remains pending. Existing delivered source evidence unchanged.
- Remaining work and handoff: Obtain user decision before substantial bridge implementation; keep exact helper compatibility and protected runtime verification explicit.
