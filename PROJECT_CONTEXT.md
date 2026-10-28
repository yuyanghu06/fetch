> **Current authorization — October28, 2026:** The user authorized starting implementation and pushing to `https://github.com/yuyanghu06/fetch.git`. Do not create a release. Main MCP connection tests are deferred to a later isolated virtual environment using the user’s server inventory. This supersedes earlier implementation/push pauses only; it does not authorize personal-client edits, certificate trust changes, credential migration or live MCP tests.

# Fetch — current project context

Updated October28, 2026 after D73. Read [the single implementation checkpoint](docs/architecture/build-readiness.md), [confirmed decisions](docs/architecture/decisions.md) and [current checkpoint index](docs/architecture/open-questions.md). These supersede older proposal/status summaries. Detailed dated notes remain in [NOTES.md](NOTES.md); the [pre-D73 context](milestones/mcp-mvp-architecture/context-before-d73.md) preserves the earlier record.

## Current release

- One local macOS Fetch application with a Rust backend; UI, API and managed-server controller share one Fetch process. Iced is the previously chosen framework to try, not a validated prototype; exact compatible dependencies belong in one engineering brief.
- This first release is MCP management/sharing. Shared memory, prompt/history/undo and their interfaces are deferred; they remain future functionality in the same product. PostgreSQL/retention, graph, extra server processes, native background Tasks and live per-resource content remain deferred at their recorded scopes.
- Each agent configures one Fetch MCP at `/mcp`, stores its own Fetch credential/token and reuses the approved renewable connection. No Fetch account or per-tool provider-token exchange.
- The user completes each installed provider/server's authentication. Fetch reuses assigned valid credentials; ordinary tool calls do not start login or repeat provider verification. Preserve current caller permissions and truthful missing/expired/provider access guidance.
- Local HTTPS and the presented certificate/trust direction are selected. Standard app authorization, separate optional management permission and native Settings-only manual secret entry with Mac confirmation are selected at D71/D72's shown scopes.
- Per-server encrypted files and Keychain-held keys live outside replaceable packages. Ordinary non-secret records contain references. Provider credentials never enter agent responses. Exact cipher/Keychain mode/header/bootstrap/file mechanisms remain engineering candidates, not claimed historical approvals.
- Raw Tool JSON, model argument objects and upstream results are preserved. The shared runner routes stdio/HTTP/SSE and selected native methods with backend API authority, ownership and approved queue/deadline/cancellation/restart behavior.
- Opening Fetch starts managed local servers; background/window close keeps services running; explicit Quit stops managed children. Package changes are explicit, with pinned preparation and no credential migration or automatic personal-client cleanup.
- Distribution is a drag-and-drop DMG with Fetch.app and an Applications shortcut. License: MIT. Actual signing/notarization/publication and destructive data actions need their operational authorization when performed.

## Current work and permission

The user asked to stop circling back and to record one complete step with everything needed together. D73 replaces the ten-group review count with one five-stage API-first build sequence and one engineering worklist. Rust/execution/login/storage direction are settled, not new broad approval topics. New APIs or meaningful behavior changes still need a complete review; ordinary internal choices are grouped rather than asked individually.

The one remaining product-scope question is a new Fetch-owned GitHub CLI adapter. Recommendation is to defer it; importing an existing GitHub MCP is a distinct compatibility task. Optional unselected management controls are not silently added. The unaccepted credential-verification exception draft is withdrawn from active review; D25/D31's success gate remains.

D74 now authorizes starting implementation and pushing to origin, without a release. The first Rust developer-backend increment exists: ephemeral local HTTPS, protected read handlers, safe discovery and private/encrypted storage primitives. Native approval/token issuance, Keychain, package execution, desktop UI and MCP sharing remain incomplete. Real MCP tests are deferred to a later joint isolated-environment session; personal clients, trust and credentials remain untouched. The earlier October25 draft stays removed; this is new source, not a restoration.

Repository: `/Users/yuyang/Documents/fetch`, branch `main`, origin `https://github.com/yuyanghu06/fetch.git`. Preserve the existing personal MCP gateway/configurations and project tool state. Notes use existing NOTES.md; architecture decisions belong in docs/architecture, and actual pass history stays in milestones/history.

## Pre-build MCP session audit — October28

[The full source audit](docs/architecture/mcp-session-audit.md) checks all seven available Fetch conversations. It restored private runtime/setup/UI/cleanup/source-independence and concrete dispatch/streaming/catalog acceptance requirements that the short plan compressed, and the missing D68 execution input on the config management tool. No new product scope or application implementation was authorized. The consolidated plan remains current.
