# Rust request wrapper

`main.rs` validates allowed source paths, stages native rollout copies, invokes
Codex's official readers, and maps their completed display items into Fetch's
message response. It contains no Codex startup, login or model code. The original
source roots are read only; SQLite projection is confined to the fresh request
folder. See the parent guide for protocol and source patch details.

Listing skips a thread only when its native selected rollout returns NotFound. Other errors propagate, and saved selection stays authoritative even if an older file survives.
