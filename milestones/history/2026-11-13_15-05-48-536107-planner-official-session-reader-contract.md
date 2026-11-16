# Planner complete-conversation contract

- Local timestamp / acting role / milestone: 2026-11-13T15:05:48.536107-05:00 / Planner plan_official_session_readers, serialized by Orchestrator / official-session-readers.
- Requested scope: official full-history reader reuse with compact named results.
- Starting repository state: raw APIs/custom JSONL UI, early assistant stop, modern Codex gaps.
- Work performed / changed files: specialist read-only source/docs inspection; root records proposed contract in docs/architecture/session-reader-reuse.md.
- Decisions: native capability only; preserve raw/project APIs; whole messages and tool-name/output fields;120second deadline and32MiB existing response ceiling with real errors. Exact runtime/native-library choice remains proposed.
- Checks and evidence: source-backed planner report and acceptance criteria, no execution/writes by specialist. Root adds official-view list selector to avoid subagent rows based on later structural finding; this addition is proposed, not approved.
- Resulting state / acceptance: plan complete for user review; no new backend route/dependency implemented.
- Remaining work and handoff: user input; safe native library/store proof and official reader parity before acceptance.
