# Popup escape keyboard checks

- Local timestamp / acting role / milestone: 2026-11-16T14:25:57.646261-05:00 / tester / workspace-session-error
- Requested scope: every current Fetch popup dismisses on Escape, including focused editor.
- Starting repository state: central event mapper/topmost Close dispatch implemented, signed build26.11s passing.
- Work performed / changed files: Tester adds two keyboard feature flows in workspace_tests.rs/neumorphic_tests.rs; minimal event_with_status exposes actual widget Shell result and production subscription mapper is called with the same event. No extra framework or screenshot matrix.
- Decisions: preserve real current close semantics and pending work; Captured mapper case checked explicitly, actual editor Escape uses Ignored Unfocus. Only current visible top popup closes.
- Checks and evidence:2cases pass1.79s; focused dirty draft/cache, no-popup no-op, layered approval/install, pending-state preservation/reopen and zero-backend dismissal tasks pass. Diff check passes. See ../workspace-session-error/escape-evidence/info.md.
- Resulting state / acceptance: focused keyboard verification passes, no findings.
- Remaining work and handoff: record final evaluation/docs; user native relaunch/keyboard observation remains untested. No live/personal-state actions.
