# Figma fidelity verification — November 9, 2026

Result: the corrected appearance and existing UI interactions pass the focused checks. The images are close to the Figma reference, but they are **not literally identical pixels**. The measured differences below remain explicit; no arbitrary error threshold was used to label them pixel-perfect.

## Commands and results

- `FETCH_UI_EVIDENCE_DIR=.../milestones/neumorphic-fidelity/evidence cargo test --locked --lib ui:: -- --nocapture`: **54 passed, 0 failed**, 6.88 seconds. See [final-ui-tests.txt](final-ui-tests.txt). This full run preceded the last changes to shadow order, inset filter padding and drawing the foreground above its background image.
- After those final drawing changes, the affected checks ran again: `cargo test --locked --lib ui::fidelity_tests:: -- --nocapture`: **2 passed, 0 failed**, 3.45 seconds; and `cargo test --locked --lib ui::neumorphic_tests:: -- --nocapture`: **3 passed, 0 failed**, 6.62 seconds. See [final-affected-fidelity-tests.txt](final-affected-fidelity-tests.txt) and [final-affected-ui-tests.txt](final-affected-ui-tests.txt).
- Final `cargo build --locked --bin fetch`: **PASS**, 6.69 seconds. See [final-build.txt](final-build.txt). Apple API deprecation and unused-code warnings remain.
- `git diff --check` for the assigned test files: **PASS**.

## Matching the Figma reference

The button specimen uses Figma node `75:2252` at its exact 524×208 size. Each control uses the official child bounds for Enabled, Disabled, Hovered, blue Pressed appearance and Selected. The native output includes the same labels, Inter weights and gray specimen background. The blue specimen uses its corresponding primary-button appearance; this board is a visual state specimen, not simultaneous real pointer gestures across five controls.

The full tools fixture uses the six Figma names and Required fields values. Its 1248×1128 content size corresponds to the Figma 1248×1160 window after excluding the 32-pixel OS title bar. `compare.py` removes only the reference presentation margin and title bar for that comparison. The functional fixture remains separate and uses realistic distinct server settings and tool names.

The current matched-size outputs are:

- [Buttons, reference left and native right](buttons-reference-left-native-right.png)
- [Full tools, reference left and native right](tools-app-reference-left-native-right.png)
- [Settings](native-design-settings-1248x1128.png)
- [Install popup](install-popup-1248x900.png)
- [Native 2× button render](native-buttons-2x-1048x416.png)

The 2× image is rendered directly at 1048×416 physical pixels with the same 524×208 logical geometry and the renderer's scale factor set to 2. It is not an enlarged 1× bitmap. There is no independent Figma 2× reference in this evidence set, so this proves the physical-resolution path, not exact Figma equality at 2×.

Final RGB comparisons on a 0–255 scale:

| Region | Mean absolute channel difference |
| --- | ---: |
| Entire button board | 0.5886 |
| Button decoration, excluding known text rectangles | 0.2160 |
| Entire matched tools content | 1.5890 |

[Pixel details](pixel-comparison.json) include maximum differences, exact-match proportions and quiet-point colors. Different text edge coverage and shadow/corner rasterization remain. These measurements must not be described as zero-error or literal pixel equality.

The shadow-layer order was checked with an A/B render. Correct CSS order improved button error from 0.6189 to 0.6129 before the inset-padding correction improved it further to 0.5886. The A images and JSON remain under `order-a-*`.

## Actual rendering and interaction checks

The fixtures create temporary application data and synthetic keys. They render the real native widgets and send actual pointer and wheel events; returned API tasks are dropped without polling.

- The first full frame contains the exact recessed gray face at the reference coordinate and visible white authentication, restart and removal glyphs.
- The first and cached tools frames are **byte-for-byte identical**. This checks that backgrounds and foreground glyphs do not appear only on a later frame.
- The numeric arrow plate is white at the matching reference coordinate, above the input's image background.
- Both window sizes show the actual blue selected checkbox face and gray installed checkbox face, not just their checkmarks.
- Tools scroll inside their own well while the page remains still. The card fields fit at minimum width and retain the complete original tool JSON in the catalog.
- Settings opens within the correct server, loads its distinct limits and assigned key, and clears the previous key value when switching servers.
- Console remains explicit; URL-only remote servers cannot open a terminal. Existing terminal session and output checks remain passing.
- Round restart controls emit their existing messages with normal/Shift modifier state, Copy sends the displayed address, and installation progress can be reopened after closing its popup. No restart request, clipboard write or installation runs.

## Findings corrected during this pass

Early images exposed black halos from premultiplied SVG pixels, missing first-frame backgrounds, raster backgrounds covering SVG glyphs, and foreground quads such as arrow plates and checkbox faces being drawn behind backgrounds. The implementation writers fixed these. The final pixel probes and first/cached comparison cover those actual failures.

The first checkbox-face probe used a mistaken offset: Iced's checkbox operation reports the whole control bounds, rather than the text's bounds. That test coordinate was corrected after visually confirming the filled faces; no application fix was needed for that probe failure. The diagnostic log remains as `checkbox-probe-diagnostic.txt`.

The final isolated tools capture took 1.697 seconds on its first render and 0.005 seconds from the retained cache; Settings took 1.235 seconds on its first render. These are headless fixture measurements, not measured native-app responsiveness. The original SVG path took 55.77 seconds for the initial two export tests; that diagnostic result is retained in `initial-fidelity-tests.txt`.

## Limits and changed test files

No Fetch window, listener, provider login, Keychain action, Finder action, product terminal, MCP connection or real client connection was started. Native operating-system behavior and real-account acceptance remain untested. Native title-bar pixels are excluded from the matched-content comparison. No literal pixel-perfect claim is established.

Test changes are limited to `src/ui/neumorphic_tests.rs`, new `src/ui/fidelity_tests.rs`, and this evidence folder. Product fixes and shared milestone documentation belong to the Orchestrator and Coder. The test fixture loads the same local Inter regular/medium/semibold/bold files as the app, redraws and draws the same widget instance, and preserves the prior interaction coverage.
