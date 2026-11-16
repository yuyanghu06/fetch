# Native window controls and compact server panels

Goal: remove the extra outer outline, put real Apple window controls at top right, drag from non-interactive areas, move Installed servers title/Refresh above its recessed list, keep Tools compact/scrollable, and match Terminal to Console.

Scope: use Iced's exact window handle and existing AppKit bridge. Keep the window borderless/transparent, disable the system shadow and remove only the main page's outside decorative shadow. Add actual AppKit standard buttons anchored top18/right24; close delegates through winit/Iced, minimize/zoom use NSWindow. Wrap the full native view in MouseArea; child controls capture their own events. Put heading outside the fixed-height list. Tools use300px high/max744px wide scroller and200px high compact cards. Console and Terminal share the same240px frame/header/padding/colors/font; Terminal Copy exposes iced_term's existing selection-copy text through a five-line getter.

Acceptance: realistic UI/scroll/drag/hover/output checks, terminal input/selection regression checks, visual screenshots, a hidden disposable AppKit window fixture checking real buttons/position/resize/close routing, and signed build. No real Fetch/MCP/provider/Keychain launches or personal-state edits. Test only task-owned hidden windows/processes. Direct sequential review, not independent.

Status: source implemented; initial7UI/11terminal checks pass. Hidden native fixture passes after routing borderless close through existing delegate; final UI/fidelity/render checks/build underway. Actual live-window appearance remains for user relaunch. Existing unresolved Keychain prompt diagnosis retained.
