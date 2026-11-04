# Authentication console

Goal: replace the repeated inline authentication prose with a single compact
console that records useful progress/errors. This is the native Iced equivalent
of the user's requested div, not a new browser interface.

Selected immediate scope: safe existing Fetch state/messages in an ordered
per-attempt log; scrolling monospace panel; host-consent action remains inside it;
remove repeated title/fallback/restart text and Hide control. Preserve Settings,
server summary, existing API/login behavior and all other dirty source changes.

Optional clarification is pending on actual MCP process output versus Fetch
progress/errors. Backend does not currently retain arbitrary process output;
this UI pass makes no actual-stdout or universal secret-redaction claim. Do not
add a new output feed before that separate choice is answered.

Completion: ordered/deduplicated independent console events, current errors and
read failures visible, browser-host consent and window-reopen behavior preserved,
focused meaningful UI checks and native build pass, independent review accepts.
No real account/native visual acceptance inferred.

Status: UI implementation complete; independent28/28focused checks, final locked
native build3.06s and whitespace pass. Independent Evaluator ACCEPT.
[Evidence and remaining limits](evidence.md). No native visual/Figma/runtime proof.
Root serialized all journal/doc writes; actual process-output choice remains
optional/unanswered and no backend feed was added.

November 4 Copy follow-up: add a fixed small Copy button to the shared console
header and reuse the existing clipboard action for newline-separated log lines.
Complete: independent locked build (9.80s), whitespace and review pass.
Native appearance and OS clipboard use were not observed.
