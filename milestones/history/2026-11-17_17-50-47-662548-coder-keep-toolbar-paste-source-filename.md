# Keep the selected card's filename in the clipboard

- Local timestamp / acting role / milestone: 2026-11-17T17:50:47-05:00 / Coder / Workspace agent actions
- Requested scope: Resolve the filename issue found in independent review.
- Starting repository state: Single Copy took a resolved filename from the GET reply.
- Work performed / changed files: src/ui/workspace.rs passes the card path through Copy completion. src/ui/workspace/bulk.rs uses that path for the clipboard filename and uses the API response for text and metadata. Updated the affected Workspace source guides and architecture record.
- Decisions: Keep the copied card's actual relative filename, including when the card points to a link. Files outside the agent directory use their filename alone.
- Checks and evidence: Inspected the selected path flowing from the button into encoding. git diff --check passed.
- Resulting state / acceptance: Correction is ready for final checks.
- Remaining work and handoff: Run the disposable alias fixture, rereview and verify the signed build.
