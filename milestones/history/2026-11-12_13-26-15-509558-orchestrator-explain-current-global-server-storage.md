# Explain existing global server storage

- Local timestamp / acting role / milestone: 2026-11-12T13:26:15.509558-05:00 / Orchestrator / project-scoped-servers.
- Requested scope: Explain how existing global MCP servers are stored now.
- Starting repository state: Prior researched project-scoping proposal remains unapproved; unrelated pending files preserved.
- Work performed / changed files: Read storage, installation, application-root, native-state and credential ownership code. Added actual disk layout to docs/architecture/project-scoped-servers.md; refreshed current milestone records and scope guide.
- Decisions: No new product decisions. Existing collection is installation-wide without an explicit global/project field.
- Checks and evidence: Source confirms hashed server directories/config.json, separate package installation directories, default development/release roots, private default local HOME, Keychain for Fetch-held grants/manual keys and server-owned native credentials. No personal config/state/credential reads, runtime calls, builds or tests.
- Resulting state / acceptance: Factual source explanation complete, direct read-only review; no independent specialist used for this small follow-up.
- Remaining work and handoff: User selection and full project-scoping contracts remain open; existing acceptance tasks unchanged.
