# Accept the installed-row progress rendering

- Local timestamp / acting role / milestone: 2026-11-17T18:28:41-05:00 / Evaluator / Servers install modal
- Requested scope: Review the control styles, status mapping and render evidence.
- Starting repository state: The color and per-row progress test had passed.
- Work performed / changed files: Reviewed the exact matching key, unchanged install API and polling, and visible-row animation. No edits in this pass.
- Decisions: Only a visible installing row schedules animation redraws. Closing the modal preserves the existing job.
- Checks and evidence: Independent review confirmed git diff --check and inspected the focused test and offscreen evidence.
- Resulting state / acceptance: ACCEPT.
- Remaining work and handoff: Use the signed build after reopening Fetch.
