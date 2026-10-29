# Shared UI components

Confirmed October 28: commonly reused UI elements must be shared components in
Figma and code. The user subsequently rejected the styled mockups and requested
literal copies of the current app. This registry now follows that correction.

Use the [current component page](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-485).
The earlier styled components are archived; their appearance is not approved
for implementation.

The source-state expansion also defines shared
[Credential editor](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=34-438),
[Inline authentication](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-556),
[Authentication server card](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-929)
and [Browser connection error page](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-740)
families. See [all state coverage](UI_STATE_COVERAGE.md).

The credential editor is already shared in current code through
src/ui/authentication.rs::credential_editor and is used by both Settings and
inline authentication. Keep that common implementation: exact labels,
10-pixel spacing, masked input, enabled conditions and optional Remove action.
Source code changed concurrently during this design pass; the nine-phase
authentication module, including NeedsVerification, is represented in Figma.

| Current family | What is shared | Caller supplies |
| --- | --- | --- |
| [Navigation](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=22-484) | Fetch text, five ordinary purple navigation/Quit buttons and the existing tagline | Navigation and Quit callbacks. The current source has no active-page pill treatment. OS window controls remain OS-owned. |
| [Button](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-492) | Current Iced primary appearance, 5/10 padding, small corners and enabled/disabled states | Label, action and enabled state. Preserve explicit label-size overrides for server/tool disclosure buttons. Current Remove entry is also an ordinary primary button. |
| [Text input](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-493) | Current full-width Iced input with 5-pixel padding and 2-pixel corners | Value/placeholder, input callback, width, enabled state and masked-entry behavior. Descriptions remain separate text in the current layout. |
| [Checkbox](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-508) | Current 16-pixel Iced control, 8-pixel label gap and selected/unselected/disabled states | Label, selected state, enabled state and toggle callback. |
| [Server card](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=27-370) | Gray rounded container, disclosure button, ID, action row, plain errors and call counts | Identity, runtime/authentication labels, expansion, available actions, errors and counts. Figma has collapsed, expanded and recovery variants. Code must retain the source's actual combinations rather than hard-code three sample states. |
| [Message](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=29-346) | Plain inline message text or the existing padded error container | Text and applicable context. Do not replace it with a colored alert panel without a separate UI change. |
| [Tool disclosure](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=29-354) | Current tool disclosure button, 10-pixel surrounding padding and raw JSON text | Tool name, expansion and preserved raw definition. |

Browser HTML keeps its own shared controls:
[checkbox](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=32-355)
and [button](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=32-356).
Do not apply the native app's purple Iced appearance to browser-default controls.

Status labels are ordinary text inside the server disclosure and progress rows.
The earlier standalone colored Status badges are not part of the current app.
The archived Window header, Field and Notice proposals likewise do not override
the actual Navigation, Text input and Message structure above.

## Required use in code

- Implement repeated native rendering in shared Rust/Iced functions or modules.
  Pages pass data, current state and callbacks rather than copying widget trees
  or styles. Browser pages use shared HTML helpers and appropriate common styles.
  Reuse within each platform; no new framework is selected.
- Keep common colors, typography, padding, gaps and corners in the shared theme,
  matching the current Iced Light defaults until an explicit UI change is made.
  Update shared components when changing their appearance, then inspect all
  pages that reuse them.
- Preserve current hierarchy, text, scrolling and availability rules when
  copying the app. New layouts or cosmetic treatments are separate changes,
  not routine translation from code to Figma.
- Keep API calls, authorization, validation, persistence, process control and
  workflow decisions in their existing owning layers. Common components own
  repeated rendering, not a separate backend.
- Use options and composition for real state combinations. Mockup names, paths,
  sample call counts and credential placeholders are fixtures, not production
  component defaults. Continue to mask private entry.
- Extend this registry as repeated patterns appear. Do not force unrelated
  controls into one helper solely to share code.

## Current implementation state

Shared Figma components and linked instances exist. This design pass did not
change application code. Current source now has the shared credential_editor
described above; broader custom UI extraction remains a separate implementation
task. No Code Connect mapping or passing runtime result is claimed.

Review populated, empty, busy, disabled, error, expanded and scrolled states when
code changes are selected. Preserve the fixed Install footer and actual long
Settings layout. Existing authorization still governs builds, tests and runtime
work. See [FIGMA.md](FIGMA.md) for screenshot evidence and typography limits.
