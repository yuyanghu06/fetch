Workspace routes live in workspace/ and require the existing native-window
capability before file or catalog access. They wrap src/workspace, accept explicit
paths and never become MCP tools in this slice. AppState provides injectable
coding-app roots for disposable tests.

# Versioned application HTTP handlers

This folder holds HTTP adapters for /api/v1. servers/ adds deletion and native folder opening in
URL-shaped modules; existing route families still live in older API files until
the separately requested reorganization. Backend business logic stays in shared
services. No handler owns credential/file cleanup directly.
`projects/` wraps native-only registered-folder list/create operations.
`sessions/` wraps separately permitted project-bound history list/chunk reads.
These share the registry/history services with MCP while `workspace/` remains
private to the native window for instruction/subagent editing and browsing.
