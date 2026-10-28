# Clarify GitHub login persistence

- Local timestamp / acting role / milestone: 2026-10-27T10:03:28.114811-04:00 / Orchestrator / mcp-mvp-architecture.
- Requested scope: Explain why the local MCP token can be RAM-only while GitHub login survives a computer restart.
- Starting repository state: D24/D25 requirements recorded; GitHub local OAuth memory-only behavior researched; auth integrations and other route policies pending, implementation paused.
- Work performed / changed files: Rechecked pinned local MCP OAuth docs, GitHub CLI login manual and GitHub cookie documentation. Added the distinction/sources to research.md and refreshed architecture guide, to-do, snapshot and milestone guide. No application or personal credential access/changes.
- Decisions: No new product or storage choice. Browser/CLI persistence does not imply MCP token persistence. No credential migration, plaintext fallback or Keychain selection for Fetch.
- Checks and evidence: Primary docs confirm local MCP RAM-only OAuth, browser login cookies and CLI system-store persistence with documented fallback. Local Markdown links/fences/whitespace checked after serialization; no app test or independent specialist pass for this narrow clarification.
- Resulting state / acceptance: Factual clarification recorded; existing D24/D25 requirements and open integration decisions preserved.
- Remaining work and handoff: Continue the authentication integration/contract review; verify actual encrypted storage and service endpoint before claiming support.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
