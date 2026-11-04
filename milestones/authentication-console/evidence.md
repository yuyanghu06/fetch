# Authentication console evidence

November4,2026. Scoped UI change accepted by independent Evaluator.

The old authentication heading, repeated setup/fallback/restart paragraphs and
Hide button are replaced by one200px dark scrolling monospace console. Existing
Fetch progress/error/host events are recorded per attempt and unchanged polls
do not add duplicate entries. The current host-only consent action remains
inside the console; existing server controls and Settings are preserved.

- Independent cargo test --locked --lib ui::auth_tests --message-format short:
  28/28PASS,71filtered, compile9.48s/tests0.02s.
- Three new tests cover ordered/deduplicated events, error history, independent
  attempts/servers and no private input/full URL/unknown raw field logging.
- Existing consent, pending-window restoration and request-coordination cases pass.
- Locked native binary buildPASS9.37s; removing unused import then final build
  PASS3.06s. Existing warnings remain; no new unused import warning.
- git diff --checkPASS; temporary Tester/root build logs removed.

No application window, provider/MCP, browser, real Keychain or real account ran.
Native visual/Figma acceptance was not observed. This console displays safe Fetch
progress/errors, not raw process output. The optional actual-output clarification
was unanswered; no new API/backend feed or universal redaction guarantee added.
It does not repair or bypass the missing input values shown in the screenshot.

Files: src/ui/authentication.rs, authentication-only portions of src/ui/mod.rs,
shared components.rs and theme.rs; focused additions in auth_tests.rs. Earlier
source/doc changes remain preserved. No commit/push requested; edits are local.

## November 4 — Copy button follow-up

The shared console has a fixed small Copy button above the scroll area. It uses
Message::Copy(flow.log.join("\n")) and the existing Iced clipboard handler.
Independent source review confirms scrolling and sign-in remain intact. Locked
binary build passes in 9.80s; whitespace passes; Evaluator ACCEPT. No new tests,
native app/window, OS clipboard or MCP execution. Changes remain local.
