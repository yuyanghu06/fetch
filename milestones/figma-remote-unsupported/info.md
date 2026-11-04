# Figma Remote unsupported import

Goal: mark Figma Remote unsupported in Fetch’s import/install screen, as explicitly selected November 4, 2026.

Scope: existing shared discovery classification and per-row reason display. Fresh file/direct installations use that same decision. Preserve Figma Desktop and existing installed records, credentials and connections. No new API route or schema.

Completion: the remote row is disabled and explains the client-approval restriction; focused import/install checks and build pass, followed by independent review.

Status: complete. Classifier and row-reason display implemented; independent19focused checks (2new), locked native build5.38s, scoped whitespace and Evaluator ACCEPT. Existing parser restrictions remain. Native appearance unobserved; no app launch. Explicitly Quit Fetch, use ./scripts/run-development.sh and refresh imports to see the change.

[Final acceptance](../history/2026-11-04_18-07-32-270262-evaluator-accept-figma-unsupported.md), [completion](../history/2026-11-04_18-07-32-270263-orchestrator-complete-figma-unsupported.md).
