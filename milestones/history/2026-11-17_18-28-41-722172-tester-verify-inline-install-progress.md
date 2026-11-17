# Verify install colors and progress on the matching rows

- Local timestamp / acting role / milestone: 2026-11-17T18:28:41-05:00 / Tester / Servers install modal
- Requested scope: Check the rendered button colors, installation status placement and bar motion.
- Starting repository state: Both button colors and inline progress were implemented.
- Work performed / changed files: Added one focused widget fixture in src/ui/neumorphic_tests.rs. It uses two configuration paths with the same server name and controls progression with fixture replies.
- Decisions: Measure the actual widget colors and two timed renders. Use widget clicks to exercise the setup and install controls.
- Checks and evidence: The focused color and progress test passed in 2.20 seconds. The existing standard and minimum size modal test passed in 1.49 seconds. The fixture checked disabled click gating, immediate polling, the correct queued and active rows, movement, terminal states and the failure message.
- Resulting state / acceptance: PASS.
- Remaining work and handoff: No real server install was run.
