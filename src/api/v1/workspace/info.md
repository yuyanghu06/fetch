## Subagent descriptions and removal

The existing file read can return a description from the same-scope Codex registration. Conversion uses it as a fallback. Native DELETE removes the selected individual definition and returns the existing file identity. Workspace discovery and file operations protect the shared Codex configuration.

## Subagent conversion name

The existing conversion request accepts an optional fallback name from the clipboard. It fills a missing name only on cross-app conversion.

## Add subagent

`PUT /api/v1/workspace/file` accepts `If-None-Match: *` to create an agent only if its name is free. An occupied name returns 412 with the error message shown in the draft popup. Other values for this header return 400. Existing requests without it use the regular save path.

# Workspace APIs

Native-window-only routes for folder discovery, existing instruction/agent file
read/save, converted drafts, installed-app model choices and bounded session
history. Handlers authorize the existing native capability before resolving a
path or starting metadata discovery. Shared operations live in src/workspace;
the frontend sends explicit paths over the existing pinned HTTPS NativeClient.
Ordinary bearer/background grants cannot call these routes, and none are MCP
tools in this slice. Original session logs are never written.
