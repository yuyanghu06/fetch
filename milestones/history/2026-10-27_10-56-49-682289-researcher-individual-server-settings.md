# Research individual MCP settings and credential references

- Local timestamp / acting role / milestone: 2026-10-27T10:56:49.682289-04:00 / researcher / mcp-mvp-architecture
- Requested scope: Determine core MCP settings support and actual maintained-server config/storage mechanisms; clarify key exposure in the agent-facing config API.
- Starting repository state: D28 directory layout selected; existing three-field PATCH draft unapproved; user clarified individual-server settings scope and raised key exposure. Read-only specialist, root sole writer.
- Work performed / changed files: Inspected current core spec, filesystem/Playwright/GitHub/memory documentation/manifests/licenses and Playwright config serialization plus central redaction. Researcher changed no files; root integrated report into research.md.
- Decisions: No universal core settings file or get/set API. Host launch config differs from server-specific settings and data. Reference/name is not key value. Recommend supported setting mappings, not arbitrary raw config paths; exact schema/permissions remain open.
- Checks and evidence: Primary source links and observed maintenance/license/dependency facts recorded. Central Playwright redaction corrected an intermediate handler-only inference; no confirmed credential leak claimed. No package installation, credential reads, services or application tests.
- Resulting state / acceptance: Research complete and inspected; enough evidence to revise the proposed scope without claiming universal support.
- Remaining work and handoff: Present findings and propose supported non-secret settings management through existing APIs; review storage/mapping/read/apply/auth/permission details before complete revised contract.
