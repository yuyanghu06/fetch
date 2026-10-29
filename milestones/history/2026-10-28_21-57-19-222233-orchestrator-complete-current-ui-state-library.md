# Integrate full source-driven Figma state coverage

- Local timestamp / role / milestone: 2026-10-28_21-57-19-222233 America/New_York / orchestrator / figma-app-pages.
- Requested scope: current-app UI state coverage in Figma.
- Starting state: existing literal baseline, rejected-design archives and concurrent operational-MVP work; preserve all unrelated changes.
- Work and evidence: User requested all currently displayable UI elements, especially authentication and individual settings. Authored/indexed 153 states/specimens: 13 baseline native, four baseline browser, 21 auth, 28 settings, 26 setup/server, 25 browser request/error/response specimens, six explicit native/OS contracts and 30 widget/client-error/minimum-window specimens. Added shared credential editor, inline auth/card and browser error families; extended Iced control states and derived current palette. Preserved existing application code and concurrent operational-MVP source/doc changes. No application source changes, builds/tests/app interactions, managed MCP/provider/client/credential/trust operations, commit/push or release in this design pass. Source hashes and all Figma nodes are in docs/UI_STATE_COVERAGE.md. Removed invalid cross-page prototype inheritance from expanded state boards. Updated canonical Figma/shared-component references, AGENTS.md, context, notes/guides and journal. Current SF Pro limitation and unobserved OS/provider pixels remain explicit. Independent source and representative visual review accepted after fixes. User visual review and all prior runtime acceptance remain open.
- Files/state: specialist roles stayed read-only; Orchestrator owns Figma/doc/journal integration.
- Decisions: no new product API, platform or implementation scope; reuse source-backed shared controls.
- Remaining work: user visual feedback, future source/state synchronization and previously pending runtime acceptance.

## Specialist records

- [planner](2026-10-28_21-57-19-221191-planner-native-ui-state-inventory.md)
- [researcher](2026-10-28_21-57-19-221780-researcher-browser-native-ui-inventory.md)
- [evaluator](2026-10-28_21-57-19-221928-evaluator-native-ui-coverage-corrections.md)
- [evaluator](2026-10-28_21-57-19-222037-evaluator-representative-figma-fidelity-findings.md)
- [evaluator](2026-10-28_21-57-19-222138-evaluator-representative-figma-fidelity-acceptance.md)
