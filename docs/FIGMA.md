# Fetch Figma workspace

Official file: [Fetch App Mockup](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487).
File key: `OcgOlmfuoSTJMHTeN753TF`. Selected by the user October 28, 2026.

## Complete current-state library

The user's follow-up requested every current UI element, including authentication
and individual settings. The file now contains **153 states/specimens** across
nine current boards, preserving the rejected-design archives separately.
See [the complete coverage index](UI_STATE_COVERAGE.md) for every node and the
finite source-branch checklist.

- [Authentication states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-348): 21 full-app specimens, including all nine current module phases.
- [Individual settings](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-349): 28 save/remove/validation/configuration specimens.
- [Setup and server states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-350): 26 additional restart/catalog/discovery/install/picker states.
- [Browser errors and requests](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-351): requested/custom/retry forms, all 21 hard-coded HTML errors and a raw-response specimen.
- [Native contracts](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-352) and [control/message specimens](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=38-381): OS-owned handoffs, widget interactions, native-client errors and minimum window.

The latest inline authentication and shared credential editor source are included.
Credential rows use stored/missing; the exact Save and verify credential label,
secret clearing and control availability are preserved. Native prompts are
source contracts, not fabricated OS screenshots. Arbitrary data/error values and
all combinations are represented by reusable slots, not an exhaustive permutation
claim. Independent source and representative screenshot review accepted the
reviewed scope after corrections; no new app/runtime action occurred.

## Current UI copies — October 28 correction

The user rejected the first styled designs because they did not look like the
application. The current boards reproduce the existing UI from source, using an
unmodified screenshot of the already-open Fetch window as the visual reference.
Future work must start from these copies, not the archived redesign.

- [Current native UI](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487) — 13 editable frames.
- [Current shared components](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-485) — seven native families plus browser form controls.
- [Current browser HTML](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-386) — four editable frames.
- [Exact native screenshot reference](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-796) — the unchanged captured empty Servers window.

| Current page | Copies and sample states |
| --- | --- |
| Servers | Empty, populated, tools expanded, authentication/uncertain-call recovery |
| Install MCPs | Before discovery, selected definitions, installation in progress, failed/retry |
| Connect apps | Setup instructions, verified cleanup handoff |
| Settings | No selected server, selected server and private fields, advanced settings shown at a scrolled position |
| Browser | App approval, expired request, provider callback success and failure |

Native windows have a 1080×760 client area plus a 32-pixel macOS title-bar
representation. Content keeps the source's 28-pixel outer padding, 20-pixel
navigation spacing, white background, purple Iced buttons, plain text statuses,
small corner radii and single-column forms. Long pages keep their actual
scrolling. Install keeps its header and action footer outside the scrolling
definitions/progress area. The advanced Settings frame shows the lower part of
the same scrolling page rather than a newly invented layout.

Data examples occupy the actual widgets. No dashboard counters, status pills,
separate client cards, colored alert panels or two-column Settings have been
added. Raw tool/configuration JSON remains plain text as rendered by the source.
Sample paths use /Users/example and contain no credential values.

There are 59 native prototype links for navigation and selected workflow
transitions. Copy/save/install/confirmation are illustrative; prototype links do
not execute backend operations. Shared definitions and code-reuse requirements
are in [SHARED_UI_COMPONENTS.md](SHARED_UI_COMPONENTS.md).

## Evidence and limits

The existing Fetch process/window was identified read-only; only that window was
captured. No new app instance was started, no page/button was operated, and no
API, credential, certificate trust or managed MCP action was performed. Empty
Servers is directly screenshot-backed. Other states are reconstructed from
src/ui/mod.rs; browser pages follow src/oauth/mod.rs and src/api/runtime.rs.
Iced 0.14 dependency source supplied default colors, padding, radii, font sizes,
line heights and disabled appearance. Source review and Figma screenshot
inspection are not application tests.

Figma cannot render the native SF Pro face in this environment. Editable Arimo
text is calibrated against the captured native window, with original source font
sizes retained as node metadata. This is a known typography difference; do not
claim pixel-identical native fonts. The unmodified screenshot is the exact
native rendering reference. Browser serif text uses a compatible Tinos face;
browser/OS-owned controls can also vary by platform.

## Rejected earlier proposal

The original 17 styled frames and their components are preserved on pages named
Archive. They were rejected as copies of the current app. Their earlier
62-link count, Inter typography, badges, two-column Settings and dashboard
layout are historical, not the current UI specification. Original linked page
0:1 is now an archive. User review of the corrected copies remains open.
