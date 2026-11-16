# Global Codex specialists

User explicitly requested creating the missing global team. Saved ~/.codex/agents/{planner,researcher,coder,tester,evaluator}.toml and registered each under agents in ~/.codex/config.toml. Added the previously referenced ~/.codex/AGENT_WORKFLOW.md and an agents/info.md guide. The main agent stays Orchestrator; no second coordinator role. Models/reasoning inherit. Planner/Researcher/Evaluator use the supported read-only sandbox; writers inherit current sandbox and must obey assigned scope.

Validation: all five TOML files/registrations parse; fields checked against the official rust-v0.167.2 schema; installed codex-cli0.167.2 features list loads the config successfully. Semantic equality confirms unrelated settings unchanged. Private original-config backup kept under ~/.codex/backups with restrictive permissions; no credentials/config values printed or copied into this repository. Existing role/config files were not overwritten. No live agents/providers/MCPs launched for validation.

Status: global setup complete. Fetch's existing global .toml discovery includes these files on refresh. This does not claim the current already-running chat automatically reloads role types. No app source/build required for global setup.
