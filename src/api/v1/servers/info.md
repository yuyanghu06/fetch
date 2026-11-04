# Server HTTP routes

server_id/ contains handlers for a single /api/v1/servers/{server_id} target.
The new DELETE adapter delegates to Manager with the current caller. This folder
does not replace the existing server-list/auth/config/restart adapters yet.
