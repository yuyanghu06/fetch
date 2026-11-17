## November 13 — native presentation follow-up

The user selected automatic project registration on successful folder opening using the existing projects API, removal of Paste/manual registration controls, local Cmd/Ctrl+Z undo and shared Copy controls. Native session display is restricted to manual user input, assistant output and tool results. Injected setup/context, reasoning and tool-call input records are omitted from presentation; original session API output/log files remain unchanged. Codex manual input comes from user_message events. Multiple text blocks stay in one fully wrapped message card. Existing API contracts/permissions remain unchanged; global subagent source investigation found missing role files rather than a known source-reader failure.

# Workspace: coding-app files and session history

November 12, 2026. User-selected product behavior and authorized implementation.
The frontend wraps the backend APIs. User explicitly approves the private HTTP
route contract below and selects finishing Workspace first, leaving session MCP
unavailable until the separate project-connection feature is ready.

## Selected behavior

Workspace is one native Fetch page. With no folder selected, it shows global
instruction files, global subagent definitions and all retained sessions. With
an explicitly selected absolute folder, it shows only that project's files and
sessions. Claude Code and Codex have their own tabs. Session rows expand and
read additional recorded content in chunks.

Instruction and agent files remain in their existing native locations. Fetch
edits those files through native-only APIs; the UI does not discover, parse or
write files itself. Neither instruction-file nor subagent operations become MCP
tools. This is an explicit exception to the older blanket API-to-MCP rule.

Copy preserves an agent's original definition. Paste converts Claude Markdown
with YAML settings to Codex TOML, or the reverse, and opens an editable draft.
Name, description and instructions carry across. App-specific fields are flagged;
no silent conversion of tool permissions or automatic model equivalence. Fetch
gets model choices from the installed app instead of maintaining a model list.
Saving the draft changes only the destination. Copies are independent.

Session logs are read-only, at their original locations, with no Fetch archive.
Models can list lightweight names/IDs and read bounded chunks. They cannot read
instruction or agent files through Fetch. The project-specific MCP connection
must restrict both listing and direct reads to its bound project. Browsing a
different folder in Workspace never changes an agent's connection.

## Current implementation facts

The existing NativeClient sends a native-window capability over Fetch's pinned
HTTPS connection. Reuse the native-only guard from the server open-folder API.
Ordinary bearer grants must not authorize Workspace file/config APIs.

Current MCP serves only `/mcp`, rejects query parameters and does not bind a
project. Caller grants have no history/project authority. The separate
[project-scoped server proposal](project-scoped-servers.md) is not implemented.
Do not expose histories to that unbound endpoint or treat a model-supplied path
as project authority. Exact project connection and permission changes require
review before session MCP integration.

The checkout has substantial unrelated changes and active server on/off work.
Preserve it. No personal client registrations, real instructions, agent files,
credentials or session logs may be changed by development fixtures.

## Approved native HTTP contracts

All routes use Fetch's existing native Settings capability, reject ordinary
bearer/background callers with 403, return `Cache-Control: no-store`, and include
`status` and `next_steps` (`[]` when no human action is needed). File paths are
absolute. Missing `folder_path` selects global scope. A selected folder must
exist and be a directory. These operations do not start MCPs or run inference.

| Method/path | Inputs | Successful result and effects |
| --- | --- | --- |
| GET `/api/v1/workspace` | Optional `folder_path` | 200: selected scope and recognized instruction/agent file descriptors, including paths and app; no full file contents. |
| GET `/api/v1/workspace/file` | `file_path`, optional `folder_path` | 200: content of the recognized instruction or agent file in that scope. An authorized Codex agent read can also return `registered_description` from its matching configuration entry. |
| PUT `/api/v1/workspace/file` | JSON `file_path`, optional/null `folder_path`, `content` | 200: saves the recognized native file; agent contents must parse. Can create a destination agent definition. No arbitrary application/config/credential file editing. |
| DELETE `/api/v1/workspace/file` | `file_path`, optional `folder_path` | 200: removes the selected subagent definition after native confirmation, returns its path, app, kind and `deleted: true`. |
| POST `/api/v1/workspace/agents/convert` | JSON `source_app`, `target_app`, `content`, optional `target_model`, `fallback_name` and `fallback_description` | 200: converted draft and explicit unresolved settings, retaining the source. No disk write. |
| GET `/api/v1/workspace/models` | `app` (`claude` or `codex`) | 200: app-reported model choices and effort metadata. Codex reads its saved native catalog; Claude uses an initialization-only metadata subprocess. No inference or MCP startup. An unavailable list is an actual error, never a hardcoded fallback. |
| GET `/api/v1/workspace/sessions` | Optional `folder_path`, `app`, `cursor`, `limit` | 200: names/IDs/app/project/time/source paths and continuation; no message contents. Default 50 sessions, at most 200 per response. |
| GET `/api/v1/workspace/sessions/read` | `file_path`, optional `folder_path`, `offset`, `limit` | 200: exact bounded recorded UTF-8 text, byte positions, next offset and EOF. Default 32KiB, at most 64KiB. Verifies session source and selected project before reading. |

Errors use the existing application envelope with an actual code and message:
400 for invalid input, 403 for missing native authority or out-of-scope file,
404 for an absent file/session, 413 for an oversized editable file/body,
422 for an invalid agent definition, and 503 for unavailable model metadata.
Filesystem errors preserve the underlying useful error text without logging file
contents. Reads and conversion are synchronous; saves return only after the
write finishes. No hidden retry, background save or session-log modification.

The UI sends paths to these APIs. Global file paths are resolved from the
current home and configured app roots; no machine-specific username is baked in.
Project file discovery never mixes in global instructions or agents. Recognized
file destinations and canonical path containment are checked by the backend.

## Add subagent save

November 17. In the Subagents view, Add opens a blank draft for the selected app and folder. The filename and definition are entered before Save. The native UI uses the app's agent directory from Workspace discovery and the app's file extension. The existing clipboard converter can prepare definitions copied from Fetch in the selected target format. Opening, typing and pasting create a draft in memory.

For Add, `PUT /api/v1/workspace/file` sends `If-None-Match: *` with the existing `file_path`, `folder_path` and `content` JSON body. This asks the server to create a new agent file only. Native authority, path scope and content validation still apply. If the filename already exists, the server returns HTTP 412 with `WORKSPACE_FILE_ALREADY_EXISTS` and keeps the saved file intact. The UI keeps the draft open for renaming. A compact error sits beside Save and its info icon shows the returned message on hover. Success returns the existing 200 response. A condition other than `*` or a create request targeting an instruction file returns 400. Without the header, existing Edit saves use the prior behavior.

## Bulk subagent clipboard

November 17. Copy all reads the selected app and Workspace folder's listed agent definitions through the native file GET. The clipboard entry contains the app, relative filenames and the source definitions. Paste all uses the current destination's agent directory and the existing converter for cross-app copies. It submits an immediate create-only Save for each eligible definition. The UI refreshes the list and shows the result before the clipboard controls. Hovering the error's info icon reveals refused filenames, their returned errors and conversions that need individual review. Paste on a card uses the first definition to replace that selected file, then applies create-only Save to each remaining eligible entry. No bulk API route is required.

Toolbar Paste reads the current clipboard and accepts a single card copy or a Copy all bundle. A single card copy carries the relative filename and original definition. Older single copies can use their recorded card name as the filename.


## Delete a subagent definition

November 17. The red X on the card opens a confirmation with that definition's name and file path. Only confirming sends the native DELETE request. The API validates the same selected folder and existing agent path as the file editor, rejects instruction files, and returns the existing error envelope if the file is absent or cannot be removed. It removes the selected definition file. The Codex registration file is outside this deletion scope.


## Session reading and upstream reuse

Codex documents thread listing and paginated turn/item reads. Claude documents
lightweight `listSessions` and `getSessionMessages` with offset/limit. However,
the inspected official Claude Python SDK reads the whole file before slicing
messages. Pagination of a response is not proof of bounded file memory.

For the full original recorded stream, a bounded seek/read of the original JSONL
is a suitable thin file wrapper: preserve exact records, split even a huge single
record across requests, and return continuation offsets. Metadata discovery uses
bounded head/tail reads and existing title indexes, inspired by the official
Claude SDK. Do not present the raw recorded stream as a reconstructed, resumed
conversation. No new history archive, summary model or inference call. Current Codex paginated
history still retains source rollouts; its SQLite history is a rebuildable
projection. Compressed `.jsonl.zst` rollouts are streamed through zstd without
extracting a file. Offsets count decoded UTF-8 bytes; later compressed reads
re-decode the preceding bytes with bounded memory. Fork parent references remain
in the raw stream; Fetch does not reconstruct inherited branches.

The two planned MCP tools list session metadata and read a session by ID plus
continuation. They wrap the same backend and cannot accept arbitrary filesystem
paths or switch projects. Tool names, final wire contracts and project-bound
authorization remain pending; no unbound history exposure is a valid fallback.

## Models and definition conversion

Codex app-server supplies `model/list`, but source inspection found that starting
app-server can migrate or compress the original logs. Production Workspace
therefore reads Codex's existing `models_cache.json` instead, returning visible
raw model entries and fetched time while excluding identity/ETag/account data.
The UI labels this saved model list. It does not reproduce native cache age,
client-version or account-identity validation; it is not a live access check.
Missing cache returns unavailable and never starts Codex as a fallback.

Claude's documented Agent SDK exposes `supportedModels`, backed by initialization
response metadata. Fetch sends only initialization over the same JSONL protocol,
using the installed CLI in safe mode with empty strict MCP configuration, empty
tools and no persistence. No user message is submitted. A ten-second timeout,
2MiB output cap and owned-process kill/reap bound the operation. Safe mode skips
personal hooks/customizations; managed policy can still apply. Finder launches
can resolve the executable through PATH, the user's .local/bin or common Homebrew
locations. Do not use `--bare`: it changes OAuth/Keychain account behavior.
No real Claude catalog process was started during fixture verification.

Use existing TOML/JSON libraries plus a maintained YAML library for native Claude
frontmatter. Research selects serde-saphyr 2.1.1 (MIT OR Apache-2.0, Rust1.89),
without filesystem includes/property expansion. No custom YAML parser or LLM
conversion. Unsupported source fields remain visible as draft issues. For a cross-app copy, a missing or blank name uses the optional `fallback_name` from the source card. An explicit name takes precedence. A missing description can use the matched Codex registration provided as `fallback_description`. An explicit description takes precedence, and conversion still reports missing descriptions when no fallback exists. The selected Codex-to-Claude translation maps `sandbox_mode = "read-only"` to `permissionMode: plan`. Other unresolved settings still require review.

## Work and acceptance

1. Shared backend for recognized files, conversion and bounded session reads.
2. Reviewed native HTTP routes and real frontend wrappers.
3. Workspace page using existing navigation, theme, buttons, editors/disclosures
   and contained scrolling; folder chooser and explicit Global action.
4. Safe model catalog integration and destination draft model selection.
5. Session-only MCP is deferred by explicit user choice until the separately
   reviewed project binding exists. Do not add history tools to this build.
6. Focused disposable API/file/conversion/session fixtures;
   real-widget normal/minimum-window checks; independent evaluation and signed
   development build-only. Do not relaunch existing Fetch or use personal client
   accounts/files for tests. No release, commit/push or credential migration.

Acceptance proves native-only file routes reject agent grants; project/global
views separate files; cross-app converted drafts can be saved without touching
source; metadata lists omit message contents; repeated bounded reads reconstruct
the raw original, including huge records; and direct project reads reject another project's session. Session MCP acceptance
is deferred with that feature. Build success alone is not actual-client acceptance.

## Sources

- [Codex app-server](https://learn.chatgpt.com/docs/app-server): models and paged history.
- [Claude Agent SDK reference](https://code.claude.com/docs/en/agent-sdk/typescript): supportedModels, native CLI path, session metadata/messages.
- [Official MIT Claude session reader](https://github.com/anthropics/claude-agent-sdk-python/blob/38eec2b9474bd486c7bbe3d1554110a6afdaed9a/src/claude_agent_sdk/_internal/sessions.py): bounded metadata and whole-file message reconstruction.
- [Claude subagents](https://code.claude.com/docs/en/sub-agents): native formats and scopes.
- [Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents): native definitions.
- [serde-saphyr](https://github.com/bourumir-wyngs/serde-saphyr): maintained licensed YAML implementation.


November12 source compatibility correction: [Codex compression](https://github.com/openai/codex/blob/main/codex-rs/rollout/src/compression.rs), [history projection](https://github.com/openai/codex/blob/main/codex-rs/thread-store/src/local/thread_history_materialization.rs), [startup migration](https://github.com/openai/codex/blob/main/codex-rs/core/src/thread_store_builder.rs), and [model cache](https://github.com/openai/codex/blob/main/codex-rs/models-manager/src/cache.rs) establish the implementation above. zstd0.13 matches upstream's dependency range and uses the maintained BSD-3-Clause Rust wrapper. No model names are hardcoded.
