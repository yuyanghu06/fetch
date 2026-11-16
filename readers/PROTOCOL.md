# Internal helper protocol

One JSON request on stdin, one JSON response on stdout, then exit. stdout contains no diagnostic chatter. Backend deadline120s and response ceiling32MiB; errors return a nonzero status with useful error code/message/details or stderr. Never return complete:true with partial history.

Common fields: op (list/conversation), source_roots (trusted absolute canonical recorded-history roots), work_dir (private writable per-request directory). No auth/config/credential values passed. Original paths are read-only.

list also receives candidates, the existing session metadata records including file_path and current public row IDs. Return sessions with recognized original file paths/IDs and official top-level source selection, not internal subagent files. Preserve source metadata rather than inventing a current branch. Optional native index support must use trusted exact read-only inputs and official query logic.

conversation receives file_path (already validated original selected session). Helper uses pinned official parsing/storage/lineage on required private copies and returns messages plus complete:true. Each message has stable id, kind user/assistant/tool_result, text for prose or preserved output for tool results; tool_name/tool_call_id are included when native source supplies them. Tool inputs/reasoning/injected context are omitted from presentation; output strings are not rewritten.

Runtime layout under configured readers root: codex/fetch-codex-session-reader; claude/bin/python and claude/helper.py. Both setup scripts take --root pointing to that same all-readers root. Custom application data roots and fixture roots must remain supported.
