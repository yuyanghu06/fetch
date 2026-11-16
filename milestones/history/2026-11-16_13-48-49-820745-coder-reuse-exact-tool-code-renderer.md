# Reuse exact tool code renderer

- Local timestamp / acting role / milestone: 2026-11-16T13:48:49.820745-05:00 / coder / workspace-session-error
- Requested scope: fenced Markdown must use the same light component as tool output.
- Starting repository state: custom Viewer used shared style but duplicated a12px expression; actual tool helper's theme size is13px.
- Work performed / changed files: components.rs code_block override now directly calls code_block_without_copy; info.md corrected to shared13px size. No global font setting changed.
- Decisions: literal helper reuse removes duplication and guarantees same font/wrap/surface as tool output. Inline remains14pxBODY_FONT matching prose.
- Checks and evidence: Coder diff check passes; root confirms exact delegation. Affected Markdown check and final signed build running.
- Resulting state / acceptance: final application source stable for verification.
- Remaining work and handoff: complete focused checks/captures and evaluation; no other product changes.
