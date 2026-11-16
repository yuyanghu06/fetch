# Server expansion, refresh hover and native window fixes

Goal: reduce the reported first expansion delay, strengthen/animate refresh hover, move native controls left with hover symbols, and remove only the right shadow cutoff.

Scope: shared native surfaces/buttons and AppKit bridge; preserve APIs, server behavior and existing work.

Implemented: Implemented the requested native UI changes directly, without subagents. First server expansion draws without server I/O; logical-density soft shadow textures with linear sampling reduce the measured Retina fixture from199–201ms to72–97ms, with8–16ms repeat expansions. Text/icons keep full display resolution. Enabled shared refresh buttons gain a stronger blue glow and a1.2-second clockwise hover rotation; leaving/disabled state stops rotation and existing actions remain. Actual AppKit window buttons move from top-right to top-left24px/top18px. A tracking area and AppKit group-hover hook expose native symbols. Only clipped right-side Panel shadows fade through the existing gutter; left shadows, card bounds and scrolling stay unchanged.

Acceptance: focused actual-widget interaction/timing, right-edge pixel check, hidden native control check, visual inspection and signed build.

Verification: Nine ui::neumorphic_tests pass, including actual first-expansion clicks, timed refresh hover/leave/disabled behavior and original restart action; three ui::fidelity_tests pass, including right-edge fade pixels. Hidden Objective-C fixture verifies left/top anchors, resize, native hover rendering/state transitions and close delegation. Inspected default Servers/Tools, refresh-hover and native control captures. Final signed development build/strict verification passes. Existing compiler/deprecation warnings remain.

Limits: No running Fetch relaunch, live provider/MCP/account calls, personal configuration changes or Figma writes. UI screenshots use the real widgets offscreen. Native controls were checked in a task-owned hidden AppKit window, not the live Fetch window; live macOS appearance still requires explicit Quit/reopen. The group-hover hook is undocumented AppKit behavior, also used in Chromium's custom control container; future macOS changes may need rechecking. Review was direct/sequential, not independent. Existing unrelated edits and unfinished acceptance work remain.

Status: direct review ACCEPT for the requested scope. Checks and final signed development build pass. User explicit Quit/reopen remains for live-window inspection.
