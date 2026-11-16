# Complete-history API presentation and subagent popup

- Local timestamp / acting role / milestone: 2026-11-13T16:19:35.076662-05:00 / UI Coder + Orchestrator integration / official-session-readers.
- Requested scope: Complete official conversation responses, collapsed named tools, formatted subagent popup, Finder buttons and no Saved line.
- Starting repository state: Native reader/backend in progress; preparatory custom parser and chunk UI still present.
- Work performed / changed files: UI Coder changed src/ui/workspace.rs/components.rs; root changed src/ui/session_history.rs, mod.rs, workspace_tests.rs and enabled Iced Markdown in Cargo.toml/lock. Root inspected actual files. Preview reuses existing native TOML/YAML parser through pure function; Markdown is cached from returned content. Shared editor remains available via Edit inside popup and inline for Instructions. Shared small Finder icon uses the approved route next to supported paths. Save clears successful feedback. Native conversation/list view replaces raw chunks and rejects incomplete success.
- Decisions: Keep UI a display wrapper over normalized complete native messages. Original raw APIs remain independent. Preview link click has no OS-open action in this scope.
- Checks and evidence: Initial integrated UI9 cases pass (3.28s) in evidence/integrated-ui-tests.txt, covering complete long replies, cache, collapsed exact-copy tools, app scopes, undo and scrolling. Focused new popup/Finder test and independent backend/API testing still running; not claimed passed yet.
- Resulting state / acceptance: UI implementation ready for final test/review. No real Finder/clipboard/model/account action executes in widget fixtures; Tasks are dropped.
- Remaining work and handoff: Popup visual evidence, focused backend/API results, independent evaluation and signed development build. Preserve unrelated shared working tree.
