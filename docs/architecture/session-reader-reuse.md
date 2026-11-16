# Official session readers — approved contract

November13,2026. APPROVED: the user selected “Approve official readers” for this complete API/dependency plan. Private helper dependencies/setup and implementation are authorized. Implementation is complete with focused checks, final signed build and independent Evaluator ACCEPT. See ../../milestones/official-session-readers/info.md.

Fetch should show a whole selected conversation, with manual user input, assistant output and named tool results. The backend will use pinned official reader/storage code; the frontend will format returned messages. Starting a model conversation, login or MCP server is outside this operation. Original histories remain read-only.

## Native operations

Extend the existing private list operation with `GET /api/v1/workspace/sessions?view=conversation&app=codex|claude&folder_path=...&cursor=...&limit=...`. Only the `view` selector is new; the other current parameters retain their meanings/bounds. Omission of view preserves existing behavior. The new view follows each official reader's top-level conversation selection rather than displaying internal subagent suffix logs as ordinary sessions. The UI automatically requests all requested pages as needed; official titles/IDs/source metadata are retained. Stores not recognized by existing session discovery are outside this first change.

Add `GET /api/v1/workspace/sessions/conversation?file_path=...&folder_path=...`. file_path is required, from a recognized listed session; optional folder_path means the chosen project, omitted for Global. No body, offset, limit or public cursor. The backend validates canonical native history roots and chosen-project membership, reads/reconstructs the entire selected conversation through the official reader and drains any upstream pages. No Read more button.

Both operations require the existing native Settings capability. Bearer/MCP callers remain forbidden. Existing raw Workspace reads and project-bound HTTP/MCP history operations remain unchanged. No new global agent-access surface or client credential copying.

## Complete response

Conversation response has status, existing session metadata, messages, complete:true and next_steps:[] on success. Each message has stable id and kind (user, assistant or tool_result). User/assistant text blocks stay together in text. Tool results retain output as the actual string/JSON value and include tool_name/tool_call_id where supplied. Copy preserves strings and uses readable JSON for object output. No fabricated user turns, summaries, progress, truncation or extra copies. Tool-call inputs, reasoning and injected setup/context are omitted from presentation. The result contains all conversation records the client reader returns; missing source history is never invented.

Keep the native client's32MiB response limit, returning SESSION_CONVERSATION_TOO_LARGE(413) instead of apparently complete truncated history. Reader deadline120seconds (below existing150second native request deadline); kill/reap owned children and remove task snapshots on completion, error or cancellation.

Reuse current invalid-query/body, native-authority, path/scope and missing-session errors. New failures: SESSION_READER_UNAVAILABLE503 (helper missing), SESSION_READER_UNSUPPORTED503 (unsupported native format/version or missing required history), SESSION_READER_FAILED502 (actual reader failure), SESSION_READER_TIMEOUT504, and size413 above. Preserve useful upstream code/message/details with existing credential redaction. Error responses include next_steps and never claim completeness.

## Reader dependency choice

Recommended Codex direction is a small native helper using the pinned official storage/rollout/history code at rust-v0.167.2 commitce21072c90d32c40af7a55b60b2fb35c84a43a6a. Its complete projector/lineage reader is private upstream, so expose that existing entrypoint with a narrow, documented visibility wrapper; keep parsing/reconstruction unchanged. This is a pinned source/build dependency, not a replacement parser. Do not use the public legacy-only reducer for modern data. Use native source selection so internal subagent suffixes are not mislabeled as independent main conversations.

Process only the selected conversation's required snapshot files and a fresh private history database; original logs/settings/auth stay read-only. Resolve ancestry using the official history_base rules and enforce the existing session-root boundary when collecting required source files. Any required path remapping affects only the disposable copies. The helper can run official projection/rebuild operations on those copies; it must not resume a real Codex conversation, start a model/MCP, or initialize normal Codex settings/auth. Preserve license/provenance for pinned source and the small exposure patch. Runtime parity and unchanged-original hashes are acceptance requirements.

The installed Codex service is excluded from this design: its startup performs config/auth/managed-policy work beyond reading, and the strongly confined fixture fails before initialization. Public store migration also skips already-paginated copies, so merely copying a JSONL into a fresh database is insufficient. These failed paths must not be described as working support or used as a normal-HOME fallback.

Claude direction is official SDK0.2.181 at commit22b8eb9c19adfa78759b79e22bd664a57843bb06 in a Fetch-private Python>=3.10 environment. Call its supplied-text parse/chain/visibility helpers directly so original metadata is available for the requested visibility filtering. No Claude query/login or SDK file lookup is needed. Pin private helper API use and retain license/provenance. Setup dependencies separately; opening a session must not silently install them.

Setup was explicitly approved and remains separate from opening a conversation. Both readers are installed under the actual development data root's session-readers directory. Codex compiled on installed Rust1.94.1 with exact official websocket patches; no Rust toolchain upgrade or global Rust setting change was needed. Claude uses private Python3.12 with SDK0.2.181, its supplied-text reader verified against the pinned official source. scripts/setup-session-readers.sh --root <absolute-reader-root> prepares both explicitly; runtime opening never installs them. Custom data roots are wired into the service.

Codex listing consults the exact native current-rollout/name query through read-only source-index access when present, including exact allowed WAL/SHM reads; without an index it uses the official filesystem fallback. Conversation reading reconstructs the explicitly selected immutable file and required native ancestors, rather than silently switching its path. Original index/session bytes are never intentionally written.

## Acceptance

Use synthetic independent fixtures covering complete Codex modern/legacy history, parent/fork history and Claude parallel tool results. Compare with official reader output; all first-to-last messages and tool associations must survive. Verify original hashes/config/auth unchanged; no model/MCP calls; snapshot/process cleanup; exact old raw API bytes/project guards. UI cases cover complete auto-loading, close/reopen, collapsed results, independent expansion and exact Copy. Missing runtime/format/errors must remain visible.

## Finder reveal — separately approved

User separately approved POST /api/v1/workspace/file/reveal. JSON body file_path is required and folder_path optional. Require native Settings capability and reuse existing instruction/subagent path validation. Reveal the canonical file with macOS open -R;200status:revealed,file_path,next_steps:[]; retain existing400/403/404 path/input failures and return503WORKSPACE_REVEAL_FAILED on actual Finder failure. No file content changes or personal configuration edits.


## Final verification and remaining observation

Independent Evaluator ACCEPT. Seven actual native Codex cases, actual confined Codex/Claude backend cases, complete125KBClaude conversation via native API,30authority denials, old raw/source preservation, successful snapshot cleanup,10Workspace widget cases and tool-error presentation pass. Final signed build and strict verification pass. Rendered subagent, tool disclosures, editor/Finder and long-message ending captures inspected. See ../../milestones/official-session-readers/evidence/tester-native-reader-report.md. Forced timeout/cancel cleanup is source-reviewed, with no dedicated hanging/drop case; live WAL/SHM, personal histories, physical Finder and relaunched app behavior remain unobserved. No model/MCP/login/app/Finder was launched for this acceptance.

## November 16 — missing indexed file repair

The native index may retain a current rollout path after that file is removed. Conversation listing now skips that thread only for a missing-file (NotFound) result, allowing unrelated available conversations to load. The saved current selection remains authoritative: an older surviving file is never substituted. Other read failures, root guards, direct conversation reads and missing-ancestor failures are unchanged. No source/index repair or history writes occur.
