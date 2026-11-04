# Explain source folders and record URL-shaped route organization

- Local timestamp / acting role / milestone: 2026-11-03T14:45:09.644674-05:00 / Orchestrator / MCP MVP architecture clarification.
- Requested scope: Explain current src folders and how they relate; record that api should contain HTTP handlers grouped by URL path, with backend functions allowed elsewhere.
- Starting repository state: Clean working tree; existing source has api/mod.rs registrations/listing, api/setup/mod.rs discovery, mixed api/runtime.rs handlers and HTTP endpoints in oauth/mcp.
- Work performed / changed files: Inspected actual route registration, handler and module guides; updated PROJECT_CONTEXT.md, NOTES.md, docs/architecture/README.md, src/info.md, src/api/info.md, milestones/milestone.md and milestones/to-do.md; appended this entry.
- Decisions: User-selected separation is HTTP handlers organized by URL under api, with reusable backend functions grouped by responsibility outside it. No public URL or behavior changes selected here. Exact Rust path naming remains an implementation detail.
- Checks and evidence: Read source route registrations/functions and module guides; git status was clean before documentation edits. No build, test, service, provider, MCP or personal-state operation performed. Explanation/documentation review was performed by the Orchestrator; no independent specialist review claimed.
- Resulting state / acceptance: Requirement recorded and current mismatch identified. Source structure remains unchanged. This pass is explanatory, not completed refactoring.
- Remaining work and handoff: Explain current folder responsibilities and an illustrative URL-shaped layout to the user. Future bounded refactor must include all existing HTTP adapters and preserve backend authority and public behavior. Existing active work remains on the to-do list.
