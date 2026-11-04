# Managed MCP runner

Idle deletion gates scheduler/lifecycle/metadata work and rejects active/queued/uncertain/unresolved calls rather than canceling them. Bounded close retains the child handle until confirmed exit. Removed IDs cannot reconnect or resolve stale aliases; successful removal clears registry/catalog/continuations and emits catalog changes. Quit enumerates raw contexts including fenced ones. Source-only evidence; no runtime acceptance.

D83 retains safe startup error codes/messages and records explicit provider-auth rejection against both the actual connection identity and its generation. Published-request errors use a brief connection read lock before the atomic latch; no RPC runs under that lock. Startup preparation uses its existing lifecycle protection. A replacement connection is not flagged by an old connection's error. Manager projects this evidence through GET servers, preserving the distinction between usable connection and verified credentials. No new runtime evidence was produced.

The runner owns one upstream connection and one bounded waiting queue per installed server. `transport.rs` sends raw protocol messages over child stdin/stdout, Streamable HTTP or legacy SSE. `scheduler.rs` keeps timed-out or cancelled forwarded calls counted until a real terminal reply or explicit lifecycle recovery. `catalog.rs` preserves definitions and records the selected modern HTTP annotation exclusions, stable names and qualified resource selectors. `raw.rs` keeps upstream JSON values intact while changing only protocol routing fields.

Provider values arrive only from the manager's protected storage. This module never reads a personal harness configuration or prompts for a credential. Source implementation is unverified: no build, checks, services, fixtures or connections were run for this pass, by user instruction.

The implementation reuses the repository's [protocol research](../../docs/architecture/mcp-protocol-adapters.md). Source reading of the [official current transport](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/transports/streamable-http.mdx), [schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts), and [MRTR flow](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/patterns/mrtr.mdx) established framing and header encoding. Maintained [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) and [ToolHive](https://github.com/stacklok/toolhive) designs were compared through existing research and their Apache-2.0 license records. No source was copied; no SDK normalizer is used because the selected contract needs the original JSON before SDK filtering.

Engineering bounds are 64 concurrent calls per server, 24 hours for a configured finite execution duration, 64 waiting calls / 16 MiB of waiting arguments / 300 seconds of waiting, 256 active native requests/continuations and 64 MiB per upstream message. Resource reads, prompt retrieval and completion share per-server admission and the server fallback duration; tool overrides remain specific to tool calls. Native aliases use a 128-character portable profile with a persisted collision/reservation map. Host acceptance of these engineering choices remains for user-started testing.

`cached_catalog` only projects a previously completed acquisition, including original observation time and known rejections. It makes no RPC and cannot label an unknown or replaced connection complete. Shutdown permanently closes admission before stopping connections; a delayed restart cannot reopen this runner after Quit.

Execution timeout precedence is an explicit per-tool override, then an internally supplied documented/current whole-call limit plus 5,000 ms, then the server's configured fallback. The internal supported-limit map is excluded from serialization and deserialization and defaults empty. No generic MCP Tool field or current provider adapter supplies such an observation; unknown or stale provenance therefore uses the fallback. A trusted future adapter must bind observations to its reviewed source and current connection context. Supported limits are positive, and checked addition of the allowance must remain within the finite 24-hour execution bound.

Remote MCP HTTP POST, legacy SSE opening and session event GETs use the safe
diagnostics sender. Logged completion is HTTP headers-ready, not terminal tool
completion. JSON-RPC payloads, content and provider headers are never logged.


ManagedStdio is an internal attachment for approved local helpers using the
shared credential bridge. Each actual child launch receives a fresh runtime
lease/capability; Connection owns it and closes it on setup failure, connection
failure, stop/delete and drop. Ordinary Stdio constructors are unchanged.
Credential RPCs use a separate private Unix socket so MCP stdin/stdout messages
remain raw. The runner neither selects provider fields nor verifies accounts;
Manager supplies the fixed-owner factory/trusted verifier.
