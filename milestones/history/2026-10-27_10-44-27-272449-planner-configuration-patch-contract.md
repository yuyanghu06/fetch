# Plan the complete configuration PATCH contract

- Local timestamp / acting role / milestone: 2026-10-27T10:44:27.272449-04:00 / planner / mcp-mvp-architecture
- Requested scope: Read-only bounded proposal for the existing configuration PATCH, after D28 layout approval.
- Starting repository state: Documentation only, Git main with untracked project records; restart/GET/general-auth approved, write contract incomplete. Root is sole writer.
- Work performed / changed files: Inspected Planner report and auth-state follow-up; no specialist files changed. Root integrated the proposed input/result/permission/error/overlap/save rules into docs/architecture/api-contract.md.
- Decisions: Three writable fields: args/working_directory/environment; immutable managed launch prefix; full-list replacement; no-op; server-scoped references; serialized saves; active login rejected, active restart edits allowed. Root proposes conservative unknown auth after actual launch/reference edits, with stale-result protection.
- Checks and evidence: Read-only reasoning against D20/D23/D25/D27; no HTTP, credential, package, runtime or provider checks run. Root inspected actual draft.
- Resulting state / acceptance: Planner proposal integrated for independent documentation review; product choices remain unapproved.
- Remaining work and handoff: Evaluate consistency and present the entire contract for user feedback; implementation/file/crypto/integration decisions remain open.
