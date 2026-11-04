November 4 follow-up: Railway’s reported failure is a missing railway executable in the child PATH, despite separately prepared CLI. [Diagnosis](../history/2026-11-04_12-59-56-434568-orchestrator-railway-cli-not-on-startup-path.md); helper lookup repair is now complete in ../private-program-paths/evidence.md; actual account login remains untested.

# Report actual local startup failure

Goal: replace unknown generic failure context with the process's own sanitized
startup stderr and actual exit status, visible in Fetch's nativeconsole. Missing
input preflight stays removed. No guess of this user's actualGooglefailurecause.

Scope: bounded transient diagnostics during local MCP startup, captured/redacted
before memoryretention; clear onnextattempt/freeze onready/failure. Runner holds
private generation/lines/exit_code/signal; general snapshot/MCP never includeit.
Existing GETservers adds auth.startup onlyfor nativeSettingscaller. UI appends
serverlines/exitstatus withoutduplicatingpolls. No newroute, logfile, credential
reads, filemigration, serverprerequisites or normaltool-resultchanges.

Completion: fastfatalexit3 message/status survivescleanup; secrets/fullURLs
removed; nativeonlyAPIvisibility and no-diskretention proven; nextattemptclears
oldfailure; UI dedup/generation/exitdisplay passes; existing regressions/build and
independentreview ACCEPT. RealGoogleaccount/nativevisualbehavior remainunobserved.

Status: complete and independently ACCEPTED for configured local startup errors.
Independent 181 checks pass; final ordinary-start UI extension passes 31 targeted
checks, final locked build 8.58s and whitespace pass. [Evidence](evidence.md).
Recorded helpers and remote bodies remain outside this report. No real Google
account/browser/native app run; old discarded output cannot identify its cause.

November 4 read-only follow-up identifies the reported Google failure: original
configuration supplies app credentials as environment values, while Fetch omits
them during import. No credentials.json exists at either package’s expected path.
Import-policy changes are separate from the completed diagnostic feed.

Source review also confirms discovery imports freshly install selected npm/PyPI
packages, reuse local scripts, or save remote connections. This explains why a
new package copy does not restore omitted original application configuration.
