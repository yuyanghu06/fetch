# Match the Figma appearance

Status: visual correction implemented, built and independently ACCEPTed. The glossy approximation is removed. Literal pixel equality is not established.

The app now uses Figma's flat fill colors and complete soft shadow layers. Shared typography, spacing, corner radii, numeric arrow strips, icon slots, disabled states and idle scrollbars follow the design. Complete original Tool JSON stays in the existing API/state while cards show only Name and Required fields. Existing server actions, credentials, terminal sessions and raw data rules remain unchanged.

## Verification

The independent Tester ran all 54 UI checks successfully. After the final rendering fixes, the two fidelity checks and three affected interaction/render checks passed again. The final locked native binary build passed in 6.69 seconds. [Tester report](evidence/tester-report.md), [evidence guide](evidence/info.md).

Equal-size comparisons cover the 524 × 208 Figma button sheet, a true 2× physical capture, and 1248 × 1128 native Tools/Settings specimens with matching visible data. Minimum-window and normal page captures include checkbox faces and white numeric arrow plates. Initial and cached page pixels are identical. Cold tools rendering took 1.697 seconds; cached rendering took 0.005 seconds in the window-free fixture.

Final mean RGB difference on the 0–255 scale:

- Button sheet: 0.5886.
- Decoration with text boxes excluded: 0.2160.
- Full matched tools page: 1.5890.

These are measurements, not a tolerance that makes a pixel-perfect claim true. Font edge smoothing and the blur implementations still differ in individual pixels. The styled Pressed button specimen does not prove literal font equality during every live gesture. [Comparison data](evidence/pixel-comparison.json).

## Rendering and scope

The existing pinned resvg library generates only decorative surfaces, not screenshot backgrounds. Correct straight RGBA, retained per-widget cache/allocation, actual window display scale, compatible image batches and a later foreground layer fix the observed transparency, cold-loading and drawing-order defects. Original SVG asset bytes are unchanged. Only resvg/tiny-skia are optimized in debug builds; Fetch's own code remains a debug build. [Research](research.md).

Verification uses temporary fixtures and synthetic keys. No native Fetch window, listener, API task, provider login, Keychain operation, Finder action, shell or real MCP connection ran. Native 2× whole-app runtime remains unobserved. The local development signer is still missing, so the app was not launched. Follow the existing per-Mac signing checklist before using ./scripts/run-development.sh. No personal-client settings, credentials, services, release or signing configuration changed.

Manual command installation remains outside scope pending its agreed behavior. Browser approval screens and OS title bars remain separate.

[Independent acceptance](../history/2026-11-09_15-27-42-333375-evaluator-accept-neumorphic-appearance-correction.md).
