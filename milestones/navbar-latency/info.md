# Responsive navigation

Goal: remove the noticeable delay when switching Servers, Install MCPs and Connect apps without changing their appearance or backend actions.

Scope: shared decorative rendering and a focused actual-widget tab-click/render check at Retina scale. Keep the existing flat fills, complete resvg shadow stacks, input behavior and display scaling. No server/API changes, personal state, service launch, credentials or client configuration.

Completion: measure before/after page switches, verify the actual tab clicks and rendered content, compare appearance, and build the native binary. Checks are performed sequentially by the primary agent at the user’s request, not independently.

Status: scoped fix complete; six focused UI checks, before/after image inspection, native build and signed development strict verification pass. Direct primary-agent review ACCEPT, not independent. Native running-app confirmation remains separate.

Final timing: first tab visits 16–74ms, return visits 10–20ms; original switches 492–1406ms. Returning-page pixels equal their first frame exactly. See [evidence](evidence/info.md). The existing running app was preserved. Explicitly Quit it, then run ./scripts/run-development.sh to load the prepared signed build.
