# Project connections acceptance

Approved contract: [project-connections.md](../../docs/architecture/project-connections.md).
Independent Evaluator: **ACCEPT**. Signed development build-only and strict
signature verification pass22.95s. Fetch was not launched.

Final checks (repeated reruns are not separate acceptance cases):

- Core2 + project HTTP/MCP4 + server-first4 + Off3 + Workspace6 pass.
- Project-filtered unit8 pass, including actual widget controls and approval.
- Connector20, native OAuth4, existing browser1, activation1, popup6 and navigation1 pass.
- Final extended project4 rerun passes0.36s; project history refresh/reopen/revoke1 passes0.10s.
- Repository diff checking passes. Full named-check inventory is in [verification.txt](evidence/verification.txt).

The project scenarios prove Shared+own visibility and guessed-target denial;
installation scope/reuse/cwd/progress; stale callers rejected after assignment;
busy active+queued reassignment rejected; bounded project histories and source-path
metadata omission; exact URL and legacy session replay denial. Native/browser
approval, refresh/resource retarget rejection, profile separation and saved
authorization reopen/revocation use fake keys and disposable folders.

Initial failures were missing new Bridge fixture fields, task roots created with
755 instead of required700, and a configured synthetic connector peer missing its
durable Shared record. Fixtures were corrected; access/storage checks and original
upstream-result assertions were retained. Full initial logs were overwritten by
retries; those observed failures and corrections are recorded in pass history.
Review also fixed hidden IDs in implicit inventory and post-await metadata races,
including server-specific guidance. Compiler warnings remain.

Inspected native widget renders: [Workspace registration](evidence/workspace-project-874x630.png),
[Shared filter](evidence/project-server-shared-874x750.png),
[project installation](evidence/project-install-a-874x750.png),
[project connection](evidence/project-connect-a-874x750.png),
[project approval](evidence/project-history-approval-874x750.png),
[minimum approval](evidence/project-history-approval-minimum-760x520.png).
These are offscreen widgets, not an observed running Fetch window.

No live Fetch/coding clients/provider accounts, personal files/configurations,
credential or trust changes, release, commit or push. Real Keychain approvals and
client setup remain manual acceptance. Project-specific notifications/completions
were reviewed in source without dedicated runtime scenarios. The test environment
used task-owned synthetic peers; no executable MCP package or personal server ran.
