# Clarify credential references and individual settings scope

- Local timestamp / acting role / milestone: 2026-10-27T10:56:49.682744-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: User asks why key is sent and whether MCP servers normally store individual settings in configs; this API must cover individual-server settings.
- Starting repository state: Documentation only with implementation paused; current GET/auth/restart approvals intact, config schema unapproved and previously limited to args/directory/references.
- Work performed / changed files: D29 scope decision and source research integrated; clarified opaque reference/private handoff; marked three-field complete-claim superseded/incomplete. Updated instructions/context/notes/architecture checkpoint, to-do/snapshot and affected guides. Root owns docs; immutable history untouched.
- Decisions: No key is a config-tool argument. D29 requires broader individual settings, but does not approve schema/native file access, settings mappings, live operations, automatic redaction, agent environment-write exceptions or implementation.
- Checks and evidence: Before research/journal append root docs checks passed 64 Markdown/14 JSON/247 local links/44 anchors/40 history; sources independently opened for spec/Playwright/filesystem. No application/runtime tests. Evaluate final research/doc changes next.
- Resulting state / acceptance: Research/scope clarification documented; independent evaluation pending. Earlier contract remains unapproved and needs revision.
- Remaining work and handoff: Verify/evaluate documentation, explain findings plainly, retain existing boundaries and review complete revised contract after settings approach is selected.
