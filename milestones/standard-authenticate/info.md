# Standard Authenticate behavior

Goal: one Authenticate action uses each server's existing login, including normal
local startup without recorded helper metadata. Show human sign-in controls while
keeping provider login, tokens and renewal owned by the server.

Scope: existing authenticate POST/GET server status; recorded helper or ordinary
local startup, interactive wait, narrowly selected sign-in URL with explicit host
consent, honest current/last-known status, removal of misleading inline fallback
controls. Existing remote OAuth, raw tool data, Settings keys and post-install
Connect remain. No guessed provider commands, source interpretation, migration or
release. Native URL elicitation has no human-reply contract yet and is not falsely
advertised by this initial implementation; existing supported remote flow/relay
remain unchanged.

Completion: configured local startup works without native_setup; server's login
link is available before readiness; consent can open it without leaking raw logs;
interactive wait survives30seconds and Quit cleans owned processes; success alone
remains auth unknown; saved last-known never manufactures current sign-in; existing
remote/Settings/raw behavior passes meaningful checks and independent evaluation.

Status: scoped implementation complete and independently ACCEPTED.171unique
checks pass; final guidance-only32check retest and locked native build pass.
All material review/test findings resolved. See [exact evidence](evidence.md).
No real Google account, native browser/Keychain/window or coding-client acceptance
was performed; those remain separate product checks. No launch/signing/release,
credential migration or personal config change. Changes remain local.

User route follow-up: explicit login button calls existing /restart start/connect
API with interactive:true. /authenticate HTTP/tool are deprecated but retained,
including restricted background renewal compatibility. No duplicate /start route.
