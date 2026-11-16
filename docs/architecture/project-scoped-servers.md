## November12 — implementation requested

User explicitly approves [the complete contract](project-connections.md).
Project scoping is implemented, fixture-tested, signed-built and independently
accepted. The older proposal below is historical context, not a competing
specification. Actual client/native acceptance remains manual.

# Project-scoped MCP servers — recommendation

November 12, 2026. Research and proposal. The user subsequently selected recording
server scope in its existing `config.json`. Exact fields, project selection,
API/connection behavior and implementation remain unapproved.


## November 12 — saved scope and obtaining the client's folder

Confirmed: store a server's scope assignment in its existing `config.json`.
This selects the storage location, not an exact schema or connection method.

The user asks whether Fetch can request the agent's operating folder. Older MCP
`roots/list` asks supporting client software for workspace folders; it does not
promise one current working directory. Claude Code documents its launch folder
plus additional allowed directories. OpenCode's current legacy connection answers
with its configured directory; shared connections disable roots. Roots are
deprecated in the modern MCP revision. A model running `pwd` and reporting it is
a separate model-chosen action, not automatic client connection metadata. Fetch's
own working directory is not the client's. Existing Fetch relay support does not
implement project selection or filtering from roots.

Recommendation: bind each connection to an explicitly selected project, using a
project-specific address or project ID in client configuration. Then match saved
server assignments against that project and existing approved access for both
lists and calls. Two connections can use different projects at once. Roots may
provide a suggestion where supported, but multiple roots must not silently enable
multiple projects. A launch helper reading its directory is another integration
to review, not a universal HTTP capability or approved transport change.

Whether projects receive all shared servers or choose a subset also remains open.
The original shared-plus-project behavior below is a recommendation. No detection
method, address, header, bootstrap tool or client edit is approved.

Primary implementation references: [Claude Code](https://code.claude.com/docs/en/mcp),
[OpenCode client](https://github.com/anomalyco/opencode/blob/dev/packages/core/src/mcp/client.ts),
[OpenCode shared connections](https://github.com/anomalyco/opencode/blob/dev/packages/core/src/mcp/connection.ts).

## November 12 — installation records and client override rules

The user asks whether installation would record project information in the project
and whether project MCP entries override global ones in Claude Code and Codex.
The proposed flow has two records: Fetch saves individual server scope in its
existing config.json (selected); the client's project settings hold one named
fetch connection selecting that project (still proposed, not implementation
approval or authorization to edit personal/project client files).

Claude Code CLI uses project `.mcp.json`; matching names use the whole entry from
local scope before project scope before user scope. Thus project fetch overrides
user/global fetch, but a private local fetch entry in ~/.claude.json for that
project wins over .mcp.json. Other server names remain loaded; this is not a rule
that all global servers disappear. Managed/other host policies may constrain it.
[Official precedence](https://code.claude.com/docs/en/mcp#scope-hierarchy-and-precedence).

Current Codex supports trusted-project `.codex/config.toml`, including named
mcp_servers tables, above user ~/.codex/config.toml; CLI overrides rank above
project layers. A project mcp_servers.fetch.url can therefore replace the global
URL value. Do not generalize Claude's whole-entry replacement rule to Codex's
other unspecified fields. Current official docs supersede the older personal
shared-MCP note claiming Codex has no project support; this research does not
change the shared sync script or any client configuration.
[Official MCP documentation](https://learn.chatgpt.com/docs/extend/mcp?surface=cli),
[configuration precedence](https://learn.chatgpt.com/docs/config-file/config-basic).

For Fetch the key is using the same client entry name, fetch, at both levels so
the project-selected connection takes precedence. Any unrelated directly
configured client MCP remains outside Fetch's server filtering. No actual
installation, project config write, login or runtime acceptance was performed.

## Everyday use

Add a project folder in Fetch, review its server definitions and install the
selected servers for that project. Connect the coding app using a Fetch address
for that project. That connection sees shared servers plus that project's
servers. A second project can run at the same time without changing the first.
The existing ordinary connection would see shared servers only.

For example, two repositories could each use a server named Database with
different settings. Calls from one repository must never reach the other's
server. Fetch must enforce this on calls as well as on the displayed tool list.

## Current source findings

- `src/discovery/mod.rs` reads global defaults or a user-selected file/folder.
  A supported project file can already be read explicitly, but the result has
  no project membership. Folder selection currently uses the existing home-style
  candidate rules; it is not a project search.
- `src/installation/mod.rs` matches source imports by source path and name.
  It does not include a destination project in that match.
- `src/authority.rs` supports target checks, but current OAuth callers are
  constructed with the all-targets grant. Project-aware authority is missing.
- `src/mcp/mod.rs` serves `/mcp`; saved sessions have owner and capabilities,
  without a project selection. An HTTP request does not supply the coding
  process's working directory automatically.
- Native server login/storage behavior remains server owned. Separate Fetch
  server records would not prove separate provider accounts or file confinement.

## Current disk layout — November 12 follow-up

Current servers form one installation-wide collection; no explicit global/project
scope field exists. Default development root is `~/Library/Application Support/Fetch-Development/`,
release root `~/Library/Application Support/Fetch/`; explicit data-root overrides
remain possible. Each record is `servers/<SHA-256 of server ID>/config.json`.
It contains identity, launch/connection settings, execution settings and internal
source association. Installed package files live separately under
`packages/servers/<installation ID>/`; default local server HOME is the record's
`home/` folder. Native services own their credential files; Fetch-held remote MCP
login grants and manually entered keys use macOS Keychain. Import stores Fetch's
own captured/prepared record; the original client config is not a live source
for each call. No user credential/state files were read for this explanation.

## Proposed first version

1. Save a local project ID, display name and absolute folder. Save each server
   as shared or belonging to one project. Keep existing records shared. Include
   destination project in import reuse, so import cannot accidentally reuse a
   different project's instance.
2. Read only the selected project's supported config inputs, using existing
   parsers where their formats match. Show source and destination before install.
   Start with reviewed Claude Code and OpenCode project formats; add others only
   after confirming their current format. Preserve explicit working directories;
   use the selected project folder as the proposed default for its local servers.
   Resolve supported relative paths according to the source client's documented
   rules rather than Fetch's own launch directory.
3. Give each project its own connection address, handled by the existing Fetch
   backend. Save that address in the coding client's project configuration using
   its normal project-over-global selection. There is no app-wide active project.
   The exact URL and OAuth resource/discovery changes require a complete contract.
4. Restrict each request to both the user's approved access and the selected
   project's shared-plus-project set. A project ID/address selects context; it
   does not grant access. Bind authenticated connections to that selection.
   Cover tools, resources, prompts, management operations, notifications and
   continuation/cancellation ownership. Default global access must not expose
   newly added project records.
5. Keep separate logical server instances and launch settings for separate
   projects. Preserve existing process/account/login rules and raw MCP data;
   do not rewrite the upstream server or invent per-project credential storage.
   Keep source config separate from Fetch's saved installation, with explicit
   reimport initially rather than automatic file watching or restart.

The UI would show All projects and a project filter, label each server's scope,
and offer the appropriate client connection snippet. Existing shared components
must be reused. Exact screens and APIs remain proposals for review.

Open decisions include same-name shared/project override behavior, project
removal/moving folders, and the supported client versions. They are not silently
settled by this recommendation. Cross-computer sync, automatic repository search,
worktree inference and changing provider credential ownership are outside the
suggested first version.

## Sources and compatibility limits

- [Claude Code MCP scopes](https://code.claude.com/docs/en/mcp): project `.mcp.json`,
  user approval and local/project/user precedence. Existing client scope rules
  can choose the project Fetch address without a new Fetch launcher. Client
  approval still applies; importing into Fetch does not transfer that approval.
- [OpenCode MCP documentation](https://opencode.ai/docs/mcp-servers/): current V2
  uses `mcp.servers`, `disabled`, workspace-relative `cwd`, and whole-server
  replacement by project definitions. Fetch's current reader uses the older
  top-level `mcp` entries, so current V2 compatibility is additional work.
- [MCP roots specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/client/roots.mdx):
  roots are informational, not access control, and deprecated in that revision.
  They should not be the foundation for selecting project authority.

The maintained [OpenCode implementation](https://github.com/anomalyco/opencode/blob/dev/packages/core/src/mcp/index.ts)
is an MIT-licensed reference for keeping connections separate by directory and
sharing only when configured. Its TypeScript MCP/Effect/Zod dependencies do not
justify adding another runtime to Fetch; reuse the design with Fetch's current
Rust code. Claude Code's public repository is not a corresponding open-source
implementation to copy. OpenCode's
[previous OAuth header issue](https://github.com/anomalyco/opencode/issues/20286)
was closed as fixed; it supports testing actual authentication/reconnect rather
than assuming a configured project selector reaches every authentication step.

No client/provider/runtime tests were run. Before calling implementation complete,
use the protected test environment with two disposable projects open together:
prove distinct tool lists and working directories, reject a guessed cross-project
call, keep shared servers usable, and retain project selection after reconnect
and Fetch restart. Include real client sign-in/reconnect because connection
address selection affects authentication too.

## Suggested build sequence

Review this behavior first, then present complete API/connection changes. Build
project records and common backend authorization before UI. Add project imports
and connection snippets next. Finish with focused isolated two-project client
checks and an independent evaluation. No implementation starts from this note.

## November 12 — commands from the repository

Checked installed `claude mcp add --help` and `codex mcp add --help`, without
adding a server. Claude supports `claude mcp add --scope project --transport http
fetch "SERVER_URL"`, writing the project .mcp.json. Its default is local, which
is private to this project and saved under ~/.claude.json, not the shared project
file. Codex supports project .codex/config.toml but this installed `mcp add` has
no project/scope output flag; do not claim running it in a repository makes the
write project-scoped. Use the existing project's [mcp_servers.fetch] table and
set url there, preserving other content. No project client file was edited.

The existing https://localhost:53672/mcp can be registered at project scope,
but that alone does not select project-specific upstream servers in Fetch.
The latter feature remains unimplemented. The proposed future project URL must
not be shown as an existing route. Connection registration is separate from
certificate trust and user authentication.

## November 12 — Codex version clarification

Installed CLI reports `codex-cli 0.167.1`. Rechecked add --help: no project/scope
flag. Current [official CLI reference](https://learn.chatgpt.com/docs/cli/reference)
describes `codex mcp` as managing entries in ~/.codex/config.toml and documents no
project output option. No supported release with such a flag was established;
do not imply upgrading fixes it or claim every historical release was audited.
Project config support and a project-writing add command are separate features.
A Fetch-owned single pasteable setup command could write the project entry while
preserving existing content, but no helper or exact behavior is selected/built.

The intervening Apple-signing question remains pending: Developer ID signs the
app; it does not supply an automatically trusted localhost HTTPS identity. No
certificate/trust/client changes were made while questions steered to CLI support.
