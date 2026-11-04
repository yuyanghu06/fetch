# How MCP clients start server-owned login

Research checked November 3, 2026. Later in this session the user selected and
Fetch implemented the [start/login follow-up](../../milestones/standard-authenticate/info.md).
Descriptions of discarded output below record the source at research time. Findings and recommendations only; no new
API, package-specific handling or application changes are approved by this note.
No server/package was installed or executed and no personal account was used.

## Main finding

No reviewed library discovers the correct login command by reading arbitrary
installed MCP source or its instructions. Remote MCP authorization does have a
standard discovery flow. Local servers may instead use existing environment
values, start their own login during startup, expose an ordinary tool that starts
login, or require a separately documented command. Those behaviors are not one
universal helper-command interface.

There is a reusable standard for a running server to request that the person
visit a supplied login URL. This is called URL elicitation: the server sends the
link, the client presents it with permission to open it, and the server keeps
ownership of the account credentials. It requires support on both ends and does
not discover a command in an installed package.

Primary references: [MCP authorization scope](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization),
[URL elicitation and provider OAuth](https://modelcontextprotocol.io/specification/2025-11-25/client/elicitation).
The HTTP authorization specification recommends environment-provided credentials
for stdio and defines discovery for HTTP. Local servers can also own saved login
files; that recommendation does not prohibit them.

## Important correction about this Google package

The relevant package is `@a-bonus/google-docs-mcp` version1.11.3. Inspection of its
upstream tag shows two ways into its own existing OAuth code:

1. Normal local startup calls initializeGoogleClient, which calls authorize.
   With no saved token, authorize starts its own interactive login.
2. Its separate `auth` command calls runAuthFlow to force a fresh login even if
   a saved token exists.

Sources: [CLI/startup](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/src/index.ts),
[client initialization](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/src/clients.ts),
[authorization implementation](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/src/auth.ts).

That version's auth implementation creates the callback listener and Google URL,
then logs the URL. Its logger writes to stderr, the process's diagnostic output.
No browser-opening call is present in that inspected flow, even though its README
describes authorization as opening the browser. The server itself exchanges the
returned code and saves its own token. Client ID/secret configuration is still a
prerequisite; supplying those settings is different from completing account login.

Sources: [logger](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/src/logger.ts),
[setup instructions](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/README.md).

Earlier chat explanations overstated the need for a separate helper command for
initial Google login. The missing native_setup record explains why Fetch's current
Authenticate action cannot run a separate setup command. It does not mean Google
cannot start its login through normal startup, or that every local server needs
such a descriptor.

## What Fetch currently prevents

- `src/manager/mod.rs::authenticate_inner` requires native_setup_for on local
  Authenticate. Ordinary imports do not write that metadata.
- `src/manager/native_state.rs::run_setup` drains and discards both output streams
  from a recorded setup command. Recording Google's auth command alone would
  therefore still leave its printed sign-in URL unavailable to the person.
- `src/runner/transport.rs` drains and discards local server stderr. The Google
  startup flow's sign-in URL would also be lost on this route.
- `src/ui/mod.rs::authentication_view` presents generic folder/manual-key controls
  in applicable failure states; those controls do not execute Google account login.

These are source findings. The saved Google instance, its input values, its live
process and the cause of its particular runtime failure were not inspected.
Do not claim that an unseen Google URL was actually emitted in the user's run.
No recommendation to retain or expose arbitrary unfiltered diagnostic output is
approved here. Decide how setup output reaches the person before implementing it.

## How existing clients handle it

VS Code's local configuration supplies the startup command, arguments, environment
and optional working directory. Its built-in OAuth flow handles supported remote
servers; local prerequisite/setup follows the publisher's instructions. It also
provides a server output view for diagnostics. This is not package-code analysis
to infer a second login command.

Sources: [configuration](https://code.visualstudio.com/docs/agents/reference/mcp-configuration),
[server management and output](https://code.visualstudio.com/docs/agents/run/manage-mcp-servers).

OpenCode similarly defines local command/environment separately from remote URLs
and automatic remote OAuth. Its auth command manages configured remote servers,
rather than discovering arbitrary local helper commands.

Source: [OpenCode MCP documentation](https://opencode.ai/docs/mcp-servers/).

Claude Code documents supplying the command/arguments/environment from publisher
instructions, with its browser-based authentication for remote services. Cline's
implementation starts the supplied local command through the TypeScript SDK and
logs stderr; its OAuth provider handles HTTP/SSE services. OpenCode's local
transport also logs stderr and keeps a tail for startup failures. None of these
inspected paths infers a native login command from installed source.

Sources: [Claude Code MCP](https://code.claude.com/docs/en/mcp),
[Cline transport and OAuth selection](https://github.com/cline/cline/blob/main/apps/vscode/src/services/mcp/McpHub.ts),
[OpenCode local transport](https://github.com/anomalyco/opencode/blob/dev/packages/core/src/mcp/stdio.ts),
[OpenCode auth command](https://github.com/anomalyco/opencode/blob/dev/packages/cli/src/commands/handlers/mcp/auth.ts).
These are implementation references, not drop-in Rust libraries. Public repository
metadata checked by the Researcher identifies current unarchived projects:
OpenCode and VS Code use MIT; Cline uses Apache-2.0; the Google package uses MIT.
OpenCode uses an MCP client package and Effect process primitives; Cline uses the
TypeScript MCP SDK and its own remote OAuth manager.

## Reusable libraries and packaging checked

| Project | What it provides | Maintenance, license and limitations checked |
| --- | --- | --- |
| [Official Rust SDK/rmcp](https://github.com/modelcontextprotocol/rust-sdk) | MCP messages and standard remote OAuth, including discovery and renewal; child-process transport is separate. Already selected by Fetch. | Workspace3.8.0; public metadata shows November3 activity; Apache-2.0. Auth dependencies include oauth2/reqwest. Not native helper discovery. An [issuer default-port issue](https://github.com/modelcontextprotocol/rust-sdk/issues/1420) is reported against3.8.0; not reproduced here. |
| [mcp-remote](https://github.com/punkpeye/mcp-remote) | A local stdio bridge to a remote OAuth MCP endpoint. | Version0.1.38; November2 activity; MIT. Includes MCP SDK, HTTP listener/browser-opening/network dependencies. It does not start an arbitrary local server's provider login. An [open CIMD compatibility question](https://github.com/punkpeye/mcp-remote/issues/382) is unresolved, not proof of unsupported behavior. |
| [MCPB](https://github.com/modelcontextprotocol/mcpb) | Publisher-supplied package startup/configuration manifests, loaded by clients such as Claude Desktop. | Version2.1.2; last push May26, less recent than the other candidates. Dependencies include Zod, fflate and node-forge. Package metadata says MIT, while LICENSE describes Apache transition with legacy MIT contributions; check exact distribution before copying. Latest [schema0.4](https://github.com/modelcontextprotocol/mcpb/blob/main/src/schemas/0.4.ts) has startup command/args/env, entry point and user inputs, no native login-command/state-folder discovery contract. |

Dependency/license references: [rmcp manifest](https://github.com/modelcontextprotocol/rust-sdk/blob/main/crates/rmcp/Cargo.toml),
[Rust workspace](https://github.com/modelcontextprotocol/rust-sdk/blob/main/Cargo.toml),
[mcp-remote package](https://github.com/punkpeye/mcp-remote/blob/main/package.json),
[MCPB package](https://github.com/modelcontextprotocol/mcpb/blob/main/package.json),
[MCPB license](https://github.com/modelcontextprotocol/mcpb/blob/main/LICENSE).
The [reported MCPB configuration restart issue](https://github.com/modelcontextprotocol/mcpb/issues/304)
is another unverified compatibility report, not a demonstrated Fetch defect.

The [official TypeScript SDK URL-elicitation examples](https://github.com/modelcontextprotocol/typescript-sdk/blob/main/docs/servers/elicitation.md)
show reusable server-supplied browser requests. This is the closest standard
general solution for provider sign-in, but an existing upstream must implement it;
installing a client library does not convert stderr links into that protocol.

## Recommendations awaiting user selection

Keep the already selected official Rust library for remote MCP OAuth. For local
servers, first preserve their supported normal startup and setup behavior, and
provide the person the setup interaction they actually require. Support the
standard server-supplied login-link flow where the upstream implements it. Use an
explicit documented setup command only where needed, rather than making it a
prerequisite for every local server.

For this Google package, completing initial sign-in needs its client settings and
a visible link from its own existing login flow; forced reauthorization can use
its documented auth command. Google continues owning callback, token exchange,
storage and renewal. This recommendation does not select new public fields,
automatic output parsing, new tool calls or token migration.

Real acceptance must connect a dedicated account in the protected environment,
use a Google tool, restart Fetch and check saved-login reuse. Research and source
inspection do not establish that behavior.
