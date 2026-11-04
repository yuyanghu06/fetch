# Start without inferred input requirements

Goal: Fetch calls the existing server start command/connection with the saved
values available, and reports its outcome. An imported variable/header name is
not proof that the value is required. The server decides what it needs.

Scope: remove shared missing-binding preflight for all startup paths, both local
environment and remote headers. Pass present values unchanged, omit absent values.
Ordinary app startup, post-install Connect, Restart, explicit Authenticate and
recorded native helpers all use shared resolution. Remove synthetic input-required
status handling. No per-provider required/optional registry, original-value import,
credential migration or new output feed. Existing API/ownership/work-safety rules
and server-result mapping remain. Raw stderr/exact exit-code feed is not added.

Completion: absent optional/file-fallback inputs no longer block actual child
startup; genuine missing requirements fail in the child/server, not preflight;
remote HTTP is attempted without missing configured headers and reflects actual
acceptance/rejection. Available inputs pass unchanged. Focused unit/confined/HTTP
and existing regression checks/build pass; independent evaluation ACCEPTs.

Status: complete and independently ACCEPTED.176checks pass (100lib+9integration+
67regressions); locked native binary buildPASS0.49s. Actual child/HTTPoutcomes
verified without personalstate. Google screenshot now shows a genuine attempted
startupfailure but Fetch's genericmessage; separate startup-diagnostics work
addresses reporting, not a return of prerequisitechecks.
