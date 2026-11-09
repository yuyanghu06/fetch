# Settings inside each server card

Goal: replace the standalone Settings page with an inline Settings tab for the selected server.

Scope: new Figma app/component pages only. User explicitly chose inside each server card and removed the navigation gear. Earlier fixed-Settings-navbar proposal was cancelled before writes. No Rust/API or original-reference change.

Shared family202:2558 has Filesystem ready and Search ready/folder-opening/key-filled/key-busy states. Existing folder, limits and assigned manual-key editor controls/values/disabled states are reused. Filesystem fixture shows common folder/limits controls; Search retains its search-key assignment/editor. Each panel is1072x440 with internal scrolling. The exact short setup sentence requested for removal is absent.

Settings views reuse frame IDs78:1639(Filesystem),78:1653(Search),78:1679(manual key),78:1849(opening folder),78:1875(filled),78:1901(busy). Search closed204:4868 provides correct expand/close routing. New design now has30 native-window specimens,22 under Servers. All22 use the approved enlarged window/recessed Installed servers layout. No standalone Settings frames or navbar Settings gear remain on the new board; original source reference pages remain historical references.

Numeric layout: Time until timeout and Max concurrent calls use17pxBold titles, with500px inputs side by side/24px gap. Fetch call limits heading is removed. Shared number input212:2318 shows120 and1 plus small up/down arrows on the right. These are Figma input/stepper appearances, not executable numeric editing/saving. No new backend limits/defaults selected.

Light Copy control115:1371 now has a transparent hit area and blue glyph only. Dark console Copy styling stays legible. The technical API_KEY→search-key button was explained as selecting the assigned key entry; Edit API key was only suggested, not approved or renamed.

Acceptance: Settings opens/closes inside the correct server, selected state is inset without glow, fields/values/busy states remain correct/reachable, requested labels/stepper appearance are present, and no gear/standalone Settings page remains.

Status: independent structure/rendered-appearance checks PASS. Independent final evaluation ACCEPT. [Review](../history/2026-11-09_13-38-31-278029-evaluator-accept-inline-settings-and-rollout.md). [Test](../history/2026-11-09_13-38-02-990504-tester-pass-inline-settings-and-controls.md). [Plan](../history/2026-11-09_13-36-16-905600-planner-bound-inline-server-settings.md); [implementation](../history/2026-11-09_13-36-16-905683-designer-implement-inline-server-settings-and-controls.md). Actual new gestures are not claimed while the user is actively using Figma.
