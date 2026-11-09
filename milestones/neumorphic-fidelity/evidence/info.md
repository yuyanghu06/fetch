# Figma comparison evidence

The Figma images are fresh exports from the official Fetch design file. Native images draw actual Iced widgets with disposable sample data. They do not launch Fetch, open a listener, execute API tasks, use Keychain, or connect to an MCP server.

## Images

- `buttons-reference-left-native-right.png`: Figma's 524 × 208 button sheet beside the native sheet at the same size and positions.
- `native-buttons-2x-1048x416.png`: the same logical sheet drawn at 2× physical resolution, not an enlarged 1× screenshot.
- `tools-app-reference-left-native-right.png`: matching tools fixtures at 1248 × 1128. Only the Figma presentation margin and 32-pixel OS title bar are excluded.
- `native-design-tools-1248x1128.png` and the cached counterpart: initial and cached renders must be identical, including all backgrounds and icons.
- Settings, Install popup, Console, Connect, minimum-window and scrolling PNGs verify the same shared controls in real page layouts. Number arrow plates are white; checkboxes retain blue or gray faces.
- `initial-*` files preserve failed diagnostic renders. `order-a-*` files preserve the earlier shadow ordering for the recorded A/B comparison. They are historical evidence, not final appearance.

## What the measurements mean

`compare.py` measures RGB differences at identical dimensions and writes `pixel-comparison.json`. It does not turn an arbitrary tolerance into a pixel-perfect claim. Flat face colors and geometry are checked directly. Font edge smoothing and the blur implementations still change some pixels. The Button sheet is a visual state specimen; its styled Pressed example does not prove literal font equality during every live gesture.

Rendering uses the exact design shadow values and ordering. Inset filter input includes enough transparent space for the inverted-alpha blur; final output remains clipped inside the face. Original static SVG bytes are unchanged. Foreground layers preserve native control fills above decorative images.

No actual native operating-system, provider-account, Keychain or signed-launch acceptance follows from these captures. See the tester report and the milestone for commands, outcomes and remaining limits.
