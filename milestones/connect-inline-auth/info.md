# Connect apps inline authentication and popup layout

Goal: put authentication beside each app, remove the requested extra controls/footer, provide dismissible approval, constrain popup shading to Fetch, and present Connect content in the same recessed panel as Servers.

Scope: native UI components, page/popup rendering and local popup visibility state. Existing authentication APIs, permissions, Mac confirmation, credential ownership and command generation remain.

Completion criteria: actual-widget clicks open the matching request; Close and backdrop hide without deciding; polling respects hiding; the same key reopens; dimming leaves rounded corners transparent; normal/minimum layouts and signed development build pass.

Status: implemented and directly checked; live observation awaits user Quit/reopen.

Connect apps now uses the shared recessed Servers-style panel with internal scrolling and a fixed title/navigation. Claude Code and Codex each have the existing shared gray authentication key beside their heading/logo; the separate Set up buttons are removed. Their commands and Copy controls remain. The entire address/copy-address/certificate/explanation footer is removed. Approval has the shared red Close action at top right, and clicking the dimmed area hides the Fetch popup. Hiding preserves the request and selected permissions, polling leaves it hidden, and the matching app key reopens it without a new request. Shared modal framing gives approval and discovery the same 32px rounded dimming boundary and outside-click dismissal. A stable outer Stack preserves the underlying scroll state when approval appears or disappears.

Final 24 real-widget UI checks pass (9.02s), including both inline key actions, actual close/outside clicks, inside-click preservation, reopening the same code after polling, transparent popup corners, discovery popup corners, copy controls, and normal/minimum screens. Inspected 874×630 and 760×520 Connect and approval PNGs. Final signed development build-only passes (10.90s), including strict signature verification; git diff --check passes. API tasks from these fixtures are not executed.

Work and review were direct, without subagents. Fetch was not relaunched; live appearance requires explicit Quit/reopen. No provider/MCP call, client configuration, credentials, trust, signing selection, Figma, commit/push or release changes. The separate macOS Touch ID/password dialog remains OS-owned; these dismissal controls hide the Fetch dialog. Earlier product/connector acceptance limits remain.

The first test attempt stopped with No space left on device. Removed 20 Fetch incremental compilation cache directories older than one day, preserving recent compilation caches, all executables, source, app data and services. This reclaimed space for the passing retry; no personal files were cleaned.

[Evidence](evidence/info.md).

## Compact controls follow-up

Connect apps authentication buttons now use the same 22px size as their logos, with proportionally scaled key glyphs. The shared app_auth_button reuses the standard icon-button rendering/behavior; other icon actions remain 44px. Logo and compact button size share theme::APP_LOGO_SIZE. On Connect only, navigation-to-title layout spacing is halved from 56px (32px gap + 24px title padding) to 28px (16px + 12px).

24 actual-widget UI checks pass (9.39s), including clicks/reopening on both 22px app keys and existing server/popup controls. Normal/minimum Connect screenshots inspected. Signed development build-only passes (10.54s), with strict signature verification; git diff --check passes. Existing warnings remain. No new test was added; the existing inline-auth case now locates the 22px controls.

Direct work without subagents. Fetch was not relaunched; explicit Quit/reopen is needed to observe the new build. No live provider/MCP, account, credential, personal settings, trust, Figma, commit/push or release changes.

## Black authentication buttons

The two compact Connect apps authentication buttons now have black faces and white key glyphs. They stay 22px, use the same shared icon-button behavior, and do not change the Servers-page key style.

The existing real-widget inline key/dismiss/reopen check passes (1.86s); the normal-size black-button screenshot was inspected. Signed development build-only/strict signature verification passes (15.46s), and git diff --check passes. Fetch was not relaunched. Server on/off work continues separately through the selected specialist roles.

## Shared approval and refresh hover follow-up

The single approval popup used by both Claude Code and Codex now uses the shared blue primary Allow button. Request code is labeled above the existing shared code block with its Copy control, which copies only the code. Removed the requested Match this code… sentence. Install refresh keeps the shared hover glow/rotation enabled when idle even with an empty path; its existing blank-path handler still performs no discovery and starts no request.

25 existing actual-widget checks pass (9.38s), including request-code Copy, absence of the removed sentence, both app popup flows and empty-path hover animation/no-discovery. Inspected Claude and minimum-size Codex popup PNGs. Signed development build-only/strict signature verification passes (17.68s); git diff --check passes. No new test infrastructure or tests added; existing cases were extended for the requested behavior.

Direct UI work; no subagents for this pass. Fetch was not relaunched. No live provider/MCP, credential/trust/config, API/authentication permission, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the new executable.
