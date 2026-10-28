# Read-only server discovery

Discovery inspects only injected home/config roots or one selected absolute file. It reads Claude strict JSON, Codex TOML and OpenCode JSON/JSONC, rejects duplicate keys and applies a four-MiB read limit. JSONC permits comments and trailing commas only; JSON5 extensions stay disabled. Failed default sources remain visible beside healthy sources. Discovery never runs packages, commands, helpers, login or network requests, and never copies source credentials.

The first supported local grammar is deliberately narrow: `npx` with optional `-y`/`--yes` and one literal package/version; `uvx` with one literal package or `package==version`; node/python with one absolute script path; or an absolute direct executable without arguments. Unknown arguments, helpers, interpolation and execution fields produce blocked rows. Literal HTTP/SSE records return only transport, origin and observed header names; URLs with user information, query parameters or fragments remain blocked. Original remote paths and header/environment values never appear in discovery output.

This is an initial reader implementation, not proof that the user's real inventory can be installed or connected. Source records are reread and validated during the later installation implementation. It does not reproduce client configuration layering or project inheritance.
