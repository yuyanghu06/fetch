# Servers install modal

Install MCPs opens from the button directly right of Refresh above the Servers list. The navbar has Servers, Connect apps and Workspace. The modal keeps the destination, source, discovery, selection and progress controls together. Closing it with X, Escape or the backdrop keeps ongoing work available on reopen.

Acceptance covers opening from the installed heading, the minimum-size chooser and results layouts, scoped selection and dismissal during background work. Eleven focused tests, independent review and the signed development build passed. The tests used simulated file picker and installation replies.

[Implementation](../history/2026-11-17_18-16-16-390145-orchestrator-move-install-to-servers-modal.md) · [Tests](../history/2026-11-17_18-16-16-390519-tester-verify-servers-install-modal.md) · [Review](../history/2026-11-17_18-16-16-390655-evaluator-accept-servers-install-modal.md) · [Build](../history/2026-11-17_18-16-16-390759-orchestrator-deliver-servers-install-modal.md)

## Button colors and inline status

The shortcut uses the same white text-button style as other secondary actions. The footer uses a light gray face and dark gray label when disabled. Each server's bar and label reflect the existing installation reply. It animates while installing and shows the result when done.

The focused test verified the palettes, animation, same-name servers in separate files and terminal states. The existing modal check also passed. [Evidence](../history/2026-11-17_18-28-41-722172-tester-verify-inline-install-progress.md) · [Build](../history/2026-11-17_18-28-41-722399-orchestrator-deliver-install-button-colors-and-progress.md)

## Checkboxes and failure details

Selectable unchecked boxes are white. Unsupported and installed rows share the gray style. Failed rows show their original preparation error in the info tooltip. [Checks](../history/2026-11-17_18-40-33-760776-tester-verify-install-paste-and-key-glow.md)
