# Python MCP environment comparison

- Local timestamp / acting role / milestone: 2026-11-04T18:08:39.473609-05:00 / Orchestrator / install diagnostics.
- Requested scope: Explain per-server virtual environments, Python version requirements and how coding clients handle them.
- Starting repository state: Broad existing work preserved; prior Blender diagnosis found unsupported Python pin.
- Work performed / changed files: Read current Python preparation code and official Codex, Claude Code and uv documentation; appended research.md and this history; updated snapshot/to-do.
- Decisions: Per-server environments already exist. Delegating environment handling to uv and honoring per-server version requests is a recommendation, not a selected implementation. No API or source changes.
- Checks and evidence: Official docs fetched and read: https://learn.chatgpt.com/docs/extend/mcp?surface=cli ; https://code.claude.com/docs/en/mcp ; https://docs.astral.sh/uv/concepts/tools/ ; https://docs.astral.sh/uv/concepts/python-versions/ . Current prepare_python explicitly creates each installation's venv but uses global PYTHON_VERSION for every one. Primary-agent research, not an independent team review. No tests/build/install/MCP launch.
- Resulting state / acceptance: Explanation distinguishes command launching by clients, Python management by uv, dependency environments and shared runtime files. No claim that uvx automatically inherits a surrounding project's Python request.
- Remaining work and handoff: User to select any implementation direction; immediate supported-pin repair and more general per-server Python handling remain unimplemented. Preserve other active work.
