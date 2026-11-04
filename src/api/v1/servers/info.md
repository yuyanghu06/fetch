# Server HTTP routes

server_id/ contains handlers for a single /api/v1/servers/{server_id} target.
DELETE delegates to Manager with the current caller. The native-only open-folder POST delegates to actual backend folder resolution. This folder
does not replace the existing server-list/auth/config/restart adapters yet.
