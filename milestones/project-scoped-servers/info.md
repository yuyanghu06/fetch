# Project-scoped servers

Goal: let coding clients use shared MCP servers plus servers assigned to their
own project, including two projects running together.

Status: November 12, 2026. User selected saving scope in the existing server
`config.json`; remaining schema/connection behavior and implementation unapproved.
See [the proposal](../../docs/architecture/project-scoped-servers.md) for source
findings, primary sources, suggested build order and unresolved choices.

Scope proposed: project records, scoped imports, per-project connection addresses,
common backend permission checks and minimal UI using shared components.

Completion would require approved contracts, implementation, and protected tests
with two actual client projects proving tool visibility, working directory,
cross-project denial, shared access and reconnect/restart. Nothing here claims
those checks have run. Existing acceptance work remains separate.

Independent read-only evaluation ACCEPTs the recommendation's source accuracy and
limits. Product/API approval and implementation remain pending.

The proposal also records the current installation-wide disk layout, checked
from source during the global-storage follow-up. No project scope field exists.

Optional workspace-folder replies do not establish one active project. The
folder-detection follow-up and client-source evidence are in the proposal.

Current official Codex and Claude precedence rules and proposed installation
record locations are checked in the proposal; no client files were changed.

Installed project-registration command support is recorded in the proposal;
CLI help only was executed, with no configuration writes or connections.

Codex 0.167.1/current-docs project-add version clarification is recorded; no
newer supporting version or implemented Fetch setup helper is claimed.
