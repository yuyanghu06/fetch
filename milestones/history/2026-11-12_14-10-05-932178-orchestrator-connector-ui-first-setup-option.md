# Clarify UI-first connector setup

- Local timestamp / acting role / milestone: 2026-11-12T14:10:05.932178-05:00 / Orchestrator / local-mcp-connector.
- Requested scope: Can connector approval/setup happen in Fetch UI or on a website instead of a separate login command?
- Starting repository state: Independently reviewed plan, new auth/API choice awaiting user. No connector implementation; unrelated work preserved.
- Work performed / changed files: Added UI-first alternative to docs/architecture/local-mcp-connector.md and milestone scope/current records. No application or client edits.
- Decisions: User question is not implementation/API approval. Recommend UI starts approval, shows scope choices/Mac confirmation and saves separate connector credentials through reviewed backend operations. Separate login command need not be mandatory; exact operation amendments still require user input.
- Checks and evidence: Current native client can verify installation HTTPS cert; existing browser approval independently needs trust. Existing four-operation proposal was connector-initiated, so UI-first credential creation/storage is explicitly a revision, not claimed already implemented. Direct explanatory review, no specialist/runtime test needed.
- Resulting state / acceptance: Proposal updated; no browser/website/deep-link flow, personal configuration write or automatic grant selected.
- Remaining work and handoff: Resolve preferred setup/approval flow and revise exact API contract before auth implementation. Website can show instructions/handoff but cannot remove need for local action on its own. Preserve original bounded scope and isolation requirements.
