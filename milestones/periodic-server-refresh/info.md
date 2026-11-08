# Periodic server list refresh

Goal: refresh the open app server list through the existing GET /api/v1/servers every60seconds, without requiring a button click or expanded card. User clarified this is the app, not the background MCP Runner.

Scope: one independent UI timer, existing refresh method and pending-read guard; retain faster console/auth/restart/installation checks. Closing the window still revokes native capability and removes the client. No new route, management permission, backend runner behavior, login, package or personal-state change.

Completion: focused disposable native-UI state checks, locked binary build and independent final review. Status: complete.2focused fixture checks, locked build12.17s and independent Evaluator ACCEPT. Checks cover scheduling/reply handling;60s interval and GET path source-inspected. No live HTTP or native launch performed. A later actual native-app observation remains separate; do not launch Fetch here.
