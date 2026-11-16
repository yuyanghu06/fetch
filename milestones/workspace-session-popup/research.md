# Session display research — November 12, 2026

The UI receives raw chunks from existing Workspace APIs. It must not open personal session files itself or replace the underlying history format.

Primary format references:

- [Anthropic SDK session reader](https://github.com/anthropics/claude-agent-sdk-python/blob/main/src/claude_agent_sdk/_internal/sessions.py): user/assistant message records carry role/content; text, tool_use and tool_result content blocks form visible messages. Metadata stays out of the conversation. Official project is MIT and actively maintained (November12 activity checked).
- [OpenAI Codex protocol models](https://github.com/openai/codex/blob/main/codex-rs/protocol/src/models.rs): response_item message payloads contain user/assistant input_text/output_text; function/custom tool calls and outputs are separate records. event_msg user_message/agent_message can duplicate these; display them only as fallback when canonical response messages are absent. Official project is Apache2.0 and actively maintained (November12 activity checked).

Compared viewers:

- [SakuyaInazaki/codex-session-viewer](https://github.com/SakuyaInazaki/codex-session-viewer): MIT, standard-library Python, no runtime dependency stack; last pushJune9 and zero open issues when checked.
- [jaydenk/SessionViewer](https://github.com/jaydenk/SessionViewer): MIT, FastAPI/Svelte/SQLite stack; last pushFebruary23 and zero open issues when checked.

Neither viewer showed recent maintenance comparable to the official SDKs. Reuse the established record shapes rather than add another viewer stack. No third-party source was copied and no dependency was added. The official Anthropic reader reconstructs parent/compaction chains; known public issue1411 concerns missing compaction-preserved records. This UI intentionally displays loaded records in chronological order and does not claim branch reconstruction.

Native picker: a disposable hidden AppKit profile first measured612.08ms first allocation and137.62ms second new allocation. The maintained hidden reuse fixture then measured988.46ms construction versus0.0000ms cached retrieval, same object, no presented panel. Move preparation away from the click; actual live presentation timing remains unobserved.
