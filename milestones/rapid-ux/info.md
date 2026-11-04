# Rapid UX changes

Goal: implement the user's small UX changes directly, without spawning agents.

Scope so far: toggle Select all / Unselect all on the Install page, and attempt
all saved MCP connections independently at app startup. Preserve existing shared
controls, server authority, credentials and readiness/error behavior.

Completion requires focused source review and native compilation. Actual window
interaction and real saved-server startup remain separate runtime acceptance.
Current source is implemented; verification details are in the rolling snapshot.
