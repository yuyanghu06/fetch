# Hover glow, smaller window and output response

Goal: remove hover popup text, add a subtle fading hover glow, improve Console/Terminal response, and reduce default launch dimensions by 30 percent.

Selected scope: native UI only. Shared cached blue halo fades in over 150ms for enabled buttons with a filled surface and disappears when hover ends; selected state alone adds no glow. Remove all existing native tooltip wrappers. Boot/reopen use 874×630 instead of 1248×900; minimum size remains 760×520. Pane selection must appear before any HTTP response; measure full retained-log rendering before choosing a performance fix. Preserve raw log/output data, terminal sessions, server APIs, current frameless window and card scrollers.

Acceptance: realistic hover event/redraw pixels at start/middle/end, no tooltip overlay, leaving restores idle; actual Console/Terminal clicks before network completion and measured full-log/reused-terminal drawing; existing UI/first-frame checks and signed build. No live shell/MCP/account/Keychain access. Direct sequential work/review, not independent.

Status: scoped work complete. Seven widget/two fidelity checks pass; screenshots inspected; signed build/strict verification pass. Direct review ACCEPT, not independent. Console full-history fixture79–89ms versus587–653ms before; Terminal14–17ms. Native shell startup remains backend-owned/unmeasured. See [evidence](evidence/info.md). Keychain same-binary test remains pending and is a separate unresolved issue; this new UI request necessarily changes the next build, so repeat that diagnostic only on one unchanged executable.
