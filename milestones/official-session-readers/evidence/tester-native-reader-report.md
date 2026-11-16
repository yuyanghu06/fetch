# Independent native conversation test report

November 13, 2026. Acting role: Tester. Owned changes: one focused conversation API test in `tests/workspace.rs` and the `tester-*` evidence files in this directory. Existing native-authority denial attempts were preserved. No application implementation, personal transcripts, credentials, client configuration, app startup, model/MCP call, or real Finder action was performed.

## Observed results

All final commands exited 0:

- `python3 readers/codex/tests/fixtures.py "/Users/yuyang/Library/Application Support/Fetch-Development/session-readers/codex/fetch-codex-session-reader"` — seven focused real-reader cases passed. Evidence: `tester-codex-native-fixtures.txt`.
- `cargo test --test workspace native_conversation_api_lists_a_whole_claude_session_and_preserves_raw_history --offline` — one actual router/API case passed, 4.90 seconds after compilation. Evidence: `tester-native-conversation-api.txt`.
- `cargo test --test workspace ordinary_bearer_and_background_requests_cannot_access_workspace_files_history_or_models --offline` — one authority case passed, 0.06 seconds. Evidence: `tester-native-authority.txt`.
- `cargo test --test workspace native_workspace_apis_edit_the_fixture_file_convert_agents_and_read_session_chunks --offline` — existing native file/agent/raw-history/model-cache case passed, 0.03 seconds. Evidence: `tester-existing-workspace-api.txt`.

The new API fixture uses the installed private Claude reader through the actual native routes and macOS sandbox. It lists only the chosen project's session, returns the SDK's manual-input title, then reads the whole two-turn conversation. It checks the exact five ordered records: user, named tool result, full assistant text, user, assistant. The assistant text exceeds 125 KB and includes paragraph breaks and Unicode; equality proves the ending survives. Tool name, call ID, and multiline output remain exact. Generated setup is absent from messages. `complete` is true, the old raw list/read still work, original first 64 raw bytes are exact, and the source SHA-256 remains unchanged. An unrelated selected project receives 403. This test's reader snapshot directories are gone before the responses finish.

The authority case exercises all ten existing/new native operations for no capability, an ordinary bearer grant, and a background token. All thirty attempts return 403, including the conversation view/list, complete-conversation read and Finder reveal. The ordinary grant deliberately has all app scopes, proving those scopes do not replace native-window authority. The original fixture instruction file remains unchanged.

The seven Codex cases use only disposable synthetic logs and an independent disposable SQLite index. They prove complete legacy history and long assistant text; 140 modern turns plus exact named tool output (281 records across native pages); inherited fork history and its cutoff; exclusion of internal subagents; native index selection instead of a stale duplicate; unchanged original log/index hashes; and an explicit missing-parent error instead of partial success. The script invokes the actual helper directly on synthetic files. Separate backend cases prove the production sandbox path.

## Reused backend evidence

Inspected the backend fixture code and its reported output rather than duplicating its tests:

- `backend-readers-tests.txt`: all three actual confined Codex/Claude/service tests passed, 1.24 seconds. Codex returns all 280 messages from 140 modern turns. Claude preserves parallel Read results, removes unused forks/injected setup, keeps a long final reply, preserves source hashes and old raw bytes, and rejects a different project. The sandbox probe permits original reads/private work writes and denies original writes, unrelated reads and network.
- `finder-reveal-seam-test.txt`: one fake-Finder case passed, 0.05 seconds. It checks `-R`, authorized file selection, unchanged content, 404 missing and 403 outside scope. No real Finder launch occurred.

## Source review and limits

The production supervisor clears the environment, uses private HOME/TMPDIR and a deny-default macOS profile, owns a child process group, and retains its private work directory until kill/reap completes. The 120-second deadline and request-cancellation branches both run group kill, child kill/wait, then remove snapshots. Successful cleanup is observed by the API test; forced deadline/cancellation cleanup was reviewed in source, not exercised with a slow or artificial reader.

The Codex helper reads native source metadata and creates its history database only in copied snapshots. It does not initialize normal Codex ConfigLoader/Auth or start a model/MCP. Optional source index access uses a narrow official query exposed with SQLite read-only/create-if-missing-false options and exact index/sidecar read permissions. The index hash case has no live WAL/SHM sidecars; preservation of active WAL/SHM contents was not separately exercised.

Conversation `session.name` retains the old raw discovery metadata, while the new conversation list name comes from the official reader. In the injected-context fixture the old metadata name contains the setup prefix, but the official list title and all visible conversation records are correct. This matches the approved existing-metadata response and current UI's list-based title; the mismatch was reported to backend and Evaluator for awareness. No implementation change was made by Tester.

Two initial test-only corrections preceded the final passing API run: compare macOS `/var` and `/private/var` paths after canonicalization, and check generated setup exclusion in message records rather than all preserved session metadata.

No broad test suite, real personal history acceptance, signed app launch, or physical Finder behavior was tested in this pass. Existing compile warnings remain; all focused final commands pass. Original disposable fixture/source directories clean themselves up, and no test process is retained.

## Tested source identity

- `tests/workspace.rs`: SHA-256 `eb57618797757b9a23fa0b72696897ed1a3ef8ddf55fcf3e82e9fc5a1d491a7d`.
- `readers/codex/src/main.rs`: SHA-256 `e54adb83e0438f1efaa78ef933f258d1c4220016bffd40e87fb4ebc17fb893b0`.
- Repository and installed `readers/claude/helper.py`: SHA-256 `d922ceadbabb21f58513f12f01cb806a6bdae6e8acc48b52337fba1acf0a0ede`.
- Actual installed Codex reader executable: SHA-256 `c99a31f5a1f1320dde469eae583197545cc1167e034b6a91d3d5a8135ff0eb67`.

Result: PASS for the assigned native/API, authority, unchanged-source/raw-history and complete-reader fixture checks. Runtime forced-cancellation/deadline, live index sidecars and personal-data/UI acceptance remain the specific limits above. The Orchestrator owns the journal update and final acceptance integration.
