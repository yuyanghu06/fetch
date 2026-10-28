# bounded install list plan

- Local timestamp / role / milestone: 2026-10-28_18-22-18-439052 America/New_York / planner / install-list-controls.
- Scope: user's screenshot shows a long discovery list hiding Install selected servers; requested own bounded scroll area/fixed action and then Select all at the top.
- Starting state: main91d491f plus preserved uncommitted dev/picker changes; prior native compilation evidence only.
- Work/evidence: Read existing view/install_view and Iced scrollable API source. Planned finite Fill Install page without its whole-page scroll, stable always-mounted discovery scroller and fixed footer; variable progress/messages stay in list. Included top Select all request. No edits/execution.
- Ownership: Coder only src/ui/mod.rs; Root all guides/context/notes/journal. No unrelated changes overwritten.
- Decisions: supported source-disabled rows require user choice; Select all is explicit bulk choice, no source flag mutation. Unsupported entries stay unavailable; no importer change.
- Result: source-scope acceptance, not proof of native geometry or working installations.
- Handoff: Root binary compilation successful; manually verify long-list/minimum-size/checkbox/empty/busy/picker/install states after relaunch. No apps/tests/providers/client configs/credentials/trust/release actions by specialists.
