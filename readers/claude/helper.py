#!/usr/bin/env python3
"""Fetch display wrapper around the pinned Claude SDK's supplied-text reader.

No SDK discovery, CLI, query, account, configuration or model APIs are called.
The parent enforces OS confinement; this helper also validates source boundaries.
"""
import importlib.metadata
import json
from pathlib import Path
import sys

SDK_VERSION = "0.2.181"


class ReaderError(Exception):
    pass


class UnsupportedReaderError(ReaderError):
    pass


def source_path(value, roots):
    path = Path(value).resolve(strict=True)
    if not path.is_file() or path.suffix != ".jsonl" or not any(path.is_relative_to(root) for root in roots):
        raise ReaderError("The selected Claude transcript is outside the supplied history roots.")
    return path


def blocks(entry):
    message = entry.get("message")
    content = message.get("content") if isinstance(message, dict) else None
    if isinstance(content, str):
        return [{"type": "text", "text": content}]
    return content if isinstance(content, list) else []


def presentation_text(sdk, text, user):
    # These are explicitly generated context records, not manually typed input.
    # Apply only to users: an assistant may legitimately explain any such tag.
    if user:
        stripped = text.lstrip()
        if sdk._SKIP_FIRST_PROMPT_PATTERN.match(stripped):
            return ""
        if stripped.startswith(("# AGENTS.md instructions", "<environment_context>",
                                "<recommended_plugins>", "<INSTRUCTIONS>",
                                "<local-command-caveat>", "<local-command-stdout>",
                                "<system-reminder>", "<command-message>")):
            return ""
    return text


def conversation(sdk, text):
    entries = sdk._parse_transcript_entries(text)
    chain = sdk._build_conversation_chain(entries)
    names = {}
    for entry in chain:
        for block in blocks(entry):
            if isinstance(block, dict) and block.get("type") == "tool_use":
                names[block.get("id")] = block.get("name")
    messages = []
    assistant_positions = {}
    for entry in chain:
        if not sdk._is_visible_message(entry) or entry.get("isCompactSummary"):
            continue
        kind = entry["type"]
        uid = entry["uuid"]
        text_blocks = []
        for index, block in enumerate(blocks(entry)):
            if not isinstance(block, dict):
                continue
            if block.get("type") == "text" and isinstance(block.get("text"), str):
                visible = presentation_text(sdk, block["text"], kind == "user")
                if visible:
                    text_blocks.append(visible)
            elif block.get("type") == "tool_result":
                call_id = block.get("tool_use_id")
                result = {"id": f"{uid}:tool:{index}", "kind": "tool_result",
                          "output": block.get("content")}
                if call_id:
                    result["tool_call_id"] = call_id
                if names.get(call_id):
                    result["tool_name"] = names[call_id]
                messages.append(result)
        if text_blocks:
            message_id = entry.get("message", {}).get("id") if kind == "assistant" else None
            if message_id and message_id in assistant_positions:
                messages[assistant_positions[message_id]]["text"] += "\n" + "\n".join(text_blocks)
            else:
                if message_id:
                    assistant_positions[message_id] = len(messages)
                messages.append({"id": uid, "kind": kind, "text": "\n".join(text_blocks)})
    return {"messages": messages, "complete": True}


def handle(request):
    if importlib.metadata.version("claude-agent-sdk") != SDK_VERSION:
        raise UnsupportedReaderError(f"This helper requires claude-agent-sdk {SDK_VERSION}.")
    from claude_agent_sdk._internal import sessions as sdk
    roots = [Path(root).resolve(strict=True) for root in request["source_roots"]]
    if request["op"] == "conversation":
        path = source_path(request["file_path"], roots)
        if not any(path.parent.parent == root for root in roots):
            raise UnsupportedReaderError("Choose a top-level Claude conversation, not a subagent transcript.")
        return conversation(sdk, path.read_text(encoding="utf-8"))
    if request["op"] == "list":
        result = []
        for candidate in request["candidates"]:
            path = source_path(candidate["file_path"], roots)
            # The SDK scans only top-level transcripts under each project.
            if not any(path.parent.parent == root for root in roots):
                continue
            if sdk._validate_uuid(path.stem) is None:
                continue
            lite = sdk._read_session_lite(path)
            if lite is None:
                continue
            info = sdk._parse_session_info_from_lite(path.stem, lite)
            if info is None:
                continue
            item = dict(candidate)
            item["name"] = info.summary
            item["native_session_id"] = info.session_id
            result.append(item)
        return {"sessions": result}
    raise ReaderError("Unknown session reader operation.")


def main():
    try:
        result = handle(json.load(sys.stdin))
    except Exception as error:
        result = {"error": {"code": type(error).__name__, "message": str(error)}}
    json.dump(result, sys.stdout, ensure_ascii=False, separators=(",", ":"))
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
