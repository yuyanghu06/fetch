# Console, system-shell terminal and saved log

November 4, 2026. User approved the feature, system shell, exact GET/POST route contract and saved-log policy through the explicit “Approve this contract” response. Scoped implementation is accepted;182 relevant checks19new/native build/independent EvaluatorACCEPT. Actual offscreen rendering verifies the embedding correction; rebuilt native appearance and real accounts remain unobserved. See milestones/server-console-terminal/evidence.md. Credential storage stays with the service.

## Everyday behavior

Each expanded server has Console and Terminal buttons above the same pane. Terminal is enabled for local command-based MCPs; URL-only remote MCPs have no local command environment, so their Terminal button is disabled with that reason. Console is present even before authentication and shows received startup/authentication logs. Terminal opens the user’s system shell, using that MCP’s executable search folders, assigned environment, account folder and saved working directory. It does not open the Fetch source repository. Server-private program folders stay first; the default command search then retains the normal PATH inherited by Fetch, so globally installed programs are discoverable. Explicit assigned PATH retains its override behavior. Switching views keeps the terminal and log intact. No login command is inferred or run automatically.

The Console log survives Fetch restarts. Approved bounded policy: keep the most recent1MiB/4096lines per server,64KiBperline, with a visible notice when older output is discarded. This supersedes the earlier first-attempt-prefix-only rule, which would eventually stop logging forever once a saved log filled. Save only sanitized service console output; terminal keystrokes and raw terminal recording are excluded. Persistent files are private under the selected Fetch state, not credentials or personal configuration.

A normal-terminal login can differ from the MCP login because the MCP has a private HOME. A private CLI binary can also use a normal account folder; program location and account storage are separate. Current behavior remains the selected per-instance HOME. No automatic global-account fallback is selected. The terminal must match the actual MCP environment. Service-owned commands such as Railway whoami can report the account without Fetch inspecting/copying tokens; no account checks were run here.

## Existing component to wrap

Selected implementation: pin MIT iced_term0.8.0 (Iced0.14, Alacritty0.25.1) and add a narrow external-transport seam. Reuse its parser/rendering/keyboard/selection; replace direct UI-owned process IO with backend API IO. Backend wraps portable-pty0.9 for actual shell process, resize and cleanup. Keep upstream license/attribution and clearly document the seam. No new terminal parser or provider authentication implementation. The downloaded0.8.0 crate archive was verified against the registry checksum; its .cargo_vcs_info identifies upstream commit e0e90be2c160bd68f1bf544a51bf772d75b98ffc. Root Cargo.lock now resolves the new dependencies for Rust1.89 compatibility. The crate currently creates its own PTY, so using it unchanged would violate the selected API-first boundary.

Alternatives researched: new iced_terminal/zeughaus0.2.1 separates model/transport but has very little usage and still overlays inherited env; xterm.js+Wry is mature but adds embedded-browser integration. Neither is selected.

## Approved HTTP route contract

One resource path, two methods:

- POST /api/v1/servers/{server_id}/terminal opens/controls the server’s terminal.
- GET /api/v1/servers/{server_id}/terminal reads output/status only. It never opens a shell.

Every request requires the existing native Settings capability and permission for the selected server. Ordinary bearer, MCP and background callers are denied403; no agent-facing terminal tool is added. HTTPSlocalhost/origin/body/no-store protections remain. Session IDs are opaque and bound to the selected server. One terminal is active per server. No environment/command override or arbitrary shell selector is accepted by the API.

POST bodies, exact action-specific fields:

```json
{"action":"open","columns":80,"rows":24}
{"action":"input","session_id":"terminal-...","data":"BASE64_BYTES"}
{"action":"resize","session_id":"terminal-...","columns":100,"rows":30}
{"action":"close","session_id":"terminal-..."}
```

For open, omitted dimensions default80x24. Dimensions must be integers1..1000. Input is base64 so terminal bytes are preserved; decoded input limit64KiB/request. No input/clipboard content is written to Fetch logs. Unknown/additional fields or invalid encoding/size/dimensions return400INVALID_REQUEST.

Open resolves the current account shell using macOS getpwuid_r/pw_shell. It starts that shell attached to a PTY in the exact native_state::transport environment/cwd, clearing inherited env first and adding SHELL/TERM=xterm-256color. No login-shell flag or command is injected. A URL-only remote server returns422TERMINAL_NOT_LOCAL. Missing/unusable shell returns503TERMINAL_SHELL_UNAVAILABLE; PTY/spawn failure returns503TERMINAL_START_FAILED with safe OS detail. Existing live terminal is reused200; new terminal returns201. No automatic new shell is started by polling or after an exit.

POST success returns application JSON with server_id, session_id, status(running/exited/closed), output_cursor, exit_code/signal when known, and next_steps[]. Open also returns the nonsecret shell/working_directory. Input/resize/close returns200 after the requested operation is accepted/applied. Unknown session/server returns404; wrongserver binding is treated as notfound. Write/resize failure returns409TERMINAL_UNAVAILABLE with safe actual IO error. Every error includes next_steps.

GET accepts session_id(required) and after(nonnegative byte cursor, default0); no request body. Response200 contains server_id,session_id,status,data(base64 output chunk up to64KiB),next_cursor,first_available_cursor,gap(boolean),exit_code,signal,next_steps[]. Backend keeps at most1MiB of terminal output for readback. If output no longer exists at the requested cursor, gap is true and the UI visibly reports missing terminal history rather than claiming a complete replay. This is session output buffering, not durable terminal recording. Existing terminal widget/parser keeps consuming while Console is shown, so toggling does not reset the screen. Exit status is factual; no terminal command is repeated automatically.

Close stops/reaps only the owned shell process group; it does not stop the MCP itself. FetchQuit and server removal also clean up owned terminal processes. Switching panes does not stop them. Closing/reopening the backgrounded Fetch window does not create another shell; a fresh authorized native view can attach to the existing session. No terminal survives FetchQuit.

## Approved persistent console storage

Use the existing checked/private atomic file helper, storing console-log.json beside each server config.json under the hashed server directory. Credentials/config/package state are not copied. Retain only sanitized log lines, a format version and observed exit facts needed to reconstruct the console. Save through a coalescing background writer so noisy output cannot block server startup; flush pending log writes on explicit Quit. Read the saved log when opening/starting the server’s capture; preserve startup→auth history even if the UI never polled the earlier attempt. File/write failure is shown honestly as a Fetch logging error without claiming successful save. No log file is read or returned through ordinary API/MCP calls; native Settings continues using its existing startup field. Registration removal preserves the log file with the rest of native state. No new log-management operation or retention archive is selected.

## Acceptance

- Expanded rows always have a console; switching/collapse/auth do not erase history.
- Saved logs reload across a new backend instance; bounds/truncation/redaction/native-only visibility remain.
- Real disposable PTY shell echoes typed input, changes directory, resizes and exits with actual status.
- Terminal uses fixture server env/cwd/HOME/program folders; does not use real credential files or global account during tests.
- GET and unauthorized calls do not start a shell; server/session isolation and owned cleanup work.
- UI wraps the existing terminal component; terminal input/output never becomes provider-login logic or a credential store.
- Relevant independent tests, native build and evaluator acceptance. Native appearance remains separate unless an isolated native view is actually observed.
