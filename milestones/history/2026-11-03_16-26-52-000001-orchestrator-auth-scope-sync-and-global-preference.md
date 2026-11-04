# Authentication scope sync and global implementation preference

- Local timestamp / acting role / milestone: 2026-11-03 16:26:52 America/New_York / Orchestrator / shared-mcp-authentication.
- Requested scope: Save the user's global rule against unapproved small edge-case handling; sync the shared authentication approach before starting implementation.
- Starting repository state: Delete API64cff43 and server-header ×63fa83e delivered; unrelated documentation changes remain. Upstream/helper research completed; authentication source rewrite not started.
- Work performed / changed files: Added a dated instruction to /Users/yuyang/.codex/AGENTS.md; updated milestone snapshot, to-do and authentication milestone status. Read existing authentication-engine documentation. No application source edits.
- Decisions: Small optional/speculative cases require explicit user approval before implementation. Shared standard MCP web authentication remains the proposed direction; final scope sync requested by user is pending.
- Checks and evidence: Inspected current files and Git status. This documentation pass runs no builds, tests, app, provider or MCP processes. Earlier research is source inspection, not live login verification.
- Resulting state / acceptance: Global preference recorded. Authentication rewrite paused for the requested scope sync; no new route or provider-specific engine implemented.
- Remaining work and handoff: Confirm the bounded common Authenticate wrapper, then implement reusable upstream-backed flow with existing storage/authorization. Preserve server-owned callbacks and actual necessary helper compatibility; ask before optional scope additions. No packaging or release.
