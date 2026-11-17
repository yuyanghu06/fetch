# Add confirmation and native removal for one definition

- Local timestamp / acting role / milestone: 2026-11-17T17:43:44-05:00 / Coder / Workspace agent actions
- Requested scope: Add a same-size red X and require an explicit confirmation before removing a definition file.
- Starting repository state: Shared row icons and existing scoped file APIs were available.
- Work performed / changed files: workspace.rs and ui/mod.rs show the named confirmation and issue the DELETE only after confirmation. The native route uses the normal Workspace authority and resolves the selected agent file.
- Decisions: Delete acts on the selected definition only. The shared Codex registration stays outside this operation.
- Checks and evidence: Initial focused temporary-file widget test exercised X, Cancel, confirmation, deletion and keeping the neighboring file and Codex registration intact.
- Resulting state / acceptance: User flow implemented.
- Remaining work and handoff: Apply the review finding about the shared config path and rerun.
