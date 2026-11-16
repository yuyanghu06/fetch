# Official full session readers

Goal: reuse official Codex/Claude saved-history readers, display complete manual user messages, assistant replies and named collapsible tool results. Also render subagent definitions in a popup, remove Saved feedback, and reveal instruction/subagent files in Finder.

Status: implementation COMPLETE; independent Evaluator ACCEPT. User approved the complete API/dependency plan in docs/architecture/session-reader-reuse.md and separately approved Finder reveal. The separate Codex helper compiles on installed Rust1.94.1; pinned Claude SDK0.2.181 and private Python3.12 are installed. Native source selection, complete-history reconstruction, confined fixtures and UI follow-ups are integrated. Five global specialist roles were completed separately.

Boundaries: old raw Workspace/project/MCP reads stay unchanged; complete conversations require the native window permission. Original histories/settings/index are read-only. Codex uses native storage libraries on selected/ancestor snapshots and exact read-only native index selection, without configuration/auth/model/MCP startup. Claude uses the pinned official SDK supplied-text chain reader. Setup is explicit. Runtime confinement,120second deadline,32MiB ceiling and owned-child cleanup apply.

Completion criteria: complete native legacy/modern/lineage fixtures, Claude chain/parallel tool fixtures, native-only route authority and original hashes, focused actual-widget popup/copy/formatting checks, independent ACCEPT, signed development build. No real Finder launch or personal model/MCP/account operation in checks. User must Quit/reopen to see the rebuilt app; native interaction remains user observation.

Earlier evidence/frontend-* files establish preparatory EOF/tool UI behavior only. Final integration, independent checks and ACCEPT are complete. See evidence/tester-native-reader-report.md, final-ui-tests-and-captures.txt and final-signed-build-after-error-display.txt. Actual personal-history/Finder observation remains after explicit Quit/reopen.
