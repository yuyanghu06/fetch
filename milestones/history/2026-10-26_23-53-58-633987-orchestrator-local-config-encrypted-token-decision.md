# Record local server files and the existing encrypted token store

- Local timestamp / acting role / milestone: 2026-10-26T23:53:58.633987-04:00 (America/New_York) / Orchestrator / mcp-mvp-architecture.
- Requested scope: User selects local files for server configs and the encrypted store already discussed for tokens; no application implementation.
- Starting repository state: PostgreSQL/retention deferred; persistent local metadata/auth still required; no non-PostgreSQL config format selected and no app source.
- Work performed / changed files: Recorded D19 and updated storage-scope,AGENTS,context,notes,architecture index/checklist/open questions/roadmap/guides,handoff and milestone records. Preserved existing credential-reference,per-server assignment,Settings-only manual entry and API-first boundaries.
- Decisions: Local server config files plus the existing encrypted store for actual Fetch-managed tokens/credentials/environment values. No JSON/TOML encoding,path,one-file-versus-many layout,write-coordination or key-protection mechanism selected. Client harnesses retain their own Fetch login credential; job/memory storage and other pending API policies remain open.
- Checks and evidence: Orchestrator checked37 Markdown files and9 JSON examples: local links/anchors,fences/headings/whitespace passed. This narrow documentation confirmation was self-checked; no separate specialist evaluation or application tests ran. No source/packages/services/credentials/personal configs/database installations changed.
- Resulting state / acceptance: User-selected storage split recorded and referenced for the next agent. Architecture milestone remains incomplete and code remains paused; no substitute database adopted.
- Remaining work and handoff: Continue the selected whole-contract discussion order. Review exact file/crypto implementation with user in step5; retain accepted file/secret separation and pending non-storage restart policies. Keep immutable journal and current handoff aligned.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
