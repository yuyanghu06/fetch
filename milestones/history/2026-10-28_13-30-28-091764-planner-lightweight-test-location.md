# Plan the lightweight test location

- Local timestamp / acting role / milestone: October 28, 2026, 13:30:28.091764 America/New_York / Planner report serialized by Orchestrator / isolated-mcp-testing.
- Requested scope: User wants short standard behavior/install/auth checks, location definition first and representative server selection/recording later.
- Starting repository state: main at 1fae5b6 with preserved uncommitted documentation; no test environment provisioned.
- Work performed / changed files: Read-only specialist reviewed existing testing proposal, CLI, developer setup and ignore rules. No specialist changes; Orchestrator owns all documentation.
- Decisions: Recommend external root ~/Library/Application Support/Fetch-Testing/. Place replaceable packages and durable servers below fetch/ to mirror the selected alternate data-root layout instead of inventing a new package-root option. No server selection or runtime action.
- Checks and evidence: Existing --data-dir and --port inspected; installer/auth/MCP sharing remain unfinished. Reviewed private raw state versus redacted project evidence and credential/Keychain/browser caveats. Root existence checked by Orchestrator: absent.
- Resulting state / acceptance: Bounded documentation plan inspected. Path is a proposal; short test scope/sequence comes from the user. Folder separation alone is not an isolation guarantee.
- Remaining work and handoff: Present location/layout, then choose and record the server list in the later requested step. Verify concrete launch/access/auth boundaries before operational tests. Specialist spawned no agents and changed nothing.
