## Claude subagent metadata editing

`agents.rs` changes the selected name or description in a Claude draft using the existing YAML parser and formatter, then attaches the original Markdown body. Fetch's native file API handles the eventual Save.

## Subagent registration metadata and file removal

`files.rs` matches Codex registration metadata to the exact definition in the selected scope. The shared Codex configuration is reserved. Confirmed removal deletes only the selected definition leaf. `agents.rs` maps the selected Codex read-only field to Claude Plan.

## Subagent conversion name

When converting between apps, an optional source card name fills a missing name. The explicit definition name wins, and a missing description remains an issue.

## Create a subagent file

`files.rs` checks the existing Workspace path and agent content rules, writes a completed temporary file, and links it to the requested filename only if it is free. The ordinary edit path retains its existing replace behavior.

## Project-bound session access

`project_sessions.rs` is the common HTTP/MCP history reader. It requires an
approved project/history caller, resolves the current registered folder and
reuses the bounded native readers. It removes private source paths from response
metadata while preserving raw log content. It exposes no instruction/agent file
operations and writes no logs.

## Models and compressed history

models.rs reads Codex's saved native catalog without starting Codex, excludes
account cache metadata and returns raw visible model definitions. Claude uses
initialization-only JSONL with no prompt/tools/MCP/persistence; managed policies
may still apply. model_tests.rs uses fake processes/cache files only.
Sessions supports plain and zstd-compressed JSONL, preserves original streams,
counts decoded byte offsets and bounds each chunk. Compressed reads re-decode
prefixes; fork streams retain native parent references rather than reconstructing
branches. Model catalogs are app reports, not live account-access verification.

# Workspace backend

Workspace reads the user's existing coding-app files. It does not own another
copy of instructions, agent definitions or session logs. Native API handlers
authorize the person before calling this module; this module never authorizes
ordinary MCP callers to edit files.

- `mod.rs` exposes the file-backed service and injectable home/app directories.
- `files.rs` discovers recognized instruction and agent files and saves edited
  native files in place. Selected project folders exclude global definitions.
- `agents.rs` parses Claude Markdown/YAML and Codex TOML, converts shared fields,
  and returns app-specific settings as explicit review issues with the source.
- `sessions.rs` lists lightweight session metadata and returns bounded pieces of
  the existing UTF-8 logs. Session files are never written or archived.

Tests must inject disposable folders rather than use personal files. The API and
MCP gateway own native authority and project-bound connection checks. A requested
path is an input, not permission for an MCP caller to read arbitrary files.


conversations.rs runs explicitly installed official readers with native authority/path checks and confined child processes. agents.rs also exposes its existing parsed fields/instructions for native preview. Finder reveal uses existing editable-file guards.
