# Plan the manual credentials PATCH contract

- Local timestamp / acting role / milestone: 2026-10-27T11:06:52.706165-04:00 / planner / mcp-mvp-architecture
- Requested scope: Read-only next complete credentials contract following user deferral of native settings normalization and approval of common config.
- Starting repository state: Documentation only; Settings-only manual write purpose approved, exact credential batch schema/policies unapproved. Root sole writer; D20 captures launch settings/refs, not selected secret-value snapshots.
- Work performed / changed files: Inspected Planner report for set/remove/manual entry semantics, private storage/readback, auth/apply status and safe results/errors. No specialist files changed. Root integrated proposal into existing API document.
- Decisions: Proposal covers string values including intentional empty strings, server-local entries, protected OAuth records, storage != auth success and no secret outputs. Planner offered restart-active secret snapshots; root instead proposes busy rejection for active restart/login to avoid silently expanding D20.
- Checks and evidence: Read-only project-contract analysis; no app/HTTP/store/provider tests or credential access. Root compared D20 reference capture before choosing the simpler unapproved overlap proposal.
- Resulting state / acceptance: Bounded credentials proposal inspected and integrated; new route policies await user input.
- Remaining work and handoff: Independently evaluate and present the whole contract, retaining later Settings/key/store/handoff/manual-test integration decisions.
