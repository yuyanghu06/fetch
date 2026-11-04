# Codex startup environment reuse

User approved replacing Fetch's environment assembly with actual Codex code on November 4, 2026.

Scope: adapt the pinned MCP environment builder and variable lists without importing the entire Codex crate. Preserve private per-server HOME/XDG folders, helper PATH calculation, assigned settings, command/arguments/working directory, `env_clear()`, service-owned credentials and authentication. No public API or schema changes, new dependencies, live account sign-ins or personal configuration edits.

Completion requires independent tests of inheritance, override priority, certificate paths, blocked internal variables, private folders/helper discovery and a real confined disposable child; a native build; independent evaluator acceptance; and source/license attribution. Research and planning are complete. Source is implemented; independent confined/helper checks32/32 and native build pass; Evaluator ACCEPT. Scoped replacement is complete. Full library tests are blocked by separate concurrent terminal test wiring and remain a tracked followup. See [evidence](evidence.md).
