# Window outlines, native hover and resize edges

Goal: remove the reported thin outlines, fix red/yellow hover as well as green, and align resize with the visible edge.

Corrected the dark rounded outlines caused by linear filtering between face pixels and transparent black texture pixels. Fully transparent texels now retain the face RGB at zero opacity; no stroke or shadow stack was removed. Retina edge regression fails before the fix (192/191/187) and passes after (238/237/232). Removed the8px transparent outer page margin and moved it into existing content padding, preserving control/card positions while the visible face reaches the real window bounds. AppKit group hover now refreshes the native theme widgets' cached state using mouseEnteredOrExited instead of forwarding cell enter/exit events, so red/yellow/green update together and clear together.

Acceptance: reproduce the Retina dark rim, verify all native hover glyphs and leave restoration, assert visible/native edge alignment, inspect screenshots and build/sign.

Ten actual-widget UI checks and four visual checks pass. The native hidden fixture independently compares each of the three button captures on hover and exact restoration on exit, verifies content/frame edge equality, left/top anchors through resizing and close delegation; passes. Inspected the Retina no-outline face, default Servers and native hover/leave PNGs. Signed development build and strict signature verification pass; git diff --check passes. First expansion in this final run67ms, returns3–4ms (fixture, not live-window timing). Existing compiler/deprecation warnings retained. Review is direct and sequential, without subagents.

Fetch was not relaunched, and no personal credentials/settings/services were changed. Native screenshots use a task-owned hidden window; real cursor movement/resize behavior and live macOS appearance remain for explicit user Quit/reopen. Native theme refresh/group-hover queries are undocumented AppKit behavior, isolated to the presentation bridge. Prior broad completion claims did not establish live acceptance; this pass strengthens the old native hover test from any group pixel difference to every button plus exact restoration. Earlier unrelated changes and remaining acceptance tasks are preserved. No Figma writes, commit, push or release.

Status: scoped implementation and fixture evidence ACCEPT by direct primary-agent review. User live Quit/reopen remains.
