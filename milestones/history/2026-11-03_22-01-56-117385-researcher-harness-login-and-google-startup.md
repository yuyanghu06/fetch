# Inspect coding clients and exact upstream Google package behavior.

- Local timestamp / acting role / milestone: 2026-11-03T22:01:56.117385-05:00 / researcher / native-server-storage research follow-up.
- Requested scope: Inspect coding clients and exact upstream Google package behavior.
- Starting repository state: main135873a with prior local journal and concurrent UI/documentation changes; preserved.
- Work performed / changed files: Compared Claude Code, VS Code, OpenCode and Cline. Google1.11.3 initial startup can initiate its own login; auth forces reauthorization. Its source logs the sign-in URL to stderr, unlike README browser-opening prose. Fetch discards that output. Saved instance and actual failure cause were not inspected.
- Decisions: Research only; no new product route, helper command source or output-presentation policy selected.
- Checks and evidence: Primary URLs and exact Fetch source responsibilities recorded in docs/architecture/mcp-login-discovery-research.md. Root reviewed both specialist reports and applied evaluator wording correction. No runtime verification claimed.
- Resulting state / acceptance: Read-only research complete and findings integrated; no Google runtime success claimed.
- Remaining work and handoff: Review how server-owned initial-login instructions/link reach the person, and explicit commands only where required. Then implement/test selected behavior in the protected environment; no personal-state changes authorized by this report.
