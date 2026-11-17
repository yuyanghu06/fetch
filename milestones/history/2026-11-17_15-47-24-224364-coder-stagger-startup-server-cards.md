# Make server cards visibly pop in at startup

- Local timestamp / acting role / milestone: 2026-11-17T15:47:24-05:00 / Coder / Startup and Workspace polish
- Requested scope: Show each installed server card entering in list order at startup and center the empty list message.
- Starting repository state: The prior drawing wrapper showed every card immediately with a subtle 8% scale change. Workspace changes and test changes were in parallel, in other files.
- Work performed / changed files: mod.rs starts a one-time entrance when the initial list arrives. surface.rs starts a common clock on the first redraw, hides each card until its turn, scales the card from 80% with a small overshoot, clips the motion to the list, and restores normal drawing and input as it lands. auth_tests.rs initializes the new state for fixtures. The two Servers empty messages reuse the shared centered layout.
- Decisions: Stagger starts by 120 ms and let each card settle in 270 ms. Keep the existing manual reload animation and server list data flow.
- Checks and evidence: Reviewed source and ran git diff --check. Widget checks are running.
- Resulting state / acceptance: Source implemented. End-to-end drawing in the offscreen fixture and signed build are pending.
- Remaining work and handoff: Verify first, second, third and settled cards, manual reload, return to the Servers tab and the empty panel.
