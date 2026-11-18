# Verify inline approval and disabled server keys

- Local timestamp / acting role / milestone: 2026-11-17T19:00:06-05:00 / Orchestrator / macOS preview
- Requested scope: Move the connection approval beside the app key, hide the unknown sign-in warning, and disable its key.
- Starting repository state: The icon fix and release preparation were pushed. The GitHub preview was pending.
- Work performed / changed files: Connect apps carries the completed request's app and project to its heading. Servers omits unknown sign-in and keeps the key disabled in that state. Updated the shared UI documentation and the widget checks.
- Decisions: Keep the existing notice text and restrict it to the approved app and selected scope. Keep the server key available for required and failed sign-in when server changes are allowed.
- Checks and evidence: Four focused native widget tests passed in 1.31 seconds. Independent review found a scope-switch issue, then accepted the corrected implementation and checked the passing test evidence.
- Resulting state / acceptance: The matching app shows the notice beside its key, including a project switch during save. Unknown server sign-in renders no warning and key clicks emit no action.
- Remaining work and handoff: Push these changes before tagging the verified Apple Silicon disk image. Live Dock appearance still awaits observation.
