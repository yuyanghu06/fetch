# Fixed server-first MCP implementation

- Local timestamp / acting role / milestone: 2026-11-12T15:49:23.897444-05:00 / Coder / server-first-tools
- Requested scope: Accepted fixed3 discovery/call flow and complete startup inventory/instructions.
- Starting repository state: Plan/research accepted, performance source integrated, unrelated Workspace/Off/UI work preserved.
- Work performed / changed files: src/mcp/mod.rs, src/mcp/management.rs, narrow ManagementAdapter dispatch in src/api/runtime.rs and folder guides. Fixed-flow coder implemented permitted Fetch-owned catalog helpers, safe local installed inventory, complete startup server names/IDs in legacy initialize.instructions and modern DiscoverResult.instructions, and explicit model guidance repeated in the list-server tool description. MCP selected tool listing rejects omitted/empty IDs; direct HTTP defaults are retained. Generic call dispatch runs before the management text wrapper, checks original target permissions/usability, forwards raw inner arguments and retains outer continuation/meta fields through existing Runner/adapt_result. Existing management tools remain scoped; no connection-local catalog state or new route/header/auth behavior. Root inspected actual source. No new runtime checks yet; independent fixture authoring and source review are in progress.
- Decisions: Generic call approval difference already accepted. Modern instructions field verified in rmcp3.8.0 and primary schema https://modelcontextprotocol.io/specification/2026-07-28/schema#discoverresult. No upstream outputSchema advertised by generic caller.
- Checks and evidence: Source inspection/scoped whitespace pass; no Cargo/app/client/credential/trust/config operations by coder.
- Resulting state / acceptance: Source integrated; fixture/evaluation/build acceptance pending.
- Remaining work and handoff: Run focused isolated tests, evaluate/fix, signed build-only. Actual clients remain manual.
