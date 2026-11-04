# Startup diagnostics verification

The generic Google-start screenshot did not contain the original process error.
This change makes the next configured local startup expose its own startup stderr
and natural exit status in Fetch's native console. It does not establish why the
past Google attempt failed or claim a successful Google-account login.

## Implemented behavior

- All startup paths still omit missing environment/header values and pass present
  saved values unchanged. No required-input preflight was restored.
- During configured local startup, stderr is sanitized before memory retention.
  The tail is capped at 32 lines, 16 KiB total and 1 KiB per line. Known environment
  and argument values, common credential/Bearer assignments and HTTP(S) URLs are
  removed. This is a bounded startup report, not unrestricted raw logging.
- Natural process exit code/signal is captured before Fetch terminates cleanup
  processes. Fetch-initiated termination is not reported as a natural server exit.
- Evidence freezes at startup completion and clears for the next attempt. Only
  native Settings GET /api/v1/servers receives authentication.startup. Ordinary
  API callers, MCP-facing reads, general Runner snapshots and saved configs do not.
- The console appends sanitized server lines and exit status without duplicate
  polls. Failed ordinary Start/Connect also creates a console from status, without
  starting an authentication request.

## Checks actually run

| Check | Result |
| --- | --- |
| Independent cargo test --locked --lib | 104/104 PASS, tests 0.42s |
| Independent standard_authentication integration target | 10/10 PASS, tests 34.56s |
| Independent existing backend/HTTPS/direct/path/import/isolated/MVP/raw targets | 67/67 PASS |
| Targeted fast fatal/cleanup integration recheck | PASS, 0.53s |
| Final ordinary-start console extension: cargo test --locked --lib ui::auth_tests --message-format short | 31/31 PASS, 74 filtered, compile 5.45s/tests 0.02s |
| Final cargo build --locked --bin fetch --message-format short | PASS, 8.58s |
| git diff --check | PASS |

The independent frozen-source suite had 181 unique checks before the final small
UI extension. The final 31 UI checks include its new ordinary-start display case;
these are a targeted recheck, not another 31 unique checks. Independent Evaluator
ACCEPTs the backend and the final display extension.

The fatal fixture starts a real confined child, prints a short final FATAL line
with a dummy secret/URL and exits 3. Native Settings sees sanitized output and
exit_code 3/signal null; ordinary callers and disk do not. Retry clears the old
report. The first diagnostic run exposed Quit returning before an initializing
child was reaped. Tracking the provisional connection and confirming idempotent
close fixed it; the unchanged immediate-PID-absence assertion then passed.

## Limits and cleanup

Tests used deny-default native fixtures and disposable loopback services. No real
Google account, native app/browser, real Keychain, personal client settings or
existing service was used. Tester reports no remaining fixtures, child processes
or task logs; root's build/UI logs were removed after summarizing.

Recorded setup-helper output and remote response bodies are not part of this
feed. Successful startup freezes the report; it is not a general tool/runtime log.
The user's old Google output was discarded, so its cause cannot be recovered from
that screenshot. Explicitly Quit Fetch, run the signed development launcher, and
retry to capture the actual Google startup message. No signing/launch/release was
performed by the agent. All source/documentation edits remain local.
