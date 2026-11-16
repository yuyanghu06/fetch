# macOS distribution source

Info.plist describes the one-process native application. scripts/package-macos.sh takes an already-built executable and creates a drag-and-drop DMG with Fetch.app and an Applications shortcut. It requires explicit FETCH_ALLOW_PACKAGING=1 and a fresh output destination. It never builds, tests, notarizes, publishes, installs trust or changes client configurations. An explicit signing identity is optional input for a later authorized packaging run; no signing credentials are stored here.

Packaging has not been executed. A signed/notarized production build and final notices remain acceptance work after dependency resolution and user-started verification.

## Approved app icon

Fetch.png is the approved1024px square Figma composition with transparent outer margins. Fetch.icns contains the standard16–1024px macOS sizes derived from it. Native development embeds the PNG; Info.plist names Fetch.icns and the packaging script copies it into bundle Resources. No package/release run is implied.

Fetch.png uses72DPI metadata so AppKit interprets1024pixels as1024points; the initial Figma export’s low DPI caused an unnecessarily large snapshot. ICNS was regenerated after this metadata correction.

## November 12 — rounder recessed icon and corrected Dock footprint

User requested corners like the neighboring Dock icon, clearer inward indentation, and comparable visible size. Source logo267:800/background283:799 now use230px corners with0.6smoothing. Background has only two inward effects: dark upper-left36px blur/10px spread at16,20 with32%opacity, and white lower-right30px blur/8px spread at-16,-20 with90%opacity. White fill, dog artwork, dog793px size/position and dog shadow retained. Latest export304:799 is1024px with an824px tile at100,100, reducing the previous roughly910px visible face. Previous export297:799 archived. Canonical PNG/ICNS regenerated at72DPI; native fixture/resource checks and signed build6.55s pass. Native preview inspected; direct/sequential checks, no independent new review. Fetch not relaunched; actual neighboring Dock comparison awaits user Quit/reopen.

Current icon export306:799 has a softly rounded outward rim; the brief recessed treatment is superseded. White face824px at100,100 on1024canvas,72DPI metadata. Source dog/corner geometry retained.

Latest canonical refresh uses Figma export308:799; same rounded outward rim and corrected824px Dock tile footprint.

Canonical export is now310:799: Dock-size shadow enhancement overrides live in this export, while the large267:800 mockup stays unchanged. Do not replace it by blindly cloning the large mockup’s lighter effects. At824px tile size the export uses inner white48px/28,32/spread14/95%, inner gray56px/-32,-36/spread18/40%, outer gray24px/8,12/spread2/30%, outer white16px/-6,-8/55%. Existing dog shadow retained.
