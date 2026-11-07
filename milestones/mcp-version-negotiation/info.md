# Codex initial version negotiation

Goal: An authenticated coding client can offer its initial MCP version and negotiate Fetch's existing supported legacy version, then list tools.

Scope: Fix initialize negotiation at the existing /mcp endpoint. Preserve2025-11-25and2026-07-28operating modes, auth, permissions and session ownership. No older operating mode, client configuration change or new route selected.

Acceptance: Actual Codex-offer fixture fails before/passes after; existing legacy session/revocation flow passes; native signed build; direct review; real test client initialization/tool listing after explicit signed relaunch. Passing fixture is not native client acceptance.

Status: Complete for scoped connection. New fixture fails before/passes after;13focused tests, signed build8.49s/strict verification pass. After user Quit/relaunch, actual dedicated fetch_codex app-server loads405Fetch tools using saved OAuth access with no error/password prompt. No model task or upstream tool call. Existing user TUI may need reopening. Direct checks only, no independent agents.
