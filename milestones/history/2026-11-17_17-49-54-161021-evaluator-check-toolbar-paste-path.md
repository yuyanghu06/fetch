# Check which filename single-card Copy keeps

- Local timestamp / acting role / milestone: 2026-11-17T17:49:54-05:00 / Evaluator / Workspace agent actions
- Requested scope: Independently review the two clipboard formats and filename used by single-card toolbar Paste.
- Starting repository state: Both shapes used the shared parser. Single-card Copy obtained its filename from the resolved file read response.
- Work performed / changed files: Read the card discovery, file read, clipboard encoding and create-only save paths. No application files changed in this review.
- Decisions: Use the path on the selected card. The read response can resolve a file link to a different path and must still supply the text and metadata.
- Checks and evidence: Confirmed bulk Copy already retains the selected path and the native read resolves an approved link. A single copy would otherwise use the target's filename.
- Resulting state / acceptance: Requested a correction before accepting.
- Remaining work and handoff: Pass the original path into single clipboard encoding and verify with a disposable linked file.
