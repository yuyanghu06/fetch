# Tool shadow boundary

Goal: remove the hard vertical cutoff at the right of Available tools while retaining card size/columns, nested scrolling and the earlier navigation performance fix.

Cause from source: Iced Grid always narrows the drawing viewport to its own bounds, clipping the rightmost and bottom card shadows before they reach the scroll padding. Row/Column can retain the parent viewport without that clip. Use ordinary Iced layout helpers rather than custom renderer/provider behavior.

Completion: reproduce the visible missing shadow in the existing real-tools render test, preserve three columns and inner scrolling, visually inspect corrected edges and build/sign. User explicitly selected the whole window frameless: both window creation paths disable decorations and enable transparency; the daemon clears transparently, and dragging unused navigation/title space invokes the native window drag. Existing tabs capture clicks normally. Native titlebar/buttons are removed by that selected appearance.

Status: both scoped visual changes complete. Observed shadow regression fails before/passes after; five widget checks and two fidelity checks pass, three screenshots inspected, signed build/strict verification pass. Direct review ACCEPT, not independent. Actual macOS window behavior awaits explicit user Quit/reopen. See [evidence](evidence/info.md).
