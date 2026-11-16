# Fixed Installed servers section

Goal: keep the Installed servers panel within the available app window, with its title/Refresh and navigation fixed while individual cards scroll inside it.

Scope: native Servers page layout only. The section uses the available window height instead of growing with card count or expansion. Keep card appearance/actions and existing inner Tools, Settings, Console and Terminal behavior. Current request selects this native change; the earlier November 8 reverted Figma trial remains historical and is not modified.

Completion: prove wheel scrolling reaches later server cards, header/section bounds stay fixed, and scrolled cards still expand at normal/minimum window sizes. Check existing nested panels and visual specimens, then build/sign. No real server/account work or app relaunch.

Status: complete for scoped native implementation. Five widget checks and two fidelity checks pass; normal/minimum-window screenshots inspected; signed development build and strict verification pass. Direct review ACCEPT, not independent. Existing running app was preserved; explicit Quit/reopen needed for live-window confirmation. No Figma edits. See [evidence](evidence/info.md).
