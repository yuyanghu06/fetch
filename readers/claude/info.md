# Claude conversation reader

`helper.py` wraps the official Claude Agent SDK 0.2.181, source commit
`22b8eb9c19adfa78759b79e22bd664a57843bb06` (MIT license).
Source: https://github.com/anthropics/claude-agent-sdk-python/tree/22b8eb9c19adfa78759b79e22bd664a57843bb06

The helper directly calls the SDK's `_parse_transcript_entries`,
`_build_conversation_chain` and `_is_visible_message` with supplied transcript
text. Listing uses its `_read_session_lite` and `_parse_session_info_from_lite`.
These private entry points are pinned deliberately; the SDK algorithms are not
copied or replaced. The wrapper hides generated context, compact summaries,
reasoning and tool-call inputs, and associates unchanged tool results with names.
It never calls Claude's query, login, model, discovery or CLI APIs.

Run `scripts/setup-claude-session-reader.py --root <Fetch data folder>/session-readers`
separately to install the pinned SDK into Fetch's own runtime directory. Opening
a session never installs packages. Runtime files live in
`<data-dir>/session-readers/claude`; the backend supplies the actual data folder,
including a custom `--data-dir`.

The native backend starts the helper with a deny-default macOS sandbox: only
its runtime/system files and read-only history roots are readable, and only a
private temporary work folder is writable. Network and desktop/Keychain IPC
are not permitted. Fixture tests must use synthetic transcripts.
