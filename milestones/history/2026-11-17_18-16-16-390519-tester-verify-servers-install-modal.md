# Test installation from the Servers modal

- Local timestamp / acting role / milestone: 2026-11-17T18:16:16-05:00 / Tester / Servers install modal
- Requested scope: Verify actual clicks and modal behavior using fixtures.
- Starting repository state: The form and results render as one modal without changing the Servers page.
- Work performed / changed files: Updated the existing widgets and state fixtures in src/ui/neumorphic_tests.rs, src/ui/auth_tests.rs and src/ui/project_scope_tests.rs. Test events exercise the new entry, picker error, discovery, selection, installation, outside click, Escape and reopening.
- Decisions: Use the existing navigation, scope and installation fixtures with simulated replies.
- Checks and evidence: Eleven focused tests passed in 4.64 seconds. Offscreen captures confirm the button location and the chooser and results layouts at the 760 by 520 minimum size.
- Resulting state / acceptance: PASS. The selected project, checkbox faces and install footer fit. The existing pending job reappears after reopening.
- Remaining work and handoff: Live picker and installation calls were outside the fixtures.
