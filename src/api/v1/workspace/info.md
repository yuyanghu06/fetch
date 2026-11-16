# Workspace APIs

Native-window-only routes for folder discovery, existing instruction/agent file
read/save, converted drafts, installed-app model choices and bounded session
history. Handlers authorize the existing native capability before resolving a
path or starting metadata discovery. Shared operations live in src/workspace;
the frontend sends explicit paths over the existing pinned HTTPS NativeClient.
Ordinary bearer/background grants cannot call these routes, and none are MCP
tools in this slice. Original session logs are never written.
