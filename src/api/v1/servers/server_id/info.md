# One server target

delete.rs handles DELETE /api/v1/servers/{server_id}, applying existing management
permission, target hiding and empty-body/query requirements before shared Manager
removal. Native state, packages, old credentials and keys are preserved.

open_folder.rs handles POST /api/v1/servers/{server_id}/open-folder for native
Settings only. It accepts no body/query/path and performs no secret confirmation.
The backend resolves the actual supported folder and asks Finder to open it;
unsupported location and opener failures are returned truthfully. A successful
opener request is not visual evidence. Other server route adapters remain in the
older runtime.rs until separately reorganized. Common HTTP errors/no-store rules
apply; handlers do not own server-state mutation or credential storage.
