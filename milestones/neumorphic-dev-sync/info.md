## Subsequent user correction

The user rejected this pass's approximate glossy appearance. Its functional/source acceptance remains historical evidence, but its visual acceptance is superseded by [neumorphic-fidelity](../neumorphic-fidelity/info.md). Do not treat the earlier approximation as the selected final appearance.

# Neumorphic development app

Goal: bring the approved native Figma design into the development app.

Status: implementation and window-free verification complete; independent Evaluator ACCEPT. The app has not been launched. Dedicated local Fetch development signing setup is missing.

Implemented: warm-white/blue shared controls and rounded depth; three navigation tabs; recessed server/tool sections; inline server Settings with numeric arrows and existing manual-key editor; explicit Tools/Console/Terminal panels; round authentication/restart/removal with Shift-click force restart; discovery/install popup and Connect styling. Existing backend APIs, raw tool definitions, secrets/session guards and terminal sessions remain intact.

Scope limits: manual shell-command installation has no agreed execution contract and remains unimplemented. Browser approval pages are outside this native port. OS titlebar remains native. Iced button effects approximate Figma shadows; pixel-exact rendering is not claimed.

Acceptance evidence:52/52focused UI checks PASS; after final value-wrap change,3/3affected widget tests PASS again. Final locked native binary build PASS in6.01seconds. Sixteen source-real sample-data PNGs cover1248×900and760×520screens, tools scrolling and minimum-window wrapping. Independent Evaluator reviewed source, logs and fresh screenshots and ACCEPTed all scoped changes. [Tester report](evidence/tester-report.md); [assets](assets.md); [evidence guide](evidence/info.md).

The fixtures use temporary state and synthetic keys. No actual Fetch window, listener, API task, Keychain operation, provider login, shell/MCP connection or personal-client change occurred. Existing uncommitted Figma documentation was preserved.

Launch readiness: macOS27.2, Rust/cargo1.95 and Xcode clang are available. The signed launcher --build-only exits because FETCH_DEV_SIGNING_IDENTITY and the selected fingerprint file are missing. Only an Apple Development identity is listed, not the project's dedicated selected signer; no identity was silently chosen. Follow [the per-Mac checklist](../../AGENTS.md) and [development guide](../../docs/testing/DEVELOPMENT.md), then run ./scripts/run-development.sh. Keychain Access certificate creation and native approvals belong to the user. Trust settings remain unchanged.

[Final acceptance](../history/2026-11-09_14-53-49-754916-evaluator-accept-native-neumorphic-port.md).
