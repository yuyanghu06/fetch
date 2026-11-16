# Record native setup launch and approval popup

- Local timestamp / acting role / milestone: 2026-11-12T14:11:16.533960-05:00 / Orchestrator / local-mcp-connector.
- Requested scope: User says install process launches/focuses Fetch and shows a popup screen.
- Starting repository state: Connector plan and UI-first alternative; no connector source. New auth/API contract not implemented, unrelated work preserved.
- Work performed / changed files: Updated connector proposal, selected architecture record, project context/notes, scope guide and milestone records. No application/client/credential/trust change.
- Decisions: Confirmed native setup launch/focus/popup rather than required terminal login; reuse existing app instance. Preserve permission selection, Mac confirmation and separate clients. This selects UX, not unshown API/trigger/helper mechanics or auto-grant.
- Checks and evidence: Existing CLI add command registration is distinct from connector execution; no claim add command itself necessarily launches app. Setup-time launch differs from ordinary existing-backend connection. No real client/app/test process was executed.
- Resulting state / acceptance: User intent recorded, earlier presentation-only choice resolved. Exact enrollment contract and startup handoff remain to consolidate/review before dependent auth code.
- Remaining work and handoff: Plan the actual setup trigger and safe popup/proof/token handoff, avoiding client initialization timeout. Preserve no shutdown of shared services, isolated testing requirement and excluded project/release/personal-state work.
