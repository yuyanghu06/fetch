# Preserve uncorrelated backend errors

- Local timestamp / acting role / milestone: 2026-11-12T16:00:02.282499-05:00 / Coder / server-first-tools
- Requested scope: Diagnose actual reported other-connection/ended-request failure without hiding upstream reason.
- Starting repository state: Fixed server-first API suite18passed, migrated connector13passed; new modern/diagnostic fixtures pending.
- Work performed / changed files: Coder changed only src/connector/mod.rs so backend JSON-RPC error responses with null/absent IDs retain their actual code/message/details and available HTTP status/originating method. Both JSON and SSE paths classify before response correlation; receive has a defensive context-free check. Existing sanitization protects saved credential values. Unknown non-null IDs still fail strict ownership checks. No new notification support, invented response IDs, suppressed errors or replay. Independent read-only reviewer found no source blockers. Regression fixtures and signed build underway.
- Decisions: A backend error without a request ID is not proof responses crossed agents. Keep strict non-null correlation and existing protocol support.
- Checks and evidence: Source inspection/scoped whitespace and independent preliminary read-only review; runtime regression checks pending.
- Resulting state / acceptance: Diagnostic source integrated, no final acceptance yet.
- Remaining work and handoff: Finish16connector checks, final signed build and evaluation. Actual Claude notification/rejection cause still needs manual evidence; this does not claim the user's live failure fixed.
