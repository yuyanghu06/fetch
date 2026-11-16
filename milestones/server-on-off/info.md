# Persistent server on/off

Goal: give server cards a real on/off control through the existing start/restart POST and retain Off across Fetch restarts until explicitly turned on.

Scope selected by user November 12: action:stop, existing management permission, preserved files/logins, non-interrupting busy rejection and durable Off. No force-stop or new config-PATCH control. Coder owns production, Tester owns tests, Evaluator is read-only, parent serializes records; shared role files were unavailable on this Mac, so agents used the roles in AGENTS.md.

Status: implementation, focused verification, signed build and independent evaluation complete. Live user observation remains pending.

Persistent server on/off is implemented. Each server card has the shared 44×22 toggle before its existing actions. Off uses POST /api/v1/servers/{id}/restart with action:stop, preserves files/logins, rejects busy work and stays off across startup/background connection attempts. Existing explicit start/restart turns it on. config.enabled is projected independently from runtime, with legacy records defaulting true. The MCP management action uses the same backend. The separate 22px Connect authentication buttons are black with white keys.

30 focused checks pass: 3 confined HTTP/MCP product scenarios (0.90s), 2 private no-network/background/conflict checks (0.07s), and 25 actual widget checks (9.88s). Inspected minimum-window toggle and Off/On/busy Console PNGs. Final signed-development build-only passes (24.59s) and strict signature verification passes. Independent Evaluator ACCEPT; no source fixes requested. Existing compiler/deprecation warnings remain.

Fetch was not relaunched. Actual personal/provider accounts, client connections and macOS approvals were not tested; those remain manual acceptance. Failed-close and caller-disconnect behavior were reviewed in source through the reused retained-child close path and detached worker, without fault injection. No personal config/credentials/trust, running services, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the rebuilt app.

Important code: src/manager/power.rs owns saved preference and idle stop; Runner holds lifecycle/admission guards and existing retained-child close; Manager start-capable paths honor the current flag. UI owns presentation/request state only. HTTP and fetch__restart_server share the management implementation. Failed stop reports SERVER_STOP_FAILED and retains Off/retryable context; failed save does not close.

Acceptance: actual confined start→stop→child exit→fresh startup stays off→explicit start→tool call; busy active/queued calls preserved; quiet disabled background paths; permission/MCP action forwarding; native pending/rejection/Off/On; signed build and Evaluator ACCEPT. All pass at the stated fixture/source boundaries.

[Selected contract](../../docs/architecture/server-on-off.md) · [Evidence](evidence/info.md).
