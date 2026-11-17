# Startup and Workspace polish

Fetch's first server list appears from top to bottom. Each card starts 120 ms after the previous one, grows from 80% with a small overshoot, and lands in its reserved slot. The panel stays clear while the first read is pending. Manual Refresh still uses its earlier motion.

The shared Finder button has a blue face and a solid white icon with blue eyes and face lines. The subagent popup keeps its path and controls visible while the preview body scrolls as one area. Agent Copy queues the usual clipboard write without announcing it. Empty lists in Servers and Workspace and an empty session popup center their standalone message in the panel.

Seven focused offscreen tests passed in 4.10 seconds. Independent review accepted the source and captures. The signed development build and strict signature check passed. Fetch and the physical trackpad were not used for this pass.

- [Startup frames](startup-sequence.png)
- [Subagent preview before and after scrolling](subagent-scroll.png)
- [Finder button inside the popup](finder-popup.png)
- [Centered empty panels](centered-empty-panels.png)
- [Focused test output](focused-tests.txt)

Quit Fetch and reopen with `./scripts/run-development.sh` to use the new build.
