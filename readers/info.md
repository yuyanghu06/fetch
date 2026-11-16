# Official saved-history readers

These helpers run pinned client reading code, not models or MCP servers. codex/ owns the native Codex storage-library helper and tiny documented access patch. claude/ wraps the pinned official Python SDK's supplied-text reader. scripts/setup-session-readers.sh prepares both in a Fetch-private reader root; setup is explicit, not triggered by session opening. src/workspace/conversations.rs owns authority, validated source paths, child confinement, deadlines and cleanup; the UI only formats returned messages.

All original histories/settings stay read-only. Helpers write only into the provided disposable work_dir during a request. Source/build caches for installed helpers are separate from per-read snapshots. No normal Codex/Claude service, model/login/MCP startup is permitted. See docs/architecture/session-reader-reuse.md for the approved contract.
