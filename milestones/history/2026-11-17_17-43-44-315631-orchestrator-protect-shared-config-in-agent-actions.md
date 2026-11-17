# Protect the shared Codex config from row actions

- Local timestamp / acting role / milestone: 2026-11-17T17:43:44-05:00 / Orchestrator / Workspace agent actions
- Requested scope: Keep Paste and Delete confined to individual subagent definitions.
- Starting repository state: Initial review found a shared Codex config could be recognized through a legacy registration.
- Work performed / changed files: files.rs excludes shared global and project config identities from discovery and scope approval, with normalized and resolved paths checked. DELETE checks the selected leaf as well. tests/workspace.rs now verifies that native PUT and DELETE both reject the shared file and preserve its bytes.
- Decisions: Keep the legacy role metadata lookup for actual definitions. Guard source registration files without editing them.
- Checks and evidence: Diff check and final focused rerun passed.
- Resulting state / acceptance: The noted shared-file path is protected.
- Remaining work and handoff: Final independent review and signed build.
