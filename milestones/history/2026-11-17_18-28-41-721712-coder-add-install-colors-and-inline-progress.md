# Add install button colors and inline status

- Local timestamp / acting role / milestone: 2026-11-17T18:28:41-05:00 / Coder / Servers install modal
- Requested scope: Use a white Install MCPs shortcut with black text, gray out the disabled footer action, and show loading beside each installing server.
- Starting repository state: The shortcut used blue. A disabled footer action kept its blue face. Installation replies appeared below the discovery list.
- Work performed / changed files: Moved the shortcut to the shared text-button style. src/ui/theme.rs supplies the disabled gray palette. src/ui/components.rs and src/ui/surface.rs draw the status label and activity bar. src/ui/mod.rs reads existing progress as soon as the job is accepted and joins results to rows by configuration path and server name.
- Decisions: Use a moving blue segment while installing. Queued, installed and failed follow the existing API statuses, which supply no percentage.
- Checks and evidence: git diff --check passed. The build signed on the first final run.
- Resulting state / acceptance: The requested visual changes are implemented. No API schema changed.
- Remaining work and handoff: Verify actual rendered colors, status placement and motion.
