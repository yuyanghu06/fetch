## November 12 — shadow made visible at Dock size

User’s Dock screenshot looked flat beside the large Figma reference. A96px native check preserves the exported image closely (mean0.156331/255,max6), so the available evidence points to faint shadows after reduction, not image loss in AppKit. Live Dock compositing was not independently captured. Kept large mockup267:800 unchanged and created canonical app-icon export310:799 with a wider/darker bevel and outer shadow for small-size readability. Same824px tile at100,100, white fill, dog and corners. PNG/ICNS regenerated72DPI.96px native after-preview inspected and fixture passes(mean0.162326/255,max6); signed build-only26.85s/strict verification passes. Existing running Fetch preserved; user Quit/reopen required.

## November 12 — rounded outward edge clarified

User clarified “rounder edges” means a curved outward rim across the tile surface, not deeper recessing or another corner-radius increase. Replaced the recessed lighting with a soft raised bevel: inner white highlight40px/20,24/spread10/88%, inner gray42px/-22,-26/spread12/25%, restrained outer shadow18px/6,8/18% and white lift16px/-6,-8/55%. Existing230px smoothed corner shape, white fill, dog793px size/position/artwork/shadow and824px Dock tile footprint retained. Latest export306:799; recessed export304:799 archived. Canonical PNG/ICNS updated72DPI. Composition/canonical/native preview inspected; existing native fixture passes (mean0.174911/255,max15); signed build-only6.89s/strict verification pass. Direct checks; no Fetch relaunch or packaging.

# Retriever logo mockups

Goal: explore the user-selected blue retriever profile picture in Fetch’s neumorphic style.

Scope: generate a concept, remove the outer shadow, use warm off-white matching the supplied app reference, and place it on a new Figma page named logo mockups.

Completion criteria: corrected artwork exists locally and in the official Figma file; inspect the Figma composition. These criteria are met. This completes this requested mockup pass; logo adoption and any vector reconstruction are not established.

Evidence: page267:799/frame267:800,1024×1024, one image fill and no Figma effects. Screenshot visually inspected. [Artwork and prompt](../../docs/logo-mockups/info.md); [Figma](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=267-800). No app tests/builds needed for this image-only change. Direct review, not independent.

## November 12 — rounded frame

Figma frame267:800 has32px corner radius and clipping enabled, matching the app shell. Screenshot inspected; image and effects unchanged. Original local PNG remains square; the rounded mask belongs to Figma.

## November 12 — corrected raised logo frame edge

The earlier corner-only edit missed the requested soft raised boundary. Logo frame267:800 now uses the app window’s existing **Neumorphic / Raised panel** effect style (two inner shadows and three outer shadows), plus shared neo/radius/32 corner bindings. Image fill and dog remain unchanged: the shadows belong to the surrounding frame. Figma screenshot inspected. This supersedes the previous zero-frame-effects state; original local PNG remains unchanged.


## November 12 — lighter retriever and brighter eyes

Revised the rejected logo concept using built-in imagegen: lighter pastel-blue dog (target #BCE7F8), deep-blue eyes with small white catchlights, same silhouette and warm-white background. Updated image fill on Figma267:800 and inspected the composition screenshot. Current frame has114px rounded corners and the existing Raised panel effect style; preserved as found during final readback. New source saved as docs/logo-mockups/retriever-pastel-blue-bright-eyes-v2.png; v1 retained. This is a revised raster proposal, not user acceptance of the logo.


## November 12 — separate PNGs and revised logo contrast

Logo267:800 now contains a separate native warm-white background283:799 and transparent dog group285:801 (image283:800 plus silhouette mask285:799). Exported fetch-dog-transparent.png (1024×1024) and fetch-logo-background.png (1094×1098 including external shadow) under docs/logo-mockups; both have alpha and were visually inspected. Dog recolored to stronger sky blue (target #7DBFE2), with black eyes and tiny white highlights. White fill #F7F6F1 retained. Background retains114px corners; local inner effects broadened to44/56px and close shadow softened to12px. Shared app styles unchanged. Before/after dog rectangle is1024×1024 at0,0 with identity transform; mask and group geometry retained on the final swap. No automatic logo adoption.


## November 12 — simplified retriever shading

Reduced the heavy muzzle and ear shadows into shallow broad shading and a simple mouth curve; stronger blue and black highlighted eyes retained. Updated Figma image283:800 in logo267:800. Before upload the current dog rectangle was793×793 at117,115 inside group285:801; restored that exact size/position after upload and checked absolute bounds237,235,793×793. Aligned the native silhouette mask to the smaller image. White background, effects and114px corners compare unchanged. Composition screenshot inspected. Saved new transparent PNG docs/logo-mockups/fetch-dog-simplified-v4.png (793×793, alpha); previous PNGs retained. Proposal awaiting visual feedback.


## November 12 — light blue dog with tongue

Returned the simplified retriever to pale baby blue (target #BCE7F8) and added a compact open smile with a pink tongue. Black highlighted eyes retained. Updated Figma image283:800; exact793×793 size and absolute237,235 position restored/verified. White background fill, effects and114px corners compare unchanged. Composition screenshot inspected; transparent cutout saved as docs/logo-mockups/fetch-dog-light-tongue-v5.png (793×793). Prior variants retained; this is a proposal awaiting feedback.


## November 12 — midpoint blue logo

Dog PNG now targets #9DD3ED, midway between prior #7DBFE2 and #BCE7F8. Existing simplified face, black highlighted eyes and pink tongue retained. Updated Figma283:800; before/after size and position compare unchanged (793×793, absolute237,235), as do white background/effects/corners. Screenshot inspected. Saved docs/logo-mockups/fetch-dog-midblue-tongue-v6.png,793×793 transparent cutout; earlier variants retained. Generated color target is approximate.


## November 12 — subtle dog drop shadow

Added one soft shadow to dog cutout group285:801:14px blur,6px downward offset,16% blue-gray opacity. Dog PNG/color/expression and793×793 absolute237,235 image bounds retained; white/background treatment unchanged. Composition screenshot inspected. Saved docs/logo-mockups/fetch-dog-midblue-shadow-v7.png (821×821 including shadow bounds). Unshadowed v6 remains available. Effect remains separately editable in Figma.


## November 12 — clearer ears and larger dog shadow

Dog ears now have deeper blue tone and clearer curved attachment shading. Midpoint-blue face, black eyes and smiling mouth/tongue retained. Figma dog group285:801 shadow expanded from14px blur/6px offset to28px blur/10px offset,2px spread and18% opacity. Image283:800 geometry compares unchanged (793×793, absolute237,235); white/background compare unchanged. Screenshot inspected; saved docs/logo-mockups/fetch-dog-defined-ears-shadow-v8.png (821×821 exported bounds). Prior variants retained.


## November 12 — rounder recessed icon and corrected Dock footprint

User requested corners like the neighboring Dock icon, clearer inward indentation, and comparable visible size. Source logo267:800/background283:799 now use230px corners with0.6smoothing. Background has only two inward effects: dark upper-left36px blur/10px spread at16,20 with32%opacity, and white lower-right30px blur/8px spread at-16,-20 with90%opacity. White fill, dog artwork, dog793px size/position and dog shadow retained. Latest export304:799 is1024px with an824px tile at100,100, reducing the previous roughly910px visible face. Previous export297:799 archived. Canonical PNG/ICNS regenerated at72DPI; native fixture/resource checks and signed build6.55s pass. Native preview inspected; direct/sequential checks, no independent new review. Fetch not relaunched; actual neighboring Dock comparison awaits user Quit/reopen.

