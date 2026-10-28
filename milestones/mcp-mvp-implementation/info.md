# MCP MVP source implementation

## Goal and scope

Implement the selected Rust/macOS MCP first release through a user-controlled testing handoff. Scope follows docs/architecture/build-readiness.md, including durable authorization/Keychain/TLS, private runtime/package preparation, server management, raw MCP sharing and native UI. Memory/database/graph/process pools/native Tasks/live resource subscriptions remain deferred.

## Completion criteria

All selected source paths and distribution scripts are written and integrated; scoped commits are pushed; actual limitations and first testing prerequisites are recorded. This is source completion only. Runtime acceptance remains a later milestone.

## Current status

Active. Planner source-reading report received. Three exclusive implementation assignments cover authority, management and runner; Orchestrator owns integration/UI/packaging and serialized records. D77 forbids all testing, build/check/format/lint commands, application launches, connection attempts, dependency/runtime installation and packaging execution. No new verification is claimed.

Authority source is written and registered; see [the pass](../history/2026-10-28_13-55-35-037181-coder-durable-app-authorization-source.md). No compile/runtime evidence. Manager/runner/UI integration remains active.

Private installer/discovery/storage source written and source-reviewed; no runtime/package preparation has run. Manager and native integration remain active.

Shared runner and native MCP source written, including raw transport fidelity/owner accounting mechanisms. No compile/runtime/client evidence.

Manager source integrated after bounded read-only review. The implementation records supported authentication adapters and unsupported cases explicitly; runtime/provider/client success is unverified.

API/native application/Iced interface source integrated after bounded source review corrections. No visual, HTTP, OS or client behavior has been exercised. Final handoff/distribution-source documentation follows.
