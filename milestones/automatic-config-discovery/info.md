# Automatic configuration discovery

Goal: opening Install finds standard Claude Code, Codex and OpenCode configuration files without asking the user to locate each file.

Scope: automatically read known config paths through the existing discovery API; make file/folder overrides explicit and optional. A home folder uses its standard children; a `.codex` folder uses its own config.toml and its parent’s Claude/OpenCode paths. Keep selected-server installation separate.

Completion: default and folder/file discovery fixtures, independent UI message checks, native build and independent review pass. Preserve read limits, authorization, grouped source failures, source-disabled choices, secret suppression and read-only discovery.

Status: implemented; 47 relevant independent fixture cases and native build pass. Independent Evaluator ACCEPT; scoped source/fixture work complete. Native visual interaction and updated Figma specimens remain pending. No native UI interaction or real MCP installation is part of this focused pass. Existing wider MVP work remains pending.

Delivery follow-up: origin/main advanced with installed-row/direct-install/deletion/auth changes. A normal merge preserves them alongside folder discovery. Integrated independent checks pass: 64 relevant cases (12 new) and native build. Independent merged Evaluator ACCEPT; source/fixture scope complete. Normal delivery follows.
