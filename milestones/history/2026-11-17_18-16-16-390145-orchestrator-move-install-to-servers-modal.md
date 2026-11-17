# Move Install MCPs into the Servers modal

- Local timestamp / acting role / milestone: 2026-11-17T18:16:16-05:00 / Orchestrator / Servers install modal
- Requested scope: Place Install MCPs just right of Installed servers Refresh, remove its navbar entry, and replace the install page with a modal.
- Starting repository state: Install was a top-level page, and discovery results opened in a second surface over that page.
- Work performed / changed files: Updated src/ui/mod.rs, src/ui/components.rs and src/ui/project_scope.rs. The modal contains the destination, source path, file picker, discovery results and job progress. Modal errors use their own messages.
- Decisions: Use the blue primary button and keep Servers selected. Closing hides the modal while pending work continues.
- Checks and evidence: Inspected control placement, dismissal priority and the previous discovery and install request paths. git diff --check passed.
- Resulting state / acceptance: The standalone route and navbar button are gone. One modal replaces the flow.
- Remaining work and handoff: Check minimum-size controls, modal reopening and signed build.
