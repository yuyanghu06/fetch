# Fetch milestone snapshot

## Working-tree delivery — November 4 follow-up

Complete: requested source and documentation pushed and remote-confirmed. Actual OpenAI/Fetch login and live MCP/Blender acceptance remain separate product follow-up; no app/session launched or credentials copied during delivery. [Latest pass](history/2026-11-04_18-22-10-968811-orchestrator-followup-working-tree-delivery.md).

## Permanent Fetch-only Codex — setup complete

Permanent private client and reusable fetch_codex command are created; only Fetch is configured and apps/plugins are disabled. Independent configuration checks and portable-guide reproduction pass; Evaluator ACCEPT. Old temporary client was removed after exit. [Guide](../docs/testing/CODEX_TEST_CLIENT.md), [scope](persistent-fetch-only-codex/info.md), [completion](history/2026-11-04_18-19-46-687669-orchestrator-persistent-fetch-only-codex.md). Actual sign-in and live Fetch tool use remain untested. Next: new Terminal, fetch_codex login, fetch_codex mcp login fetch, then fetch_codex and /mcp.

## Python package environments and cross-Mac guide — complete

New uvx package installs delegate environments/compatible Python selection to uv, forwarding explicit Python requests and preserving private installation/account storage, package pins and supported args. Existing saved servers and standalone local scripts unchanged.32 focused checks (one new two-server preparation scenario), signed native build in 13.26 seconds, strict signature verification and whitespace checks pass; independent Evaluator ACCEPTs source and cross-Mac guide. Real Blender installation/connection remains unverified: restricted experiment stopped before uv ran, state cleaned. Fetch not relaunched. [Setup guide](../docs/testing/PYTHON_MCP_SETUP.md), [evidence](python-server-environments/evidence.md). Final passes: [tester](history/2026-11-04_18-18-52-541717-tester-verify-python-package-environments.md), [evaluator](history/2026-11-04_18-18-52-542050-evaluator-accept-python-package-environments-and-setup-guide.md), [orchestrator](history/2026-11-04_18-18-52-542214-orchestrator-integrate-python-environments-and-cross-mac-guide.md). Earlier active checkpoints below are historical for this scope; other work remains.




Python pin origin confirmed: D06 approved private tooling and sharing compatible runtimes; the universal exact Python version came from an internal installer choice, not an explicit universal-version decision. [Pass](history/2026-11-04_18-09-35-609619-orchestrator-python-pin-origin.md). No source change.

Python MCP follow-up: separate per-installation venvs already exist; the global Python 3.13.18 requirement is the problem. Codex/Claude Code document launching configured commands; uv manages uvx environments and Python requests. Using uv's own environment handling with per-server version requests is proposed only. [Research](../docs/architecture/research.md#november-4--python-mcp-environments-in-coding-clients), [pass](history/2026-11-04_18-08-39-473609-orchestrator-python-mcp-environments-research.md).

## Working-tree delivery — complete

User requested focused commits and push. Four main groups: dependencies/licenses, connected application work/tests, discovery fixes, remaining records. Source commits 2e74cf1, 36e324b and e73e172 plus documentation bcc1c01 are pushed; git ls-remote confirms origin/main at bcc1c01, matching local HEAD with a clean tree before this journal confirmation. [Delivery](history/2026-11-04_18-11-11-215570-orchestrator-confirm-scoped-push.md). [Preparation](history/2026-11-04_18-10-40-069651-orchestrator-prepare-scoped-delivery.md). Independent plan accepted; 44 safe focused checks and locked offline native build pass. Independent Evaluator ACCEPTs the staged source. [Review](history/2026-11-04_18-09-43-793838-evaluator-accept-working-tree-delivery.md). [Verification](history/2026-11-04_18-09-11-018033-tester-working-tree-delivery-checks.md). Preserve all prior limitations. [Plan](history/2026-11-04_18-08-12-781105-planner-working-tree-commit-split.md), [scope](working-tree-delivery/info.md).

## Blender MCP install — diagnosis complete

Fetch pins Python 3.13.18, but its bundled uv 0.13.5 lists downloads only through 3.13.17 for this Mac. Development Python runtime has no interpreter. This confirms a blocking preparation defect and strongly explains the failure; original installer error is unavailable because stdout/stderr are discarded and terminal output was inaccessible. No repair, install retry or MCP execution. [Diagnosis](history/2026-11-04_18-06-06-480987-orchestrator-blender-install-diagnosis.md).

## Figma Remote unsupported import — complete

Figma Remote now shows unsupported/unselected with the app-approval reason in the import/install screen. Shared classification rejects fresh file/direct installs; Figma Desktop and saved-record validation/instances remain unchanged. Two new feature tests plus existing scoped tests pass19/19; native locked build passes5.38s, scoped whitespace passes, independent Evaluator ACCEPT. Native appearance unobserved; app was not launched. Next user action: explicitly Quit, signed relaunch and refresh imports. [Scope](figma-remote-unsupported/info.md), [Evaluator](history/2026-11-04_18-07-32-270262-evaluator-accept-figma-unsupported.md), [Completion](history/2026-11-04_18-07-32-270263-orchestrator-complete-figma-unsupported.md).

## Figma remote registration — diagnosis complete

User-reported registration403 strongly matches Figma's approved-client policy. Fetch sends its own name; current Figma metadata makes rmcp fall back to dynamic registration. Callback/listener source matches; exact rejection reason is not returned. No app/code/config/account changes or live registration. Subsequent user choice: mark Figma Remote unsupported in the import/install screen (active scope above). No approval request/workaround selected. [Researcher](history/2026-11-04_18-01-51-215696-researcher-figma-registration-policy.md), [Orchestrator](history/2026-11-04_18-01-51-215697-orchestrator-figma-registration-diagnosis.md), [primary-source findings](../docs/architecture/client-identity-adoption.md#figma-registration-rejection--november-4-2026).

GitHub executable identity freshly verified: current entry uses global Homebrew gh through an imported local script; no Fetch-installed gh/package folder. It still receives private HOME. No auth/Keychain execution or code change. [Inspection](history/2026-11-04_17-54-37-382153-orchestrator-confirm-actual-global-github-cli.md).

## Keychain / command lookup — research complete, behavior unchanged

User requested HOME matching during local/global command lookup. Shell/source audit establishes PATH has no per-exec environment callback; shared runtimes and absolute helper paths prevent reliable account selection by location alone. Thin wrappers would cover only their explicit launches. Private instance HOME remains; no wrapper or Keychain setup implemented. [researcher](history/2026-11-04_17-51-29-280200-researcher-home-by-command-lookup-limits.md), [planner](history/2026-11-04_17-51-29-280201-planner-audit-home-selection-boundaries.md), [tester](history/2026-11-04_17-51-29-280202-tester-restore-withdrawn-home-fixture-edits.md).

## Keychain / instance login clarification — private HOME retained

User requires login for a locally installed CLI to remain with that Fetch instance. Normal-HOME proposal withdrawn; brief source change restored before any runtime test/launch. Packages/helper PATH preserved. Keychain fix remains research-only. [Correction](history/2026-11-04_17-43-46-982773-orchestrator-preserve-instance-logins-restore-private-home.md).

## Scrolled terminal pointer coordinates — complete

Fixed press/move coordinate mismatch through a scrolled page; normal clicks no longer jump selection rows in the reproduced case. One actual outerScrollable click/drag/clear render test failed before and passes after; final11terminalUI checks1new/native build6.90s/whitespace PASS, independent EvaluatorACCEPT. [Evidence](terminal-pointer-coordinates/evidence.md). Previous tests lacked actual parent input; earlier release/trackpad fixes preserved. User native reload/observation remains.

Final passes: [Tester](history/2026-11-04_17-41-23-628273-tester-scrolled-terminal-click-final.md), [Evaluator](history/2026-11-04_17-41-23-628274-evaluator-accept-scrolled-terminal-pointer.md), [Orchestrator](history/2026-11-04_17-41-23-628275-orchestrator-integrate-scrolled-terminal-pointer.md). Earlier active checkpoints below are historical.

## Normal system HOME / Keychain — research complete, choice pending

Conditional plan complete; source untouched. Fixture review found a pre-confinement HOME PID write that must be repaired before any opt-in real fixture run. [Planner pass](history/2026-11-04_17-41-29-250820-planner-normal-system-home-conditional-scope.md).

Apple Security directly reads HOME; Fetch currently changes it. Proposed normal HOME inheritance keeps private XDG/packages/cwd but allows shared service Keychain logins and HOME files. Explicit user choice pending before source changes. Conditional plan checks fixture safety; no auth or Keychain state touched. [Scope](normal-system-home/info.md), [research](history/2026-11-04_17-41-05-958357-researcher-macos-keychain-home.md).

## Selected Codex discovery — complete November 4

Selecting `.codex` must read only its `config.toml`. The current parent expansion incorrectly includes Claude/OpenCode configs. Read-only Planner confirms a bounded shared-reader fix. Independent Tester reproduced the original HTTP failure: selected `.codex` returns three source groups, expected one. [Reproduction](history/2026-11-04_17-41-19-076794-tester-reproduce-selected-codex-discovery.md). Source now passes only the selected `config.toml` to the shared candidate reader; MCP/API descriptions match. Independent verification passes: 10 HTTP discovery tests and native build (7.55s). [Verification](history/2026-11-04_17-42-50-205423-tester-selected-codex-discovery-verification.md). Independent Evaluator ACCEPT, no findings. [Evaluation](history/2026-11-04_17-43-20-236061-evaluator-accept-selected-codex-discovery.md), [completion](history/2026-11-04_17-43-20-236548-orchestrator-complete-selected-codex-discovery.md). No app launch or personal config changes. Next: explicitly Quit Fetch, run `./scripts/run-development.sh`, then refresh discovery. [Source pass](history/2026-11-04_17-42-21-122265-coder-selected-codex-discovery.md). Automatic/home/single-file behavior stays selected. [Plan](history/2026-11-04_17-40-29-854519-planner-selected-codex-discovery.md), [scope](selected-discovery-scope/info.md). Other active work below is preserved.

Real scrolled-parent click bug reproduced (1px move interpreted500px away); two-line movement routing now uses the adjusted Cursor consistently. Independent11focused terminalUI checks/build/review underway. [Reproduction](history/2026-11-04_17-39-37-056474-tester-reproduce-scrolled-pointer-jump.md), [fix](history/2026-11-04_17-39-37-056475-coder-use-translated-terminal-pointer.md).

## Scrolled terminal pointer coordinates — active

User reports large selection on normal clicks. Found press/move coordinate mismatch when outer page is scrolled; prior release/trackpad fixes remain. Planner confirms Iced translates child cursor but raw move event stays unchanged. One actual outerScrollable interaction regression is preparing before two-line source fix. [Scope](terminal-pointer-coordinates/info.md), [report](history/2026-11-04_17-37-47-573720-orchestrator-recurring-terminal-click-selection.md), [plan](history/2026-11-04_17-37-47-573721-planner-scrolled-terminal-pointer.md).

gh debug capture instructions prepared: same terminal context, private log, ordinary verbose output and read-only Keychain path metadata. Await user log; cause remains unconfirmed. [Pass](history/2026-11-04_17-36-49-255671-orchestrator-gh-auth-debug-capture-guidance.md).

gh auth login screenshot diagnosis: macOS cannot locate a usable Keychain for credential saving; private Fetch HOME is an unconfirmed hypothesis. No repair or Keychain changes. [Read-only pass](history/2026-11-04_17-35-15-506420-orchestrator-gh-keychain-prompt-diagnosis.md).

Public identity adoption research is complete: CIMD is the recommended remote MCP registration direction, with confirmed provider support and uneven adoption. The same Fetch file can serve multiple supporting services; each still has separate account consent and tokens. [Research](../docs/architecture/client-identity-adoption.md). Passes: [researcher](history/2026-11-04_17-27-31-163759-researcher-research-cimd-adoption.md), [orchestrator](history/2026-11-04_17-27-31-163760-orchestrator-record-client-identity-adoption-research.md). No code or settings changed.

## Terminal selection and scrolling — complete

Selection drag now ends on release outside the pane. Trackpad direction matches mouse wheel; fractional gestures stay inside Terminal and can reveal older output. Intentional selection/current cursor unchanged. Original2failures reproduced; final10focused widget/render tests (2new), native build8.30s and whitespace PASS; independent Evaluator ACCEPT. [Evidence](terminal-selection-scroll/evidence.md). User rebuilt native window still needs explicit Quit/relaunch observation. No shell/provider/credentials used.

Final passes: [assertion cleanup](history/2026-11-04_17-27-17-961032-coder-align-terminal-wheel-expectations.md), [Tester](history/2026-11-04_17-27-17-961033-tester-terminal-selection-scroll-final.md), [Evaluator](history/2026-11-04_17-27-17-961034-evaluator-accept-terminal-selection-scroll.md), [Orchestrator](history/2026-11-04_17-27-17-961035-orchestrator-integrate-terminal-selection-scroll.md). Earlier partial checkpoints below are historical.

Terminal input fix saved after original failures reproduced: release outside ends selection drag, pixel scrolling uses correct sign and stays in pane even below one line. Independent10terminalUI tests/native build/review underway. [Source](history/2026-11-04_17-25-22-061363-coder-terminal-selection-scroll-fix.md).

Terminal input failures independently reproduced on original widget: outside-release reentry extends selection and fractional trackpad wheel escapes pane. Two new realistic widget/render tests fail as expected; Coder now applying narrow fix. [Reproduction](history/2026-11-04_17-24-37-162870-tester-reproduce-terminal-input-bugs.md).

Terminal input plan verified: release outside must end drag; pixel-wheel sign is reversed and fractional gestures escape to outer page. Two realistic widget/render tests are reproducing original failures before source edits. [Plan](history/2026-11-04_17-23-14-983141-planner-terminal-selection-scroll.md).

## Terminal selection and scrolling — active

User confirms unwanted gray selection and cannot scroll up. Proven source issue ignores an active drag release outside pane; pixel-wheel direction/fractional capture under review. Preserve normal selection/current cursor style; reuse widget/Alacritty. Plan/source/test specialists assigned exclusive files. [Scope](terminal-selection-scroll/info.md), [pass](history/2026-11-04_17-22-04-649460-orchestrator-terminal-selection-scroll-scope.md).

## Normal command-search PATH — complete

Shared startup/setup/Terminal now retains inherited command folders after private programs and executable parent. Explicit PATH/private account folders unchanged.23 focused checks including1actual confined bare-gh/private-priority test, native build8.40s and whitespace PASS; independent Evaluator ACCEPT. [Evidence](normal-program-path/evidence.md). No personal gh/login/native app execution. Next: explicitly Quit Fetch, ./scripts/run-development.sh, open fresh Terminal. Inherits the PATH available to Fetch; no shell-profile loading.

Final passes: [Tester](history/2026-11-04_17-14-41-049864-tester-normal-program-path-verification.md), [Evaluator](history/2026-11-04_17-14-41-049865-evaluator-accept-normal-program-path.md), [Orchestrator](history/2026-11-04_17-14-41-049866-orchestrator-integrate-normal-program-path.md). Earlier active checkpoints for this feature below are historical; unrelated active work stays current.

Final website deployment is READY at commit7891229; its public identity file again returns direct HTTP200/JSON with the correct IP callback. Task-created site worktree/build copies were removed; original dev changes are preserved. Fetch source, focused tests, build and independent review are complete. [Final verification](history/2026-11-04_17-14-17-221908-orchestrator-final-identity-deployment-verification.md).

## Public client identity and IP callback — complete November 4

The [public identity file](https://www.yuyang-hu.org/oauth-client.json) is deployed and verified: direct HTTP 200, JSON and the expected content. The homepage also returns HTTP 200. Fetch passes this URL to rmcp and receives new interactive sign-in through http://127.0.0.1:<temporary-port>/oauth/provider/callback. Its main HTTPS API and certificate are unchanged.

One new full feature test, two existing regressions and the native build pass. The independent evaluator reports ACCEPT. No real Vercel account sign-in or Fetch launch was performed. [Evidence](client-identity/evidence.md). Next user action: explicitly Quit Fetch, run ./scripts/run-development.sh and retry Vercel. Earlier pending checkpoints below are historical.

Finalpasses: [evaluator](history/2026-11-04_17-12-37-774578-evaluator-accept-static-identity-candidate.md), [coder](history/2026-11-04_17-12-37-774579-coder-implement-temporary-ip-callback.md), [tester](history/2026-11-04_17-12-37-774580-tester-verify-public-identity-loopback-flow.md), [evaluator](history/2026-11-04_17-12-37-774581-evaluator-accept-public-identity-and-ip-callback.md), [orchestrator](history/2026-11-04_17-12-37-774582-orchestrator-integrate-published-identity-and-ip-callback.md). Earlierhosting/listenerpendingnotesbelowarehistorical.

Normal PATH source implemented; independent confined terminal/helper checks and review underway. [Plan](history/2026-11-04_17-11-43-880643-plan-inherited-program-path.md), [source](history/2026-11-04_17-11-43-880644-code-inherited-program-path.md). No feature test/build acceptance yet. Other active work preserved.

## Normal command-search PATH — active

User approves inherited normal command folders after private server programs, fixing bare gh lookup in startup and Terminal. Preserve explicit PATH/private account folders; no shell profiles or personal auth. Read-only planning underway; source/tests next. [Scope](normal-program-path/info.md), [approval](history/2026-11-04_17-10-05-626636-orchestrator-approve-inherited-command-path.md).

## Public client identity and IP callback — implementing

User selects personal site yuyang-hu.org, using canonicalwww identity URL to avoid its existing redirect. StaticJSON prepared in isolated production/main worktree; defaultsitebuildPASS18pages. User explicitly selects temporaryHTTP127callback; mainHTTPS/cert remains unchanged. SDKmetadataURL wiring saved; dedicated callback receiver and one realistic integrationtest underway. Site review/publication next. Passes: [planner](history/2026-11-04_17-06-10-724166-planner-plan-personal-site-client-identity.md), [coder](history/2026-11-04_17-06-10-724167-coder-configure-library-client-metadata-url.md), [coder](history/2026-11-04_17-06-10-724168-coder-prepare-personal-site-static-identity.md), [researcher](history/2026-11-04_17-06-10-724169-researcher-research-ip-loopback-callback.md), [planner](history/2026-11-04_17-06-10-724170-planner-plan-temporary-http-callback.md).

Latest read-only GitHub inspection: existing dev local-script launcher already calls global gh by absolute path and passes its returned token to GitHub MCP2.0.2. No downloaded package at recorded ID. Terminal PATH omits Homebrew; private account configuration remains separate. No auth executed or behavior changed. [Inspection](history/2026-11-04_17-04-23-846865-orchestrator-inspect-installed-github-cli-launcher.md).

Latest documentation-only direction: GitHub should default to globally installed gh because its install script does not bundle it (user report). Saved in NOTES.md/architecture decisions; no source/runtime change. [Note pass](history/2026-11-04_15-32-08-073540-orchestrator-note-global-github-cli.md). Existing feature acceptance and other active work below remain unchanged.

Latest read-only clarification: public app-registration metadata can list a local native callback if the provider accepts it; no hosting/authentication change selected. [Research pass](history/2026-11-04_15-28-29-615396-researcher-public-metadata-local-callback.md). Console/Terminal accepted state below remains current.

## Server Console/Terminal and saved logs — complete scoped implementation

Expanded rows share Console/Terminal; local terminals use the account system shell and exact MCP environment, retained through pane/window changes. Sanitized Console history survives restarts under approved newest-output limits; terminal input/transcript excluded. User-reported blank-pane drawing is corrected using widget-local cache/translation/clipping.

Independent182 relevant checks (19new), actual offscreen prompt/cursor/scroll/clip render, owned shell/foreground cleanup, locked native build16.25s and whitespace PASS. Evaluator ACCEPT. [Evidence](server-console-terminal/evidence.md). Actual rebuilt native window/account login remain unobserved; explicitly Quit then ./scripts/run-development.sh. One initial ordinary server-read failure did not recur in full rerun/five repeats; cause unconfirmed and preserved in evidence.

Final passes: [Tester](history/2026-11-04_15-28-11-920855-tester-console-terminal-final-verification.md), [Evaluator](history/2026-11-04_15-28-11-920856-evaluator-accept-console-terminal.md), [Orchestrator](history/2026-11-04_15-28-11-920857-orchestrator-integrate-console-terminal.md). Earlier partial checkpoints below are historical.

## Server Console/Terminal — blank-pane correction

User screenshot establishes native blank-pane failure. Source now follows Iced Canvas local geometry, draw-time translation and clipping for embedded/scrolled terminals. Independent offscreen render check, final core tests/build/review underway. [Correction pass](history/2026-11-04_15-24-11-939731-coder-fix-embedded-terminal-drawing.md). Earlier unobserved-appearance statements are historical; rebuilt native appearance remains unobserved.

Public-client-identity bounded plan ready: static identity JSON plus existing rmcp with_client_metadata_url option; keep local callbacks and library-owned negotiation. Actual hosting URL unselected. Latest AGENTS.md test rule narrows new coverage to realistic feature flow. [Planner pass](history/2026-11-04_15-22-44-343477-planner-public-client-identity-wiring.md).

## Client identity — scope/hosting discussion

User approves configuring rmcp with Fetch identity JSON and asks hosting/code details. Existing repo has no homepage; Pages lookup404. Public HTTPS URL remains unselected. Static file and one SDK option are the bounded implementation; no hosted login backend. Latest user test preference is now saved at top of AGENTS.md: small realistic feature tests, no speculative matrices. [Scope](client-identity/info.md), [pass](history/2026-11-04_15-22-15-847071-orchestrator-focus-tests-and-scope-client-identity.md).

Both terminal findings are corrected in source and await final independent retest/build: [startup replies](history/2026-11-04_15-20-59-717647-coder-fix-fresh-terminal-queries.md), [close/reap](history/2026-11-04_15-20-59-717648-coder-fix-terminal-close-reap.md). No new API/schema or provider-login flow.

Checked Vercel callback requirements: HTTPS, custom scheme, or HTTP loopback; no fixed universal callback path. Current Fetch HTTPS callback matches a documented category, while Fetch still lacks the public identity-document setup for unreviewed clients. No auth/source/hosting change selected. [Pass](history/2026-11-04_15-19-13-027898-orchestrator-check-vercel-callback-requirements.md), [research](../docs/architecture/research.md).

## Server Console/Terminal — review corrections active

Saved-log reload/privacy/limits, native route authority, UI state and confined shell environment/resize/exit checks pass. Two material terminal findings are being fixed: close must recognize an already exited shell after HUP, and fresh sessions must answer startup terminal questions instead of treating them as replay. Two old prefix-only assertions need the approved newest-output expectation. Final tests/build/evaluation remain. [Review pass](history/2026-11-04_15-18-18-541564-evaluator-terminal-initial-findings.md).

Vercel user retry now reaches registration and receives provider HTTP400 invalid_redirect_uri. Default development return address is https://localhost:53672/oauth/provider/callback; actual port override unobserved. Explained browser-return purpose and checked official identity requirements; no new auth configuration/hosting selected. [Pass](history/2026-11-04_15-12-27-725543-orchestrator-vercel-redirect-rejection.md), [research](../docs/architecture/research.md).

Terminal backend/UI source handoff: [pass](history/2026-11-04_15-11-31-729995-coder-terminal-backend-and-ui-integration.md). Source review corrected blocked-input close and final-output ordering; independent test/review underway.

## Server Console/Terminal — source integration and tests

Persistent console source is ready for independent checks. Backend and UI are finishing close/output handling; the independent Tester owns new PTY/persistence/UI fixtures. No current feature test/build acceptance yet. [Source handoff](history/2026-11-04_15-09-55-228986-coder-persistent-console-history.md). Exact user-approved contract: [Console/Terminal](../docs/architecture/server-terminal.md).

## Wrapper authentication — complete November 4

Removed six extra Fetch provider-login gates. Official rmcp owns discovery/fallback and callback validation; saved-login endpoint/issuer/client identity, consent and actual-request access rules remain.43 independent auth/storage/authority checks PASS; current-tree locked native build PASS21.68s; final Evaluator ACCEPT. [Evidence](wrapper-authentication/evidence.md). No real Vercel sign-in or app launch. Newest unrelated console-test wiring prevents another library-test repeat, outside scoped acceptance. Final passes: [tester](history/2026-11-04_15-09-19-022329-tester-wrapper-auth-native-build-pass.md), [evaluator](history/2026-11-04_15-09-19-022330-evaluator-accept-wrapper-authentication.md), [orchestrator](history/2026-11-04_15-09-19-022331-orchestrator-complete-wrapper-authentication.md). Next user action for actual login: explicitly Quit Fetch, then ./scripts/run-development.sh and retry; no auth correction remains. Earlier pending/build-failure checkpoints below are historical.

## Codex startup environment — complete November 4

Fetch now adapts the actual pinned Codex MCP environment builder and its variable lists. Fetch's private HOME/XDG folders and existing helper PATH are explicit overrides; executable/args/cwd, assigned inputs, omission of missing inputs, cleared child environment and service-owned credentials remain. Full upstream Apache license/NOTICE and modified-source attribution are retained. No Codex authentication/store code or new dependency/API/schema is imported.

Independent24integration checks plus8pure helper checks pass; native build passes; Evaluator ACCEPT. Both ordinary startup and recorded setup received asserted args/cwd/private folders/helper PATH/LANG in real confined children. Full library tests are blocked by unrelated in-progress terminal test wiring, so no full-suite claim is made. [Scope](codex-startup-environment/info.md), [evidence](codex-startup-environment/evidence.md).

Followup: run focused library filters once concurrent terminal test files are complete. No app/provider sign-in, personal configuration edits, signing/release, commit or push occurred. Existing failed explicit Restart→Authenticate observation remains separate.

Final passes: [tester](history/2026-11-04_15-11-39-990275-tester-codex-environment-build-and-fixtures.md), [evaluator](history/2026-11-04_15-11-39-990276-evaluator-accept-codex-environment-replacement.md), [orchestrator](history/2026-11-04_15-11-39-990277-orchestrator-complete-codex-environment-replacement.md).

## Wrapper authentication — source implemented, final build pending

Removed six extra Fetch compatibility gates; rmcp owns discovery and callback validation.43 independent auth/storage/authority checks PASS; Evaluator finds no auth changes needed, but REQUEST CHANGES for final build verification. Build retry passed log_store integration then failed in concurrent iced_term notifier code (E0596); final auth retest also awaits newly declared logging test file. Preserve other owners’ work. [Evidence](wrapper-authentication/evidence.md). Next: once terminal integration settles, locked native build + three fixture filters, then final evaluation. Passes: [tester](history/2026-11-04_15-07-07-764799-tester-wrapper-auth-build-retry-terminal-blocker.md), [orchestrator](history/2026-11-04_15-07-07-764800-orchestrator-integrate-wrapper-auth-removal.md).

Wrapper-auth evaluator finds source/tests acceptable and requests final native build only. Concurrent log_store module is now saved/wired; Tester retrying build. [Review](history/2026-11-04_15-05-51-025226-evaluator-wrapper-auth-await-native-build.md).

Wrapper-authentication independent regressions PASS43/43 (remoteOAuth26, background6, native-storage11). Final native build is waiting on separate concurrent log_store integration: E0433 in runner startup diagnostics references a module not saved yet. No auth source finding. [Pass](history/2026-11-04_15-04-56-319997-tester-wrapper-auth-regressions-and-build-dependency.md).

Latest pass: [approved contract and dependency preparation](history/2026-11-04_15-04-30-247160-orchestrator-approve-terminal-routes-and-dependencies.md); source implementation active, no tests/build acceptance yet.

Wrapper-auth initial independent tests:24/25passed; one fixture wrongly expected storage before callback and is corrected. Retest briefly hit concurrent terminal dependency/lock mismatch; updated lock now visible, locked/offline retry underway. No production finding so far. [Pass](history/2026-11-04_15-02-42-621425-tester-wrapper-auth-initial-verification.md).

Wrapper-authentication source removal is implemented: four extra preparation gates, unknown callback-field rejection and full-document metadata reuse rejection are removed. SDK callback validation remains. Independent fixtures/build/review underway. Passes: [planner](history/2026-11-04_15-00-09-483482-planner-plan-library-owned-authentication.md), [researcher](history/2026-11-04_15-00-09-483483-researcher-audit-wrapper-authentication-restrictions.md), [coder](history/2026-11-04_15-00-09-483484-coder-remove-pre-login-and-callback-gates.md), [coder](history/2026-11-04_15-00-09-483485-coder-remove-full-metadata-reuse-gate.md).

## Permanent expanded console — complete November 4

Every expanded server shows the shared console, including empty and not-required states. Read-only display updates do not authenticate or mark it busy. Sanitized output remains from startup through Authenticate/manual save, collapse and window reopen within the current Fetch run. Fresh attempt IDs/redaction state, accurate exit facts, stale-handle freezing and existing limits remain.

The review correction preserves a fresh local request failure beside old history until new service output arrives. Authentication, issuer checks, account folders and credential stores are unchanged. No Codex code imported.

Independent **150 relevant tests**, including **eight new tests**, native build and whitespace pass. Evaluator **ACCEPT**. [Evidence](continuous-server-console/evidence.md). Native appearance and real account acceptance remain untested; the separately observed failed explicit Restart → Authenticate SERVER_BUSY behavior remains open.

Next: explicitly Quit Fetch, run ./scripts/run-development.sh and expand a server. No app launch, personal settings, signing/release, commit or push action was performed.

Final passes: [tester](history/2026-11-04_14-59-13-496977-tester-continuous-console-final-verification.md), [evaluator](history/2026-11-04_14-59-13-496978-evaluator-accept-continuous-expanded-console.md), [orchestrator](history/2026-11-04_14-59-13-496979-orchestrator-complete-continuous-expanded-console.md).

Earlier dated checkpoints below preserve partial results; this section is current for console continuity.

## Wrapper authentication — active November 4

User authorizes removal of Fetch-imposed login compatibility restrictions. Official MCP library owns discovery/callback validation; existing request-time network checks, consent and credential boundaries stay. Planner/audit underway; implementation/tests next. [Scope](wrapper-authentication/info.md), [pass](history/2026-11-04_14-57-55-339924-orchestrator-select-wrapper-authentication.md).

## Expanded Console/Terminal and saved logs — active

User approved feature/system shell and exact new GET/POST route/persistent-retention contract. Three exclusive source writers are implementing UI/vendor renderer, backendPTY/API and saved Console logs; dependencies resolved. [Proposal](../docs/architecture/server-terminal.md), [scope](server-console-terminal/info.md), [pass](history/2026-11-04_14-56-54-791135-planner-console-terminal-persistence-proposal.md). Earlier always-console memory-only plan superseded by this expanded user request. Existing independently implemented continuous-console changes are preserved/reused.

Continuous-console final checks pass the library and startup/history targets. One old authentication test still expected logs to be erased; it is being updated to the new retained-history contract while preserving privacy/fresh-attempt checks. [Test pass](history/2026-11-04_14-56-09-979466-tester-update-superseded-history-expectation.md).

Custom issuer-check origin traced to the October29 security review and retained during the November3 SDK refactor. This is an explanation, not a new authentication-policy decision or fix. [Pass](history/2026-11-04_14-55-42-214895-orchestrator-trace-custom-issuer-check.md).

The current-request error correction is implemented and under final independent retest. Actual unpolled app-startup-to-auth history passed. [Correction pass](history/2026-11-04_14-55-24-348500-coder-preserve-new-errors-with-history.md).

Vercel authentication log explained: discovery succeeds, then Fetch stops because the service does not advertise its required authorization-response issuer check. No policy/code change or live sign-in. [Pass](history/2026-11-04_14-53-40-702140-orchestrator-vercel-authentication-log-explanation.md).

Continuous-console review found a real display regression: old service history can hide a new local request error. Source owner is fixing the current-request output distinction; tests are extended before acceptance. Initial UI checks pass, remaining checks/build continue. Passes: [tester](history/2026-11-04_14-52-54-031825-tester-initial-continuous-console-checks.md), [evaluator](history/2026-11-04_14-52-54-031826-evaluator-request-current-error-history-fix.md).

Continuous-console source is implemented. Expanded rows always use the shared console, and backend snapshots retain sanitized history across startup/auth attempts. Independent checks are compiling; no pass claimed yet. [Coding pass](history/2026-11-04_14-50-41-924188-coder-continuous-expanded-console.md).

Read-only startup reuse refinement: Codex’s environment builder is the useful startup-input piece; its macOS command resolver is a no-op. Fetch already supplies its name/version and configured launch arguments. Credential storage is unchanged. [Research addendum](../docs/architecture/railway-codex-startup-research.md#november-4-follow-up--exact-reusable-startup-input-pieces), [pass](history/2026-11-04_14-48-54-032872-orchestrator-clarify-codex-startup-input-reuse.md). Console implementation continues.

## Continuous expanded console — active November4

User requests consolealwaysvisible in expandedserver and startup→authhistory. Credentialstore remainsserviceowned. Independentplan selects passivedisplay and boundedalready-sanitizedhistorycarryover without newauthbehavior/storage. [Scope](continuous-server-console/info.md), [plan](history/2026-11-04_14-47-15-845406-planner-continuous-expanded-console.md). Source/independenttestsunderway. Earlierloggingacceptance below remains valid for prior scope.

## Service startup logs and wrapper architecture — complete November 4

Fetch’s default is to wrap existing code and outputs unless the user explicitly says otherwise. Recorded in AGENTS.md and canonical decisions/context/notes. Console displays actual startup service logs/instructions/errors and observed process exit facts; invented authentication progress and protocol-error substitution are removed.

Shared private per-attempt capture covers ordinary startup, recorded setup, MCP/HTTP rejected replies and OAuth failure stages. Retain the beginning with1MiB4096line64KiBline limits/explicit notices; sanitize before retention. Native-only memory projection, no disklog or successful protocol/tool/token dumps. Existing auth/credential/public API/raw forwarding behavior remains. [Evidence](service-startup-logs/evidence.md).

Independent184uniquechecks including15new PASS; final31UIrecheckPASS; lockednativebuild9.33swall and whitespacePASS. EvaluatorACCEPT; no unresolved materialfindings. No nativeapp/account/signing/release/personalstate/commit/push actions. Next: explicitlyQuit, run ./scripts/run-development.sh and retry desired server.

Actual confined Railway startup initializes, then tools/list returns-32001notloggedin with service-owned instructions; no guessed logincommand executed, packages/processes cleaned. [Railway/Codex report](../docs/architecture/railway-codex-startup-research.md). Codex source Apache2.0, same RMCPfamily plus identity/storage/environment glue; wholecrate brings12internaldependencies. No codecopy/new authdesign selected.

Final passes: [tester](history/2026-11-04_14-41-14-671262-tester-service-startup-logs-final-checks.md), [evaluator](history/2026-11-04_14-41-14-671263-evaluator-accept-service-startup-logs.md), [orchestrator](history/2026-11-04_14-41-14-671264-orchestrator-complete-service-startup-logs.md).

Earlier checkpoints below are historical; completed logging scope above supersedes their pending-log statements.

## Follow-up login failures — explanation only

Vercel client identity compatibility is missing in current setup; its generic error groups multiple possible failures. Railway now reports a protocol rejection; existing CLI login in Fetch’s private HOME is a possible prerequisite, not a confirmed cause. Original messages are not exposed. No source changes or live sign-in tests. [Read-only pass](history/2026-11-04_14-14-44-908353-orchestrator-explain-vercel-railway-login-errors.md). Startup lookup acceptance below remains scoped to program discovery.

## Private helper programs at startup — complete November 4

Managed MCPs can now find helper programs installed for their own instance. New installations record relative executable folders privately; older rows use the installer’s known layout in memory, so the current Railway package needs no reinstall. Shared ordinary startup and recorded setup prepend those folders, preserving configured/default PATH after them. No recursive scan, global inheritance, public API or login/credential changes.

Independent 60 checks (10 new) and locked native build pass; Evaluator ACCEPT. Actual confined child tests prove bare-name helper execution for ordinary legacy startup and recorded setup, with a negative lookup in another installation and private metadata projection. One test synchronization issue was fixed; no product finding remained. [Evidence and commands](private-program-paths/evidence.md).

Next: explicitly Quit Fetch, run ./scripts/run-development.sh and retry Railway Authenticate. No app launch, actual account login, personal-state edits, signing or release ran in this pass. The startup lookup fix is complete; Railway account acceptance remains untested.

Final passes: [tester](history/2026-11-04_13-30-26-108163-tester-verify-private-program-folders.md), [evaluator](history/2026-11-04_13-30-26-108164-evaluator-accept-private-program-folders.md), [orchestrator](history/2026-11-04_13-30-26-108165-orchestrator-complete-private-program-folders.md).

## November 4 — Railway fails before sign-in because its CLI is not found

User-provided startup log matches @railway/mcp-server 0.1.12's missing-executable diagnostic. Its published and installed wrapper spawns `railway mcp`; it prints the deprecation/install message on ENOENT (the executable cannot be found) and exits 1. The installed development package is 0.1.12 and has its private Railway CLI present under the sibling railway-cli installation. Fetch still launches the wrapper, clears inherited process environment, and defaults PATH to the primary executable directory plus /usr/bin:/bin. It does not add the separately prepared Railway CLI bin directory. A global Railway executable exists under /opt/homebrew/bin, also outside that default PATH.

This is an executable lookup failure before account authentication, not evidence of invalid Railway credentials. Recommended repair: expose the already prepared private CLI to the server process. No per-server code/config change, login, MCP execution, personal settings edit or installation was performed. Account login after repairing startup remains untested.

Sources: [Railway MCP instructions](https://docs.railway.com/cli/mcp), [published wrapper metadata/archive](https://registry.npmjs.org/@railway%2fmcp-server/0.1.12); source inspection of src/installation/mod.rs, src/manager/native_state.rs and src/runner/transport.rs.

Latest pass: [Railway diagnosis](history/2026-11-04_12-59-56-434568-orchestrator-railway-cli-not-on-startup-path.md).

## November 4 — Google MCP README and actual fresh installation

The requested MCP is @a-bonus/google-docs-mcp, not OpenAI’s built-in plugin. Its [1.11.3 README](https://github.com/a-bonus/google-docs-mcp/blob/v1.11.3/README.md#quick-start) says to create or select a Google Cloud project, enable the APIs, configure consent, create a Desktop app client ID, supply its ID/secret, and then authorize. Remote deployment supplies those app settings on the host instead. This does not establish who originally configured the user’s existing local MCP.

A real fresh npm install of 1.11.3 succeeded in 7.69 seconds in a task-only folder with empty HOME/npm configuration, separate cache and install scripts disabled. No credentials.json was included; installed auth code reads supplied environment values or an external credentials.json. No MCP/server/login was run. Temporary files/cache removed. Existing Google definitions already have app settings; Fetch currently omits those values on import. No need to recreate the project is established, and no import-policy change was implemented.

Passes: [58-16-392527-researcher-google-mcp-developer-setup.md](history/2026-11-04_12-58-16-392527-researcher-google-mcp-developer-setup.md), [58-16-392528-orchestrator-fresh-google-install-and-readme.md](history/2026-11-04_12-58-16-392528-orchestrator-fresh-google-install-and-readme.md).

## Google research evidence clarification — November 4

Earlier research inspected the published package archive; it did not run a
fresh-install authentication test. Found client values belonged to existing
MCP definitions, not bundled defaults. [Read-only evidence audit](history/2026-11-04_12-30-24-946373-orchestrator-clarify-google-research-evidence.md).

## Discovery install explanation — November 4

Source review confirms selected npm/PyPI packages are freshly installed using
package managers; scripts remain in place; remote definitions need no package.
Existing saved associations may be reused. Connect follows preparation. This
does not carry over omitted environment/header values. No code changes or
execution. [Latest read-only pass](history/2026-11-04_12-26-37-011573-orchestrator-explain-discovery-install.md).

## Google startup cause — confirmed November 4

Neither original nor managed Google package has credentials.json; both are
version 1.11.3. Original definitions supply literal nonempty app credentials in
environment values, which Fetch’s import deliberately omits. auth.js reads its
own package root, so working directory is not the file-lookup cause. No values
were printed/copied or account validity tested. Preserving app configuration
during import is recommended; no policy/code change made.

Latest pass: [read-only investigation](history/2026-11-04_12-25-29-917436-orchestrator-google-existing-app-config.md).

## Console Copy button — complete November 4

Add a small fixed Copy button at the top right of the existing console. It copies
the displayed log lines in order through the existing clipboard action. No API
or logging changes. Implementation is present; whitespace and independent locked build pass (9.80s).
Evaluator ACCEPT. No native window or clipboard execution was performed.

Final passes: [evaluator](history/2026-11-04_12-22-41-742031-evaluator-console-copy.md), [orchestrator](history/2026-11-04_12-22-41-742032-orchestrator-console-copy.md).

Build evidence: [tester](history/2026-11-04_12-22-22-071046-tester-console-copy.md).

Latest implementation: [coder](history/2026-11-04_12-21-50-031614-coder-console-copy.md).

Plan: [planner](history/2026-11-04_12-21-13-502846-planner-console-copy.md).

## Startup behavior and error reporting — complete November 4

Missing configured environment/header values no longer block any startup path.
Fetch supplies values that exist, omits missing ones, and attempts the configured
command or remote connection. The server decides its own requirements.

Configured local startup errors now reach the native console as a bounded,
sanitized stderr tail with natural exit code/signal. Only native Settings reads
receive it; ordinary API/MCP responses and saved records do not. New attempts
clear old evidence. Ordinary failed Start/Connect also shows the console without
an extra authentication request. Pending children are reaped before Quit returns.

Independent 181 checks pass; the final small display extension passes 31 targeted
UI checks. Final locked build passes in 8.58s; whitespace passes; Evaluator ACCEPT.
[Evidence and limits](startup-diagnostics/evidence.md).
The reported Google failure's actual cause is still unknown because the old build
discarded its output. Next: explicitly Quit, run ./scripts/run-development.sh and
retry Google to see its own startup message. No real account/browser/native app,
signing/release or personal-state changes were performed. Recorded helper logs
and remote response bodies are outside this startup report. Changes remain local.

Final passes: [evaluator](history/2026-11-04_12-09-23-374073-evaluator-accept-startup-diagnostics.md), [orchestrator](history/2026-11-04_12-09-23-374074-orchestrator-complete-startup-diagnostics.md).

## Local startup diagnostics verified — final integration

Configured localstartupstderr andnaturalexitstatus now enter a bounded sanitized
private buffer; onlynativeSettings GET adds it toconsole. Nothing entersordinary
Manager/MCP/diskrecords.181independent checks pass after fixing awaitedQuitof
provisionalchild; unchanged immediatePIDassertion nowpasses.
RootaddedordinaryfailedStart/Connect consolerehydration so noAuthenticateclickis
required;31targetedUIchecks pass, finalbuild/reviewpending. ActualGooglecause
remainsunknownfromdiscardedoldrun; no realaccount/nativeprocessreproduction.

Passes: [coder](history/2026-11-04_12-07-13-468390-coder-local-startup-diagnostics.md), [orchestrator](history/2026-11-04_12-07-13-468391-orchestrator-native-startup-console-projection.md), [tester](history/2026-11-04_12-07-13-468392-tester-startup-diagnostic-cleanup-finding.md), [coder](history/2026-11-04_12-07-13-468393-coder-await-provisional-startup-child.md), [tester](history/2026-11-04_12-07-13-468394-tester-startup-diagnostics-final-checks.md).

## Startup input pass-through accepted; actual failure reporting next

Allstartup paths now omit absent configured env/header values and pass present
values unchanged.176independent checks and lockedbinarybuild0.49sPASS; Evaluator
ACCEPT. User's newGoogle screenshot shows actualconnectionended after launch,
with a generic Fetcherror. No claimofGooglelogin success; nextscopedpass captures
sanitized startupstderr/exitstatus for nativeconsole only.

Passes: [tester](history/2026-11-04_11-33-00-376432-tester-startup-pass-through-verification.md), [evaluator](history/2026-11-04_11-33-00-376433-evaluator-accept-startup-pass-through.md).

## Startup passes available inputs — November4

User explicitly removes missing-input checks for all startups. Shared resolver
passes present assigned environment/header values and omits absent ones, letting
the server's command/HTTP response decide requirements. Synthetic input-required
branch removed. Source ready; independent actual child/HTTP tests and build pending.
No input-value import, credential migration or raw-output feed selected.
[Milestone](startup-pass-through/info.md).

Passes: [planner](history/2026-11-04_11-23-56-027417-planner-remove-startup-input-preflight.md), [coder](history/2026-11-04_11-23-56-027418-coder-omit-missing-startup-bindings.md).

## Google setup workflow clarification — November4

Older instructions used credentials.json then a printed browser login link;
current published1.11.3 still accepts a file instead of separate env settings.
No bundled defaults/install hook creates Google app credentials. Hosted setup can
keep that work on the host. Which workflow this user originally used is unknown.
Fetch's all-imported-inputs preflight currently blocks native fallback and wrongly
requires optional profile. No new source/import policy selected by research.
[Primary findings](../docs/architecture/research.md#november4--fresh-google-setup-has-multiple-input-paths).

Passes: [researcher](history/2026-11-04_00-28-17-556807-researcher-google-fresh-install-workflows.md), [orchestrator](history/2026-11-04_00-28-17-556808-orchestrator-explain-google-setup-difference.md).

## Google configuration values — November 4 metadata check

Matching development Google package1.11.3 has the three configured input names.
Its recorded original definition has nonempty entries for all three; import
discarded values while preserving references. Presence only was inspected and
reported, not credential validity. No values printed, provider call, Keychain
access or config writes. [Pass](history/2026-11-04_00-23-52-401657-orchestrator-google-input-presence.md).
Next: user decides whether/how original setup values should be imported; no
change or migration inferred from this explanation.

## Authentication console — complete November 4

User's screenshot section now renders one dark200px scrolling monospace console
with per-attempt safe progress/error history. Unchanged polls are deduplicated;
title/repeated paragraphs/Hide are removed. Host consent remains inside; server
controls, Settings, APIs and login behavior stay unchanged.

Independent28/28UI checksPASS, final locked native buildPASS3.06s, whitespacePASS
and EvaluatorACCEPT. [Evidence](authentication-console/evidence.md).
No raw MCP output feed was selected or implemented; optional question unanswered.
No native visual/Figma/browser/provider/account acceptance or launch performed.
Next: explicitly Quit Fetch and run ./scripts/run-development.sh to see it.
Changes remain local with unrelated prior source/docs preserved.

Final passes: [evaluator](history/2026-11-04_00-19-42-164968-evaluator-accept-authentication-console.md), [orchestrator](history/2026-11-04_00-19-42-164969-orchestrator-complete-authentication-console.md).

## Authentication console source and UI checks

One shared200px dark scrolling monospace console now records existing safe Fetch
progress/errors. Title/repeated paragraphs/Hide removed; host consent remains
inside. Independent UI28/28checksPASS including three new history/dedup/privacy
cases. API/backend unchanged; no raw-process-output feed or native visual proof.
Root build/final review pending.

Passes: [coder](history/2026-11-04_00-18-11-293926-coder-authentication-console-source.md), [tester](history/2026-11-04_00-18-11-293927-tester-authentication-console-ui-checks.md).

## Authentication console — November 4

User requests replacing repeated authentication detail text with one console.
Immediate UI scope records existing safe progress/errors with a scrolling
monospace panel; host consent remains inside. No API/login prerequisite changes.
Optional raw-server-output choice remains pending; no such feed is claimed.
Independent Planner complete, exclusive UI Coder/Tester active.
[Milestone](authentication-console/info.md). [Plan](history/2026-11-04_00-14-58-757432-planner-authentication-console.md).
Next: implement, focused checks/build and independent review.

## Standard Authenticate — scoped implementation complete

The button uses /restart interactive:true. Local login runs an explicitly recorded
helper or the existing configured server command, waits for server-owned login,
and presents host-only browser consent. /authenticate stays functional/deprecated.
Terminal status is remembered separately from fresh current proof; no raw logs,
transient URLs or provider tokens are stored/exposed. Pending consent survives
window reopen, idle-work guard is atomic, Settings repairs refresh process values
and Quit waits for owned helper cleanup. Existing remote auth/post-install behavior
is preserved.

Independent171/171checks PASS; final guidance recheck32/32 PASS; final locked
binary build PASS0.30s; Evaluator ACCEPT. All material findings resolved.
[Evidence and limits](standard-authenticate/evidence.md). [Milestone](standard-authenticate/info.md).
No real Google-account/browser/Keychain/native appearance/client acceptance or
launch/signing/release performed. Native URL elicitation remains unadvertised.
Next: explicitly Quit Fetch and use ./scripts/run-development.sh to try the new
flow; protected real-account/tool/restart-reuse verification remains separate.
Unrelated unfinished work below is preserved; changes are local/uncommitted.

Final passes: [tester](history/2026-11-03_23-52-16-646996-tester-standard-authenticate-final-checks.md), [evaluator](history/2026-11-03_23-52-16-646997-evaluator-accept-standard-authenticate.md), [orchestrator](history/2026-11-03_23-52-16-646998-orchestrator-complete-standard-authenticate.md).

## First authentication checks and fixes

Locked native binary build PASS15.41s. First independent lib run95/96 PASS
(stale error-code assertion corrected); first confined scenarios5/7 PASS
(real completion/admission race fixed; fixture canary must exist before probing).
Registered stopped-route inputs are also refreshed after idle fencing. Helper
and descendant cleanup passed. Final eight-scenario/lib/regression retest is
underway; no final acceptance or real-account proof yet.

Passes: [coder](history/2026-11-03_23-47-25-206126-coder-atomic-login-admission.md), [tester](history/2026-11-03_23-47-25-206127-tester-initial-authentication-findings.md), [coder](history/2026-11-03_23-47-25-206128-coder-authentication-review-and-test-fixes.md).

## Manager login source integrated — verification pending

Local startup/helper flow, host-only consent, ten-minute bound, terminal saved
status/current distinction, named Settings inputs and awaited helper tasks are
implemented in source. /restart interactive:true is canonical; old auth remains
compatible. Root fixed reopening pending consent. Runner atomic admission follow-up
is the last compile dependency before combined checks. [Manager pass](history/2026-11-03_23-44-37-831222-coder-local-authentication-manager.md).

## Authentication review corrections — in progress

Independent Evaluator requests changes for reopened-window host consent, actual
active-work guard, missing-input status consistency and helper Quit ownership.
Root fixed consent restoration with a source fixture; Coders/Tester address
backend fixes/evidence. No build/test pass yet. [Review](history/2026-11-03_23-42-17-557862-evaluator-request-auth-flow-corrections.md).

## Interactive transport source ready — November 3

Runner supports configured local startup with attempt-scoped login URL observer
and600second initialize/discover wait; normal/remote stay30seconds. Raw logs stay
private and bounded reader emits only recognized HTTPS browser links. Five parser
tests written, unrun; source whitespace check passes. Manager/UI/API integration
and independent verification remain active. [Coder pass](history/2026-11-03_23-11-48-941919-coder-local-login-transport.md).

## Start route owns login — November 3 steering

User selects existing /restart start/connect for explicit Authenticate via
interactive:true. Old /authenticate is deprecated but retained; no /start duplicate
or removal timeline. Implementation/independent tests in progress.
[Steering pass](history/2026-11-03_23-09-17-803619-orchestrator-select-start-route-auth-compatibility.md).

## Standard Authenticate — implementation started November 3

User selected ordinary server startup with existing provider login and visible
sign-in instructions. Independent Planner identified missing startup fallback,
discarded URL output and two30second readiness deadlines. Exclusive Runner and
Manager Coders plus independent confined Tester are active; root owns UI/docs.
Existing POST/GET remain; safe sign_in host/opened state and saved last-known
status are additive, never provider tokens or current success from startup alone.
Native URL elicitation is not advertised without its missing reply contract.
[Milestone](standard-authenticate/info.md). [Plan](history/2026-11-03_22-16-25-312598-planner-standard-authenticate.md).
Next: source integration, meaningful isolated checks, independent review/fixes.

## Saved authentication field clarification — November 3

Installer still writes authentication.status unknown in config.json. Live login
outcomes are in Manager memory and overlaid by GET; they are not written back as
updated auth verdicts. Storage projection resets saved auth to unknown. Inspection
of pre-refactor80782d9 also finds live auth status separate from record saves; do
not claim the latest refactor newly removed durable login-outcome persistence.
Old per-credential stored/missing fields are a distinct historical record.
[Pass](history/2026-11-03_22-08-24-422012-orchestrator-clarify-saved-auth-status.md). No source/runtime changes.

## Post-install startup/status confirmation — November 3

Read current Installer prepare_group and Manager install/connect_installed,
connection_failed/connection_succeeded and servers projection. Product installation
still awaits normal Connect after preparation; login/start failure preserves the
installation. Runtime and authentication status are separate. Local readiness
sets auth unknown; recognized auth errors set required. Current status lives in
memory, not a durable authentication verdict; saved definitions/grants are separate.
No runtime operation or source change. [Pass](history/2026-11-03_22-06-55-657313-orchestrator-confirm-post-install-start-status.md).

## MCP login libraries and client research — November 3

User requested online general solutions and coding-client comparisons. No reviewed
library discovers arbitrary native login commands from installed source. Official
remote OAuth and server-supplied URL elicitation are reusable, with upstream support.
Google1.11.3 can start its own login during ordinary startup; its auth command is
for forced sign-in. Source logs a browser link; Fetch discards it on both ordinary
startup and recorded setup. This corrects the earlier overbroad claim that initial
Google login necessarily needs a separate recorded helper. Missing native_setup
still explains the explicit Authenticate action's source limitation. Saved instance
configuration and actual runtime failure cause were not inspected.

[Research and recommendation](../docs/architecture/mcp-login-discovery-research.md).
Two read-only Researcher passes and independent core-source Evaluator ACCEPT;
no application changes, runtime tests, installs or account operations. Next: user
reviews how setup interaction is presented and when a separate command is needed.
Existing product/UI tasks remain open; no new API/source behavior selected.

Passes: [researcher](history/2026-11-03_22-01-56-117384-researcher-native-login-libraries.md), [researcher](history/2026-11-03_22-01-56-117385-researcher-harness-login-and-google-startup.md), [evaluator](history/2026-11-03_22-01-56-117386-evaluator-accept-core-login-research.md), [orchestrator](history/2026-11-03_22-01-56-117387-orchestrator-integrate-login-research-correction.md).

## Folder-first Install UI — November 3, 2026

Active UI milestone: rapid-ux. Install opens empty and discovers only after an
explicit file/folder selection or typed-path Enter/Refresh. The title now includes
the requested small gray hint. Removed the long discovery explanation, app-wide
tagline and accepted-install notice. Install is gray and disabled while submitting
or monitoring; progress/errors and existing retry rules remain. Backend routes,
source reading, credentials and installation rules are unchanged.

Direct UI checks22/22 PASS; locked native binary build PASS11.56s; diff whitespace
check PASS. The same agent implemented/tested/reviewed this pass at the user's
request; no independent review or native visual/Figma acceptance is claimed.
No app launch or signed launcher run. Optional .claude/.opencode folder recognition
awaits the user's answer; existing special folder recognition is .codex only.
Next: user's folder decision and visual check after explicit Quit/signed relaunch.
[UI pass](history/2026-11-03_21-51-06-855728-orchestrator-folder-first-install-ui.md). Unrelated product work below remains open.

## Google authentication report — November 3

User screenshots show folder-unavailable and generic key controls instead of
Google login. Read-only source tracing confirms Authenticate requires explicit
native_setup metadata for local servers; normal imports do not supply it. The
UI offers folder/key controls without establishing that either can start the
server login. The prior Google helper adapter was removed by3008546. This is
an unfinished native-login integration, not evidence of Google rejecting login.
No live server record, credentials, provider process or account was inspected.
No source changes or runtime checks performed; concurrent UI edits preserved.
Next: review how supported native setup command/folder records reach imported
instances before implementation, as required by the current contract.
[Diagnosis](history/2026-11-03_21-49-55-213550-orchestrator-google-auth-gap-diagnosis.md).

## Repository sync — November 3, 2026

Pulled origin/main with `git pull --ff-only origin main`: clean main advanced
from80782d9 to135873a (3 commits), without conflicts. No application launch,
build, tests or MCP connections ran in this pass. Existing product status and
unfinished work below remain unchanged. Only the required local journal was
updated after pulling; no commit or push requested.
[Sync pass](history/2026-11-03_21-42-33-474236-orchestrator-pull-main.md).

## November3 — scoped wrapper refactor accepted

User resumed; native provider credentials belong to unmodified server/helpers.
Official rmcp owns online login/renewal; actual authenticate route handles user and
restricted background requests. Manual-key PATCH is retained with direct Keychain
saving; GET exposes safe nullable auth times; config edits execution only; removal
preserves old/native/package/grant state. All material review findings fixed.
Final151 checks +separate original Everything confinement check1PASS and locked
native binary build PASS. Independent Evaluator ACCEPT.
[Pass](history/2026-11-03_21-10-00-310480-tester-final-native-storage-checks.md) [Pass](history/2026-11-03_21-10-00-310481-evaluator-native-storage-accepted.md)
[Current contracts](../docs/architecture/native-server-storage.md).
[Complete evidence and remaining limits](native-server-storage/evidence.md).
[Integration pass](history/2026-11-03_21-10-43-020636-orchestrator-integrate-native-storage.md).
[Concurrent docs merge](history/2026-11-03_21-12-03-941043-orchestrator-merge-concurrent-development-docs.md).
[Merge review ACCEPT](history/2026-11-03_21-12-44-288190-evaluator-accept-concurrent-docs-merge.md).
Ordinary native command/path authoring and real OS/provider/client acceptance
remain open product work; this is not full-MVP completion. Integrated as3008546. Next: separately review native metadata entry before new
public fields or per-server handling. Existing personal configs/credentials/state remain untouched.

## November 3 — normal-practice condition recorded

User accepts auth timing/status insofar as normal practice. Saved expiry, before-use
checks and shared library renewal are established; early timer is optional and GET
fields are Fetch reporting. No hidden native expiry guesses, new polling system or
source resume inferred. [Plan](../docs/architecture/native-server-storage.md).
[Pass](history/2026-11-03_20-37-18-125113-orchestrator-standard-auth-behavior-condition.md).


## November 3 — auth timing and server GET metadata proposed

User requests how/when checks run and login status/expiry in GET servers. Proposed
known-expiry scheduling plus connection/use/rejection checks through authenticate;
GET only reports safe local state. Nullable access_expires_at/last_verified_at,
existing auth states and native unknowns are specified for review. Official library
source timing checked; no application/tests/runtime changes.
[Proposal](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-34-10-731157-orchestrator-auth-timing-and-server-get-proposal.md).


## November 3 — recorded setup commands and manual-key PATCH selected

Manual fixed keys use retained PATCH/shared Keychain delivery. Authenticate reads
explicit recorded setup commands and launches existing native helpers; user rejects
README interpretation. Exact command/path recording and save/check/remove outcomes
remain open. Background caller permission explained as proposal; no approval inferred.
Official library and same authenticate route stay selected. No code/tests resumed.
[Plan](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-30-49-061815-orchestrator-recorded-native-setup-and-manual-keys.md).


## November 3 — official auth library selected; source remains paused

User selects official Rust MCP auth library and asks exact remaining Fetch work
before coding. Plan now identifies server/library/Fetch responsibilities, necessary
connection work vs chosen product features, replacement modules and open contracts.
Raw runner, native auth ownership, Keychain and actual authenticate-route callers
remain. No Cargo/source/test/runtime changes; prior partial code stays incomplete.
[Current accepted plan](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-24-22-871619-orchestrator-official-library-selected-plan.md).
Independent read-only planner confirms the ownership list and identifies partial-source
mismatches (direct refresh function calls and removed credentials route).
[Planner pass](history/2026-11-03_20-24-56-219543-planner-official-library-fetch-boundaries.md).
Next: user reviews remaining work and precise background/manual-key/native-path
details before source resumes.


## November 3 — minimal-wrapper lifecycle research

User wants server-owned behavior and existing libraries used before adding Fetch
code. Read-only Researcher recommends unchanged local servers plus official Rust
OAuth-only library/shared Keychain store for online connections, raw runner retained.
Library adoption remains proposed; same authenticate route/Keychain preserved.
[Lifecycle and ownership](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-20-08-657443-researcher-minimal-wrapper-lifecycle.md).
No source/test/runtime work resumed; partial source remains incomplete.


## November 3 — button and background call authenticate

User explicitly selects existing authenticate POST for both callers. The route
owns shared saved-login/renewal/replacement/save/status behavior; scheduler only
chooses timing. No new refresh route or per-server refresh engine. Still plan
background browser restrictions and authorized self-request details before coding.
Source remains paused/incomplete/unbuilt/untested.
[Plan](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-09-40-949231-orchestrator-shared-authenticate-route-decision.md).


## November 3 — global explanation preference saved

User requires basic-fact, plain-language explanations that lead to important
decisions without jargon clarification loops. Global AGENTS updated and readback
confirms prior content preserved. No architecture choices changed; Fetch source
remains paused/incomplete/unbuilt/untested. [Pass](history/2026-11-03_20-07-15-936402-orchestrator-global-basic-explanation-preference.md).


## November 3 — retain credentials route; scope under discussion

User revises credentials-route removal to Keychain storage unless native servers
already handle it. Plan now retains PATCH /credentials. Recommendation: native
server OAuth stays server-owned; manual keys could use Keychain/shared delivery.
Exact scope/save/verification/Settings behavior awaits decision; coding remains
paused and partial source still incomplete/unbuilt/untested.
[Current plan](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-06-01-786436-orchestrator-retain-credentials-route-plan.md).


## November 3 — source paused for complete native-storage plan

User selected Keychain, then asked to plan all route/other changes and discuss any
server-specific code before writing. Partial Manager/API/UI/remote-Keychain/fixture
edits are preserved. No builds/tests/runtime/Keychain/Finder operations occurred.
Current source is incomplete (missing native-state/verification functions and old
helper/deletion references remain). No working-refactor claim.
[Complete plan](../docs/architecture/native-server-storage.md). [Pass](history/2026-11-03_20-03-53-183471-orchestrator-native-storage-plan-pause.md).
Next: review route list and native helper/folder/environment/Railway details with
the user before further source work.


## November 3 — native server storage: pre-code review

Clean main pulled8f2a118 →515c7b1. User selects native server config/login/storage,
superseding Fetch provider encryption requirements while preserving legacy data.
Read-only Researcher/Planner audits identify shared app protection and remote-client
OAuth ownership, environment-only setup and real-folder/deletion limits.
[Inventory/proposal](../docs/architecture/native-server-storage.md) awaits decisions;
application source unchanged, no tests/runtime actions. Independent proposal review
ACCEPT; explicit remote Keychain readback and authenticated transport check added.
[Review](history/2026-11-03_19-54-32-541617-evaluator-native-storage-proposal.md).
[Milestone](native-server-storage/info.md). [Pass](history/2026-11-03_19-53-41-841065-orchestrator-native-storage-inventory.md).


## November3 — first-launch checklist for each Mac

Project AGENTS.md now starts with required tool/signer checks, correct launcher
identity precedence, human certificate setup and exact user guidance when missing.
It distinguishes reusable source from local keys/fingerprint/logins and requires
explicit launch scope. Docs-only checks pass; no source/runtime/Keychain changes.
Independent review ACCEPT. [Review](history/2026-11-03_20-01-44-702738-evaluator-accept-first-launch-agent-checklist.md). [Pass](history/2026-11-03_20-01-19-069965-orchestrator-document-per-mac-first-launch-checks.md).

## November 3 — full working-tree delivery

User explicitly requests publishing all remaining tracked/untracked changes.
This snapshot includes UI selection/parallel startup, Keychain cache/development
launcher, architecture notes, public verification evidence and preserved history.
Independent review ACCEPTs snapshot publication; root scan finds no apparent
credentials/private keys/conflict markers or large artifacts. No new runtime
acceptance is claimed. Feature-specific verification and remaining work below
retain their original scope. [Publication pass](history/2026-11-03_19-49-42-068944-orchestrator-full-working-tree-delivery.md).


## November3 — development Keychain prompt fix complete

Latest remote main integrated with prior local work; duplicate helper source
skipped and restored UI toggle preserved with new folder discovery. Successful
individual keys now stay in Zeroizing memory until Quit; new-key OS readback,
failure exclusion, delete eviction and permanently closed shutdown preserve
existing encryption/verification. Stable DEVELOPMENT launcher signs/verifies a
copy with fixed identifier and exact public certificate fingerprint.

The user created the dedicated signer in Certificate Assistant. Public fingerprint
settings saved; private key never exported and trust/old item permissions unchanged.
Actual signed Fetch build succeeds; two changed binaries share the same verified
identity requirement. Tests:30credentials+1host+18UI,4launcher fixtures; native
build18.42s. Independent source/final evaluations ACCEPT. Temp fixtures/probes
removed; persistent signer/settings are intentional dev setup. Existing Fetch and
services preserved. Quit Fetch then run ./scripts/run-development.sh. Old items
may need final approval; native prompt counts/rebuild lock/denial remain unmeasured.
No wrapping-key migration, release signing or packaging selected.
Source/launcher delivered in515c7b1; final verified recordsd73865d pushed to
main. Working tree and whitespace checks pass; no outstanding selected work.
[Delivery](history/2026-11-03_19-53-42-343030-orchestrator-keychain-workflow-delivery.md). [Completion](history/2026-11-03_19-52-53-335817-orchestrator-complete-keychain-development-setup.md).
[Evidence](keychain-prompts/evidence.md).
[Pass](history/2026-11-03_19-51-19-045118-evaluator-cache-and-launcher-source-acceptance.md).
[Pass](history/2026-11-03_19-51-19-045707-tester-key-cache-launcher-and-pull-regressions.md).
[Pass](history/2026-11-03_19-51-19-046417-orchestrator-configure-and-verify-development-signer.md).
[Pass](history/2026-11-03_19-51-19-047925-evaluator-development-identity-final-evidence-acceptance.md).

## November3 — development Keychain prompt investigation

Read-only investigation of the user's five identical dialogs: no key cache, same
label on separate keys, ad-hoc dev binary identity. Metadata-only current record
count is one app/one server; exact five requests untraced. Dedicated research
supports stable dev signing first, optional process caching; one wrapping key
would be a new migration/recovery choice. No implementation or personal
signing/Keychain/trust changes. [Findings](../docs/architecture/keychain-prompts.md).
[Research pass](history/2026-11-03_19-24-21-984231-researcher-development-keychain-prompts.md). Independent review ACCEPTs source/research claims with exact prompt count untraced;
no implementation selected. [Review](history/2026-11-03_19-25-00-343374-evaluator-development-keychain-prompt-findings.md).

## November 3 — deletion speed fix complete

User-approved subtree flush batching is implemented in storage/mod.rs. Recursive
path/depth/ownership/nofollow checks and credential/config sync remain; wrapper
flushes once after removal, including already-missing retries. Independent safety
fixtures6/6 PASS, source/final evaluation ACCEPT, native build PASS13.52s.

Real confined repeats use identical versions/entry counts: Google1.11.3 deletion
75.225s to0.889s (74.336s saved); Everything2026.8.31 deletion20.400s to0.262s,
with real connection/13tools/echo. Preservation and confinement probes pass.
All temporary environments/packages/caches/processes cleaned; personal state and
everyday Fetch/Node/Railway preserved. Quit/reopen Fetch to use the new binary.
Native UI/HTTP/realKeychain/Google-account acceptance remains separate. Concurrent
helper-bridge work below is preserved; no packaging/release action occurred.
Delivered as ed78aac and pushed to main; unrelated working-tree changes remain
preserved. [Delivery](history/2026-11-03_19-15-58-568116-orchestrator-deliver-deletion-speed-fix.md).
[Optimized evidence](delete-server/optimized-evidence.md).
[Source review](history/2026-11-03_19-14-39-483237-evaluator-batch-delete-source-acceptance.md), [tests/cleanup](history/2026-11-03_19-14-39-483624-tester-batch-delete-optimized-runs-and-cleanup.md),
[native build](history/2026-11-03_19-14-39-483753-orchestrator-batch-delete-native-build.md), [final review](history/2026-11-03_19-14-39-483850-evaluator-batch-delete-final-acceptance.md).

## November 3 — shared helper credential storage accepted

Integrated with newer automatic config discovery. Final188/188 fixture checks
and native build (0.54s) PASS; independent Tester/Evaluator ACCEPT. Source pushed
asd19d891 and live main matched. Owned test processes/storage cleaned.
[Orchestrator report](history/2026-11-03_19-35-51-970660-orchestrator-integrate-helper-storage-with-current-main.md)
[Tester report](history/2026-11-03_19-35-51-971503-tester-verify-integrated-helper-storage.md)
[Evaluator report](history/2026-11-03_19-35-51-971680-evaluator-accept-integrated-helper-storage.md)
[Orchestrator report](history/2026-11-03_19-35-51-972125-orchestrator-deliver-shared-helper-storage.md)

User replied go to the shared bridge with Google first. Independent research
reverified installed1.11.3 and its nested OAuth2 implementation; Planner completed
the bounded lifecycle/storage/verification plan. Shared credential/runner core
is implemented and inspected, including revocation/timeout corrections. Google/
Manager source is integrated. Independent review found and corrected stored-proof,
retry, config-invalidation and late process-failure issues.176 unique fixture
checks pass, including OS-confined Node/actual managed-child lifecycle and raw
MCP checks; locked native build passes (0.48s). Independent Tester/Evaluator
ACCEPT. Initial sandbox/fixture failures were corrected in tests only. No real
Google login/installed-helper account/native acceptance is claimed. Delivered asd19d891. Preserve callback ownership, per-server
encrypted readback/CAS/renewal and service-access proof. No new public route/UI
or personal credential authority. Existing unrelated edits are preserved; real
provider account authentication remains unverified.
[Researcher pass](history/2026-11-03_19-02-57-627115-researcher-helper-bridge-source-research.md).
[Planner pass](history/2026-11-03_19-02-57-627390-planner-shared-helper-bridge-plan.md).
[Core implementation pass](history/2026-11-03_19-08-28-176447-coder-shared-helper-storage-core.md).
[Coder pass](history/2026-11-03_19-21-11-988354-coder-google-helper-adapter.md).
[Orchestrator pass](history/2026-11-03_19-21-11-988889-orchestrator-helper-bridge-review-corrections.md).
[Helper storage evidence](shared-mcp-authentication/helper-storage-evidence.md).
[Tester pass](history/2026-11-03_19-28-52-403779-tester-helper-storage-fixture-acceptance.md).
[Orchestrator pass](history/2026-11-03_19-28-52-405135-orchestrator-helper-storage-native-build.md).
[Evaluator pass](history/2026-11-03_19-28-52-405501-evaluator-accept-shared-helper-storage.md).
[Proposal pass](history/2026-11-03_18-18-43-063245-orchestrator-propose-shared-helper-credential-storage.md).

## November 3 — automatic config discovery

Delivery found newer main commits; both sets of changes are merged with conflicts resolved. Integrated independent checks pass: 64 relevant cases, including 12 new, plus final native build. [Merged test evidence](history/2026-11-03_19-30-58-261856-tester-integrated-folder-discovery.md). Independent Evaluator ACCEPTs the merged result. [Final review](history/2026-11-03_19-31-19-393149-evaluator-integrated-discovery-acceptance.md). Delivered as369267a; local/live main refs matched and tree was clean. [Delivery record](history/2026-11-03_19-31-43-298030-orchestrator-deliver-folder-discovery.md). [Integration report](history/2026-11-03_19-29-25-940238-coder-integrate-newer-main.md).

Active focused milestone: [automatic config discovery](automatic-config-discovery/info.md).
Opening Install will read known global config files automatically. Explicit home/.codex folder or single-file selection remains optional; installation still requires selecting servers and pressing Install. Independent fixture checks pass: 47 relevant cases, including 10 new; native binary builds. [Test evidence](history/2026-11-03_19-26-54-226235-tester-automatic-discovery-fixtures.md). Independent Evaluator ACCEPTs the scoped source/fixture change. [Evaluation and integration](history/2026-11-03_19-27-27-851163-evaluator-automatic-discovery-acceptance.md). Actual native picker/window interaction and refreshed Figma specimens remain pending. [Implementation report](history/2026-11-03_19-25-58-185229-coder-folder-discovery-and-ui.md). The latest provider scope decisions and protected runtime work below remain unchanged.

[Planner report](history/2026-11-03_19-25-18-599038-planner-automatic-config-discovery.md).


## November 3 — Google deletion latency reproduced

Requested investigation and disposable backend demos are complete. Google MCP
1.11.3 preparation/deletion PASSes:13,871files/1,798folders,75.225s total deletion;
package cleanup through75.210s, final15ms. Fetch did not Connect or authenticate
Google. Everything install/echo/delete also PASSes (20.400s delete; its process
stopped within~139ms). A separate1000-file Rust comparison measured4559ms with a
flush after every file versus42ms with one final flush. Source performs one full
disk flush per removed file/folder, while holding the shared installer lock.

Independent Tester and Evaluator ACCEPT actual results, confinement canaries and
preservation. Both temporary environments, packages/caches/copied runtimes,
failed-attempt state and external canary/pointer were removed; no owned process
remains. Everyday Fetch/Node/Railway processes and personal settings/accounts
were preserved. Only ignored diagnostic tests and evidence/documentation were
added in that diagnostic pass. The subsequently approved optimization is complete
above. Native UI/HTTP/real
Keychain/Google-account acceptance remains open. Recommended next step, if user
requested that fix: implemented and verified above. Broader MVP work remains
tracked below.

[Final integration pass](history/2026-11-03_18-24-07-034474-orchestrator-complete-google-delete-investigation.md).
[Evidence](delete-server/latency-evidence.md), [Google log](delete-server/latency-google.log),
[probes](delete-server/latency-confinement.json).
[Google test/cleanup pass](history/2026-11-03_18-23-43-642191-tester-google-deletion-and-temporary-cleanup.md).
[Google final evaluation](history/2026-11-03_18-23-43-643255-evaluator-google-delete-latency-acceptance.md).
[Google source evaluation](history/2026-11-03_18-20-20-124512-evaluator-google-delete-diagnostic-source.md).
[Everything test](history/2026-11-03_18-15-46-053228-tester-real-everything-delete-latency.md),
[Everything evaluation](history/2026-11-03_18-16-32-060854-evaluator-everything-latency-evidence.md).
[Research](history/2026-11-03_18-05-50-741147-researcher-delete-latency-trace.md).

## November 3 — direct installation body accepted

User approved command/url alternatives in the existing POST body after the full
proposal and explicitly excluded UI. Independent Researcher selected already
locked shlex2.0.1 plus restricted syntax checks; Planner completed the bounded
plan. Coder source is integrated: shared capture/preparation, parsed command/URL
inputs, saved direct-identity digest and matching MCP schema. Root inspected the
diff; independent Tester/Evaluator ACCEPT.142 unique checks pass and the locked
native binary builds (0.35s), with no new warnings. No real package install/
provider/connection acceptance is claimed; hooks use disposable simulations.
Source/evidence delivered as53adad6; live remote/local refs matched.
[Delivery pass](history/2026-11-03_18-15-17-581559-orchestrator-deliver-direct-install-body.md). Preserve
concurrent UI/startup edits, source imports and Google deferral.
[Contract proposal pass](history/2026-11-03_17-56-44-860492-orchestrator-propose-command-install-input.md).
[Researcher pass](history/2026-11-03_18-04-47-964906-researcher-command-parser-selection.md).
[Planner pass](history/2026-11-03_18-04-47-965249-planner-direct-install-body-plan.md).
[Implementation pass](history/2026-11-03_18-08-05-870550-coder-direct-install-body.md).
[Verification evidence](direct-install-inputs/evidence.md).
[Tester pass](history/2026-11-03_18-10-43-880763-tester-direct-install-fixtures.md).
[Orchestrator pass](history/2026-11-03_18-10-43-880970-orchestrator-direct-install-native-build.md).
[Evaluator pass](history/2026-11-03_18-10-43-881085-evaluator-accept-direct-install-body.md).
[Scope](direct-install-inputs/info.md). [Pass](history/2026-11-03_17-40-38-021507-orchestrator-scope-direct-install-inputs.md).

## November 3 — rapid UX changes

Active focused milestone: [rapid-ux](rapid-ux/info.md). Select all now becomes
Unselect all when every selectable discovery row is selected and clears the
selection on click. Startup attempts every saved MCP connection independently,
so one slow connection no longer delays the remaining servers. Existing
credentials, readiness deadlines and per-server status/error handling remain.
Locked native compilation passes;53 manager and14 existing UI fixtures pass.
Source review was direct, without agents; these checks were not independent and
do not establish actual window or real saved-server startup acceptance. No app,
real provider/MCP, personal state or packaging/release action ran. Next: user
interaction/startup verification and any next requested UX change.
[Pass evidence](history/2026-11-03_17-13-40-579746-orchestrator-selection-toggle-and-parallel-startup.md).

## November 3 — install route walkthrough

Read-only trace confirms POST setup/installations accepts selected source entries
and returns202 progress tracking. One active group prepares entries sequentially,
preserves successful preparation and attempts normal Connect for each prepared
server. Runtime/authentication outcomes remain separate from preparation success;
no Fetch Authenticate flow is invoked by installation. No application change or
new execution evidence. [Walkthrough pass](history/2026-11-03_17-12-59-367921-orchestrator-explain-install-route.md).

## November 3 — instance identity walkthrough

Source inspection confirms each new instance receives a persisted `server-` ID
with32 random hexadecimal digits. Names are separate. Different imported entries
can create multiple instances of one package; the same source path/name reuses
the existing ID. The user accepts this behavior and does not need an explicit
duplicate-instance control for now; it is outside current scope. Google account
integration remains pending. No source change or new execution evidence.
[Scope confirmation](history/2026-11-03_17-09-36-297254-orchestrator-accept-existing-instance-imports.md).
[Walkthrough evidence](history/2026-11-03_17-06-53-160519-orchestrator-explain-instance-identifiers.md).

## November 3 — shared authentication router accepted

The user clarified selection uses imported command, arguments and environment
requirements, not README interpretation. Remote defaults to web OAuth; shared
handlers cover API credentials and supported CLI/local helpers. A selected-flow
failure is returned to the user without a fallback login method. Existing public
contracts, account isolation, encrypted readback/proof and responsiveness remain.
Independent Planner completed the bounded common OAuth/dispatcher plan. Coder
implemented private configuration dispatch and common registration/URL/token
requests; Orchestrator inspected the diff. Independent Tester/Evaluator ACCEPT,
99 unique fixture checks pass and locked native binary compilation succeeds
(0.74s). Initial stale alias fixture failure was corrected in test setup only;
no production alias behavior changed. No live provider/helper/native/client
acceptance is claimed. Installed
Google1.11.3 has no storage hook; the user explicitly keeps its encrypted bridge
and integration pending while the common router is completed. Multiple instances
of the same MCP must retain separate methods/credentials/accounts. The existing
same-source installation reuse remains unchanged. User requests a behavior
walkthrough after routing completion. Source/evidence delivered as4f18905;
live remote and local commit matched. Pre-existing documentation edits remain
uncommitted. [Delivery pass](history/2026-11-03_16-55-36-339850-orchestrator-deliver-shared-router.md).
[Verification evidence](shared-mcp-authentication/evidence.md).
[Tester pass](history/2026-11-03_16-54-04-913350-tester-shared-router-fixtures.md).
[Orchestrator pass](history/2026-11-03_16-54-04-913622-orchestrator-shared-router-native-build.md).
[Evaluator pass](history/2026-11-03_16-54-04-913739-evaluator-accept-shared-router.md).
[Implementation pass](history/2026-11-03_16-49-58-499042-coder-shared-authentication-router.md).
[Research and user scope decisions](history/2026-11-03_16-48-24-958601-researcher-google-helper-and-router-scope.md).
[Planner pass](history/2026-11-03_16-46-28-931238-planner-shared-authentication-dispatch.md).


## November 3 — authentication scope sync before implementation

Delete API and server-header × are delivered. The user requests another scope
sync before authentication implementation starts; no authentication source has
been changed in this pass. Proposed direction remains the existing generic
Authenticate API wrapping standard MCP web authentication, reusing portable
Codex code and OpenCode's start/URL/finish structure. Nonstandard local helpers
need only their actual compatibility differences, not a Google-specific route.
Optional small edge-case handling now requires explicit user approval, saved in
the global Codex AGENTS.md. Research is complete at source-inspection level;
no live provider login is verified. Await the requested scope sync.
[Pass record](history/2026-11-03_16-26-52-000001-orchestrator-auth-scope-sync-and-global-preference.md).

## November 3 — server-header × ready for user testing

Delete API/tool delivered as64cff43. The user-requested shared × now sits beside
every server disclosure and invokes that DELETE operation. Confirmed removal
clears relevant cached state; busy/pending and uncertain-response handling avoid
duplicate deletion. Independent source ACCEPT; native binary build passes and
14 existing UI-handler regression checks pass. No actual app/window/deletion or
personal-state operation ran. [Evidence](history/2026-11-03_16-20-25-000001-orchestrator-delete-button-build-and-ui-checks.md).

Shared authentication remains the next implementation: user emphasized a common
standard harness/MCP web-auth rewrite for all servers, with no Google-specific
route or login engine. Completed upstream/helper research is not live auth proof.

## November 3 — delete server, then shared authentication

Active milestone: [delete-server](delete-server/info.md). User delegates clean
implementation using existing conventions. DELETE /api/v1/servers/{server_id}
will use shared per-server management authority, reject busy work, stop the idle
connection and remove only Fetch-owned state/private packages. HTTP/MCP share
the backend; source files, scripts, runtimes and unrelated accounts remain.
Plan accepted; HTTP/MCP adapters are implemented and inspected; backend cleanup/
lifecycle is implemented; cancellation-owned gates and shutdown handling corrected.
Tester and Evaluator final source ACCEPT, no material findings. Scoped Git
delivery is next. No build/runtime or actual deletion ran.
[Plan evidence](history/2026-11-03_15-11-28-000001-planner-delete-server-contract-and-authentication-queue.md).
[Backend source](history/2026-11-03_15-27-24-000001-coder-safe-server-deletion-backend.md).
[Initial source review](history/2026-11-03_15-27-24-000002-tester-delete-cancellation-finding.md).
[Adapter source evidence](history/2026-11-03_15-16-40-000001-coder-delete-http-and-mcp-adapters.md).

User explicitly queued the shared asynchronous MCP authentication implementation
after deletion delivery. Preserve that full request: pinned upstream reuse,
standard HTTP OAuth including existing application registrations, URL-mode
elicitation/server-owned callbacks, local login helpers with Google first,
independent encrypted account storage/renewal, shared async ownership/cancel/
expiry/UI responsiveness, Railway/Linear compatibility, actual verification and
attribution. Read current AGENTS/November3 notes/authentication-engine first.
No authentication implementation starts before deletion is delivered.

- [Guard/shutdown correction and Tester acceptance](history/2026-11-03_15-32-49-000001-coder-delete-owned-worker-and-tester-acceptance.md)
- [Final deletion source acceptance](history/2026-11-03_15-32-49-000002-evaluator-delete-source-acceptance.md)

## November 3 — installation should leave usable servers running

Active focused milestone: [install-connect](install-connect/info.md). User selects
installation's final stage invoking existing Restart / Connect, retaining usable
connections and reporting authentication.status not_required for credential-free
success without a known login requirement. Keep GET servers status; unknown alone
must not prompt login. Missing/rejected access requires setup; unrelated start
errors remain runtime errors. No automatic login or tool calls. Source implemented;
independent Tester's identity/generation finding is corrected and re-reviewed
ACCEPT. Final Evaluator's late-response gap is also fixed: provider rejection
after timeout/cancel updates status without changing the call's original outcome.
Tester and Evaluator final source ACCEPT; no material findings remain. Source is
committed/pushed as fcf6c34. No builds, tests
or runtime/provider/MCP actions ran for this change.

- [Implementation](history/2026-11-03_14-55-20-000001-coder-install-connect-source.md)
- [Independent source finding](history/2026-11-03_14-55-20-000002-tester-install-connect-generation-finding.md)

The installed-discovery fix was rebased onto newer upstream source, independently
re-reviewed ACCEPT and pushed as9eddd00. Its build/native/Figma evidence remains
pending. [Delivery and new plan](history/2026-11-03_14-45-40-000001-orchestrator-discovery-delivery-and-install-connect-plan.md).

## November 3 — source layout explained and requirement recorded

Current pass: read-only source inspection plus documentation. The user clarified
that `src/api` should hold HTTP route handlers grouped by URL path, while other
folders may contain reusable backend functions. Current implementation differs:
route handlers are combined in `api/runtime.rs` and some HTTP endpoints live in
`oauth`/`mcp`. No application source was reorganized; no builds/tests/runtime
operations ran. Next layout action is a bounded source refactor preserving
existing routes and shared backend behavior, when requested. Existing MVP and
installed-discovery work remains unfinished as recorded below.
[Pass evidence](history/2026-11-03_14-45-09-644674-orchestrator-source-layout-clarification.md).

## November 3 — newer GitHub work discovered during delivery

- [Late-response correction and Tester acceptance](history/2026-11-03_15-01-26-000001-coder-late-auth-errors-and-tester-recheck.md)
- [Final D83 source acceptance](history/2026-11-03_15-01-26-000002-evaluator-install-connect-source-acceptance.md)
- [Actual source delivery](history/2026-11-03_15-02-49-000001-orchestrator-deliver-install-connect-source.md)

- [Identity correction](history/2026-11-03_14-57-40-000001-coder-bind-auth-rejections-to-connection.md)
- [Corrected source acceptance](history/2026-11-03_14-57-40-000002-tester-install-connect-source-acceptance.md)
- [Final review late-response finding](history/2026-11-03_14-59-42-000001-evaluator-late-authentication-rejection-finding.md)

Pushing the installed-discovery fix was rejected because GitHub had twelve newer
commits through a7a6e0d. Fetching revealed D81/D82 work absent from the initial
local checkout: shared UI/theme, documented remote/bridge import support,
issuer-bound generic OAuth, checked credential groups, shared verification data,
parallel authentication and further encrypted-document/token handling. Incoming
records include substantially newer fixture evidence (including110 combined
checks for D82); those records supersede the pre-fetch local readiness summary.
Real native/provider/client acceptance is still not established by those fixtures.

The feature is being rebased onto that work, preserving its auth request tracking
and shared components. The new installed checkbox now lives in components.rs
with styling/opacity in theme.rs. Initial immutable review records describe the
older checkout and earlier implementation; this snapshot corrects their current
applicability. No tests/build/apps/provider/MCP/personal-state actions were run
for this integration. Newly discovered D81 is recorded without treating this
small source fix as an instruction to resume the entire runtime acceptance task.

## November 3 — already-installed discovery rows

Active focused milestone: [installed discovery](installed-discovery/info.md).
The user requested checked, gray discovery rows for definitions already imported
into this Fetch instance. Backend matching will reuse exact saved source path/name;
UI default/manual/bulk selection and submission will exclude those rows. Distinct
accounts/configuration sources remain separate and backend installation reuse is
preserved. HTTP and MCP discovery must agree. Successful progress will update
displayed rows without rereading source files. Source is implemented in discovery,
the HTTP/MCP adapters and UI. A common eligibility rule excludes installed rows
from defaults, toggles, Select all, count and submitted payload; successful progress
updates rows and preserves failed-new-row retries. Checked disabled rows use a
shared style to dim label text as well as the box. Independent Tester and
Evaluator source/documentation reviews ACCEPT with no material findings. No new
build/runtime result exists; source-only commit/push integration is next.

The user's separate Vercel/Miro/HF questions were answered from recorded source
limits; they do not expand this fix into provider or importer support work.
No build/test/formatter/linter/validation, app/provider/MCP, personal state or
release action ran or is newly authorized.
[Plan evidence](history/2026-11-03_14-27-45-000001-planner-already-installed-discovery.md).
[Implementation evidence](history/2026-11-03_14-31-55-000001-coder-installed-discovery-state.md).
[Independent source review](history/2026-11-03_14-35-00-000001-tester-installed-discovery-source-review.md).
[Final source acceptance](history/2026-11-03_14-34-53-000001-evaluator-installed-discovery-acceptance.md).

## Historical November 3 pre-fetch local-checkout review

This review inspected local 1276516 before fetching newer GitHub commits.
Its provider/import/UI/test conclusions are superseded by the incoming October
29–31 records below; it does not describe the integrated repository.

Active goal: finish the selected MCP MVP so Codex, OpenCode and Claude Code can
each use one Fetch connection for the user's imported servers. Read-only source
inspection confirms the shared gateway, client authorization, transports and
management machinery exist; the four backend gaps in
[MVP_GAPS.md](../docs/testing/MVP_GAPS.md) still prevent claiming general everyday
inventory coverage. Remaining shared UI extraction is a separate required item.

Known provider setup is limited to Tavily/GitHub verification and Linear/Railway
browser integrations. Importing definitions deliberately does not copy their
credentials. General remote OAuth, broader safe launcher/URL imports and correct
credential batch/removal outcomes remain unfinished. Unknown authentication
does not by itself prove a no-auth server cannot connect. Railway local children
still need explicit Restart after background token refresh.

Historical verification remains 37 passing earlier-foundation tests plus later
successful native binary compilation/source reviews; no new runtime evidence
was produced. The native/provider/client acceptance roster remains unrun and its
isolation unprovisioned. No build, test, app/provider/MCP connection, personal-state
change, packaging or release occurred. Starting checkout was clean main at
1276516, matching its local origin/main tracking reference; no remote query made.

- [Connector source-review evidence](history/2026-11-03_14-21-37-000001-researcher-connector-readiness-review.md)
- [Plan, pages and testing-evidence review](history/2026-11-03_14-21-37-000002-planner-plan-pages-and-test-evidence.md)
- [Independent summary acceptance and integration](history/2026-11-03_14-21-37-000003-evaluator-status-summary-acceptance.md)

All four native pages exist: Servers, Install MCPs, Connect apps and Settings,
plus browser approval/callback pages. Connect apps provides setup/certificate
guidance rather than configuring clients automatically. The 153 Figma specimens
are design/source references, not runtime-tested pages. The original seven
discussion areas now map to the consolidated build plan: source spans all core
areas, authentication/import remain incomplete and testing/delivery acceptance
is outstanding. Independent Evaluator ACCEPT covers this factual summary only.
Shared role/workflow files referenced under ~/.codex were unavailable; explicitly
scoped read-only specialists supplied independent review without running checks.

Next: complete a per-server import/authentication coverage record from the
existing inventory, finish its selected missing flows, then obtain authorized
isolated end-to-end evidence before switching everyday clients.

## October31 — existing Google/Slack checks confirmed

Slack auth.test and Google Drive account information can reuse the shared
checker. Google needs its fixed fields=user option allowed in the trusted request.
The checks are known; wiring them to encrypted saved access and handling login
storage/renewal remain. No application source, live request or test result changed.
[Source-backed explanation](history/2026-10-31_13-02-43-203191-orchestrator-confirm-google-slack-account-checks.md).

## October31 — global clarity rule saved; Google/Slack explanation corrected

The global AGENTS.md now requires plain descriptions of what works, what is
missing and what must happen next, with the user's example. Readback confirmed
the rule once and preserved existing instructions.

Google already has browser sign-in and token refresh; Slack already has setup
and token/cookie inputs. Fetch still needs a supported way to save their new/
renewed access encrypted and use it on restart, plus saved-key checks and real
account/tool/restart tests. Current source confirms Authenticate returns unknown
for these local servers. No application code or test/build result changed.

- [Actual package login findings](history/2026-10-31_12-49-55-808975-researcher-google-slack-existing-login.md)
- [Global rule and correction](history/2026-10-31_12-49-55-808976-orchestrator-global-clarity-rule-and-login-correction.md)

## October30 delivery — generic authentication committed and pushed

Accepted D82 source/tests/docs are pushed as df094cc; HEAD/tracking/live remote
matched with a clean tree. This delivery-record follow-up is separate.
[Delivery and exact handoff](history/2026-10-30_17-21-02-095274-orchestrator-deliver-generic-authentication.md).
The bounded source/fixture milestone is complete; the parent working MVP remains
incomplete. No actual native/provider/MCP/client readiness or completion alert.

## October 30 — D82 generic authentication accepted

Generic input resolution and the trusted-description verifier are implemented
and independently accepted after lifecycle corrections. Seventeen new cases,
35 targeted manager cases and110 combined checks pass; native binary builds.
Existing check facts are trusted data, input names come from saved imported
bindings, and adding proof preserves standard remote OAuth. D25/D31 remain.
Commit/push delivery is next. Parent local working MVP remains incomplete;
no real provider/native/MCP/client result is claimed.

- [Accept the shared authentication engine](history/2026-10-30_17-19-45-557612-evaluator-accept-shared-authentication-engine.md)
- [Integrate generic authentication and its verified limits](history/2026-10-30_17-19-45-557613-orchestrator-integrate-generic-authentication.md)

## October 30 — generic engine passes corrected fixtures and native build

D82 implementation now resolves imported inputs generically and runs trusted
request descriptions through one verifier. Existing service check facts are data.
Remote OAuth remains independent of descriptor presence; only actual auth refs
or OAuth count as stored authentication. Review findings were fixed and three
regressions added. Independent 17 new cases,35 targeted manager cases and110
combined checks pass; native binary builds in 0.43s. Final evaluation/delivery
remain. Unknown providers still need trusted proof descriptions and compatible
login ownership; no live native/provider/MCP/client success is claimed.

- [Fix login selection independently of verification profiles](history/2026-10-30_17-17-32-765976-coder-preserve-generic-oauth-with-verification-data.md)
- [Verify corrected generic-authentication source and native build](history/2026-10-30_17-17-32-765977-tester-final-generic-authentication-regressions-and-build.md)

## October 30 — generic source compiles; lifecycle review fixes pending

Shared input resolver/verifier and trusted data profiles are implemented. The
first independent 32 manager and107 combined cases pass, but final acceptance is
withheld: review found new descriptions could disable generic OAuth, and unrelated
saved headers could be mistaken for existing authentication. Correct login-mode
selection/evidence, add production-path regressions and rerun before accepting.
No live provider/native/MCP/client proof or readiness notification.

- [Implement shared input resolution and trusted verification descriptions](history/2026-10-30_17-13-48-867235-coder-generic-verifier-source.md)
- [Run initial independent generic-authentication fixtures](history/2026-10-30_17-13-48-867236-tester-generic-authentication-preliminary-fixtures.md)
- [Request corrections to generic login classification](history/2026-10-30_17-13-48-867237-evaluator-request-generic-oauth-lifecycle-fixes.md)

## October 30 — D82 generic-authentication implementation active

User approved the shared engine and trusted descriptions. Planner accepted; one
writer owns Manager/helpers/data and independent Tester/Evaluator own checks and
review. No provider-name gate on input storage/delivery; proof semantics remain
D25/D31. Missing descriptions and wrapper-login limits stay explicit. The
installed CLT toolchain works process-scoped, resolving the prior shell/license
block without a global change. Source/test/build/review work is in progress.

- [Plan](history/2026-10-30_17-05-50-737001-planner-generic-authentication-implementation.md)
- [Scope and acceptance](generic-authentication/info.md)

## October 30 — generic authentication design under review

The user questions connector-specific hardcoding and asks for reusable functions
using imported credential names. Source/research confirm those inputs can drive
shared storage/delivery and standard HTTP OAuth. Direct service proof is a Fetch
D25 requirement, not a universal MCP health method. Current Manager recognition,
auth-name lists and request cases are hardcoded. A shared engine with trusted
request descriptions is proposed; no implementation/success-rule change approved.
No new product tests/builds or live connector actions occurred. The current shell
now blocks developer-tool Git/Python execution on an Xcode-license requirement;
prior 93-test/build evidence is historical and unchanged.

- [Primary research](history/2026-10-30_16-51-25-000001-researcher-generic-authentication-boundaries.md)
- [Clarification and handoff](history/2026-10-30_16-51-25-000002-orchestrator-generic-authentication-clarification.md)

## October 30 delivery — scoped feature committed and pushed

Accepted code/tests are delivered as b194de7; local HEAD, tracking ref and live
remote matched with a clean tree after push. This delivery-record follow-up is
separate. [Delivery and remaining testing handoff](history/2026-10-30_16-25-37-211141-orchestrator-deliver-parallel-authentication.md).
The parallel-authentication milestone is complete for source/fixture scope; the
parent local MVP remains incomplete. No real native/provider/MCP/client result
or completion alert is claimed.

## October 30 — scoped authentication fix accepted

POST authenticate keeps its saved-key authenticated check; supported named
API-key batches check every required covered value. Authentication and Settings
credential requests now track each server independently. Independent25 new
fixtures and93 combined checks pass, the native binary builds and final
Evaluator ACCEPTs the bounded change. Commit/push delivery is next; no live
native/provider/MCP/client readiness is claimed.

The user can begin the no-login reference-server subset after the protected
native session exists. Broader provider setup, generic remote authentication
proof, deletion/non-auth outcomes and actual installation/client/restart
acceptance remain the concrete full-MVP blockers.

- [Accept the scoped authentication fix](history/2026-10-30_16-24-52-062867-evaluator-accept-parallel-authentication.md)
- [Integrate checked credential batches and parallel service authentication](history/2026-10-30_16-24-52-062868-orchestrator-integrate-parallel-authentication-and-blockers.md)

## October 30 — authentication fixes pass independent fixtures and native build

The scoped change now passes 25 new authentication/UI checks and the combined
93-test suite; the locked native binary builds in 0.54s. Five new tests exercise
actual Desktop message handling without a window or executing Tasks. POST still
requires the saved-key authenticated service check. Supported named API-key
references can be verified in batches; per-server UI requests no longer block
one another. Final evaluation and commit/push are pending.

Remaining MVP blockers are broader provider/wrapper setup, generic remote login
proof where no supported service checker exists, credential deletion/non-auth
outcomes, protected native-session setup and actual installation/provider/client/
restart acceptance. No source fixture proves those live workflows.

- [Independent fixture/build pass](history/2026-10-30_16-22-11-273949-tester-parallel-authentication-fixtures-and-build.md)
- [Bounded milestone and criteria](parallel-authentication/info.md)
- [Current remaining work](../docs/testing/MVP_GAPS.md)

## October 30 — authentication source implemented; independent checks in progress

Backend source now checks supported saved credential groups without a one-entry
shortcut. UI source tracks authentication and credential-change requests per
server; same-server ownership and old-response protection remain. Both writers
compiled their source successfully. A test-only host factory enables actual
Desktop message tests without a window, Keychain or native authority prompt.
Independent fixtures/build and final evaluation are pending. This is not full
MVP/native/provider acceptance.

- [Implement supported multi-entry authentication verification](history/2026-10-30_16-20-04-162340-coder-backend-checked-credential-groups.md)
- [Implement independent authentication request tracking in the UI](history/2026-10-30_16-20-04-162341-coder-ui-independent-authentication-requests.md)
- [Provide safe host construction for independent Desktop update tests](history/2026-10-30_16-20-04-162342-orchestrator-safe-window-free-ui-test-fixture.md)

## October 30 — authentication-route clarification and bounded fix plan

User reaffirmed POST authenticate's saved-key service check and requested parallel
authentication across services. Read-only Planner/source review confirms PATCH's
one-entry shortcut and the UI's one outstanding auth-request slot are separate
implementation limitations; backend locks are already per server. Plan: retain
D25/D31, track UI requests per server, verify actual required credential groups,
and check independent success/failure, same-server duplicates and stale results.
No application changes/tests/runtime actions in this pass. Deletion-only success
and protected-MCP proof remain unselected; protected native testing remains pending.

- [Planner contract/source findings](history/2026-10-30_16-12-16-172362-planner-authentication-route-and-parallel-services.md)
- [Orchestrator clarification and handoff](history/2026-10-30_16-12-16-172363-orchestrator-authentication-plan-clarification.md)

D81 scoped source/evidence delivery is pushed through f6563b3 with matching local/
remote SHA and a clean tree at that checkpoint. [Delivery and exact pending handoff](history/2026-10-29_12-24-46-000001-orchestrator-scoped-local-source-delivery.md).
This record's documentation follow-up is separate; no local MVP completion is claimed.

## Active — local working MVP under D81

The user now authorizes finishing the selected source, local build/test execution,
computer-use installation of a real MCP and a separate test Codex instance. No
packaging/signing/DMG. [Scope](local-working-mvp/info.md),
[actual run and protection prerequisites](../docs/testing/LOCAL_MVP_RUN.md).
Planner and primary-source Researcher reports accepted; imports, remote auth and
shared UI source assigned separately. Two credential-contract choices are pending.
Native login-Keychain/owner-confirmation isolation remains a concrete prerequisite;
do not treat a fresh profile as sufficient. No current-pass runtime acceptance yet.
All earlier incomplete scope remains tracked below.

- [Implementation plan](history/2026-10-29_12-01-40-000001-planner-local-working-mvp-plan.md)
- [OAuth/isolation primary-source research](history/2026-10-29_12-01-40-000002-researcher-remote-auth-and-isolation.md)
- [Shared native UI source pass](history/2026-10-29_12-01-40-000003-coder-native-shared-ui-source.md)

Native/browser shared helpers, known import forms and generic remote OAuth source
are implemented. Final locked native build exit 0; independent combined suite68/68
PASS. Initial import assertion failure was corrected/retested. Source review found
and resolved issuer mix-up; final scoped ACCEPT. Clippy exits 0 with72 warnings.
No live UI/MCP/provider acceptance: a copied offline Codex starts with0 servers;
CUA test-user setup is at administrator Unlock. No account/password/security/trust
change or real MCP connection. Pending2 credential choices and protected native
session are the next required inputs. Do not mark working-local completion or notify.

- [Imports source and fixture correction](history/2026-10-29_12-16-09-000001-coder-import-compatibility-source.md)
- [Generic remote OAuth source](history/2026-10-29_12-16-09-000002-coder-generic-remote-oauth-source.md)
- [Browser/integrated native build](history/2026-10-29_12-16-09-000003-orchestrator-shared-browser-and-first-build.md)
- [Independent fixture runs/failure/retests](history/2026-10-29_12-16-09-000004-tester-fixture-failure-and-retest.md)
- [Issuer-mixup review finding](history/2026-10-29_12-16-09-000005-evaluator-oauth-issuer-mixup-finding.md)
- [Issuer binding fix and final checks](history/2026-10-29_12-16-09-000006-coder-fix-oauth-issuer-binding.md)
- [Separate offline Codex and Mac handoff](history/2026-10-29_12-16-09-000007-orchestrator-confined-offline-codex-and-handoff.md)

## October 29 — reassess the work remaining for a working MVP

At `1276516`, fresh read-only source inspection and independent specialist
reviews confirm five known implementation areas. Provider setup/general remote
login are the largest overlapping work; import/credential outcomes are medium;
shared UI extraction is smaller. Native/client/provider correctness remains
unverified. A working local MVP requires those areas plus isolated end-to-end
acceptance/fixes; distribution is a separate later stage. No percent or date is
supported. Existing app/API/manager/runner/storage boundaries and product scope
are preserved. No tests, builds, launches, connections or personal-state changes.

[Assessment scope and completion](mvp-readiness-review/info.md).
Next: known imports and server setup coverage, shared/provider login, credential
outcomes and UI helpers, then user-started isolated workflows. All unfinished
work below is preserved. This assessment changes documentation only and remains
local; it does not mark the operational MVP complete.

- [Planner source findings](history/2026-10-29_11-46-15-000001-planner-mvp-remaining-work-source-review.md)
- [Independent Evaluator ACCEPT](history/2026-10-29_11-46-15-000002-evaluator-mvp-local-readiness-review.md)
- [Orchestrator integration](history/2026-10-29_11-46-15-000003-orchestrator-integrate-mvp-readiness-assessment.md)

## October 29 handoff — full tree saved and remaining work counted

Documentation review is complete; debug/release paths and stale completion
wording are corrected. [Independent review](history/2026-10-29_11-36-36-000139-evaluator-remaining-work-documentation-review.md) and
[full-tree handoff](history/2026-10-29_11-36-36-001631-orchestrator-full-tree-push-and-documented-remainder.md) record the scope/evidence.
The final documentation commit contains this updated handoff; Git history and
the delivery response identify its hash. Earlier source/build evidence is unchanged.


The complete tracked/untracked working tree was committed and pushed as a15eacc,
including the Figma153-state index, shared-component guidance and pending history.
Current known remainder: five implementation areas (four core backend areas plus
remaining shared UI extraction), then isolated runtime acceptance/fixes and
separate distribution preparation. These are broad areas, not a percentage or
seven equal-sized fixes. [Current remaining-work list](../docs/testing/MVP_GAPS.md)
contains the source locations and finish criteria.

Railway/auth/logging source is implemented and compiled; Figma reference work is
recorded. Live native/provider/client verification remains unrun. The separate
Railway temporary-callback renderer still needs explicit Figma-state mapping.
No builds, tests, app/provider/MCP calls or personal-state actions in this
October29 documentation/push pass. No release is authorized.


## Current delivery — Railway sign-in, inline authentication and request logs

Scoped code is committed/pushed as9def537; local HEAD and origin/main matched.
[Delivery record](history/2026-10-28_21-55-32-489892-orchestrator-deliver-railway-auth-and-diagnostics.md). No runtime acceptance is claimed.

The selected Railway fix is implemented and independently source-reviewed:
Authenticate starts Fetch-owned browser OAuth, uses D80's temporary numeric
127.0.0.1 callback and privately verifies the stored credential through /oauth/me.
Inline server cards show progress/results/errors and share a masked credential
editor with Settings. Old status reads and repeated actions are guarded.

Railway's required CLI5.71.0 is prepared in its own private installation folder;
old installations repair it on explicit Connect. HOME/PATH/token handoff stay
private. Official remote Railway root URLs can be imported without rewriting.
Background token refresh for an already-running local child exposes Restart
required; Fetch does not claim that process's environment changes live.

Every first-party incoming/outgoing HTTP call has safe stderr metadata logs,
including auth outcomes/error codes. No secret bodies/headers/queries/raw URLs.
HTTP completion is headers-ready, not terminal streamed-tool completion.
Install, restart and auth monitoring now run independently.

Final command: cargo build --locked --bin fetch --message-format short,
exit 0 in4.66s (prior integrated build19.64s). Existing four Keychain deprecation
warnings and one unread protocol field warning remain. Tester and independent
Evaluator ACCEPT scoped source/build only. No tests/test-target builds, app
launch, provider/MCP/client requests, npm/CLI setup, personal state/credentials/
trust, packaging or release executed. Working runtime remains unproven.

Full MVP is still incomplete: [actual source gaps](../docs/testing/MVP_GAPS.md).
The user explicitly requires operational completeness; do not rename missing
selected features as scope deferrals. [Diagnostics guide](../docs/testing/DIAGNOSTICS.md).

- [Coder and actual build evidence](history/2026-10-28_21-52-54-894807-coder-railway-browser-flow-and-private-runtime-source.md)
- [Independent source verification](history/2026-10-28_21-52-54-895007-tester-railway-callback-and-diagnostics-source-acceptance.md)
- [Independent final acceptance](history/2026-10-28_21-52-54-895117-evaluator-accept-scoped-railway-authentication-and-logging.md)

## Latest Figma delivery — complete current UI state library

The official file now contains 153 source-driven states/specimens: the 17-frame
baseline plus 21 authentication, 28 individual Settings, 26 setup/server,
25 browser request/error, six native-surface contracts and 30 control/message/
minimum-window specimens. All nine current authentication phases and the shared
credential_editor are represented. Current UI structure and scrolling remain;
native OS appearance is explicitly unobserved rather than fabricated.

Source inventory and representative Figma review independently accepted the
reviewed scope after corrections. No application source/runtime action was part
of this design pass. Concurrent operational-MVP implementation above is separate.
See [the complete coverage/node index](../docs/UI_STATE_COVERAGE.md) and
[integration record](history/2026-10-28_21-57-19-222233-orchestrator-complete-current-ui-state-library.md). User visual feedback and previous
runtime acceptance remain open. Current SF Pro rendering limitations remain.

## Prior correction — literal current-app UI baseline

The user rejected the styled mockups. Current boards now reproduce the source's
actual UI: 13 native states, four browser HTML states, seven current native shared
component families and separate browser form controls. An unmodified screenshot
of the already-open empty Servers window sits beside its editable counterpart.
Current navigation, Iced controls, plain text status/errors, single-column forms
and scrolling are preserved. There are 59 native prototype links.

The original three boards are archived as rejected proposals; do not implement
their added dashboard, badges, alert panels or two-column Settings. Figma cannot
render native SF Pro: calibrated editable Arimo is used with original sizes in
metadata. Other states are source-based fixtures, not observed runtime states.
No app launch/operation, source change, test, build or managed connection occurred.
See [current inventory](../docs/FIGMA.md) and
[correction record](history/2026-10-28_21-17-10-000001-orchestrator-literal-current-ui-correction.md).

## Historical planning checkpoint — inline authentication before the fix

Superseded by the implemented/compiled Railway delivery above. The following
paragraph records the earlier diagnosis, not current provider support or build status.

Authenticate currently drops required/unknown outcomes into a generic Saved notice.
Implement dedicated per-server progress/results and shared inline credential entry
through existing native Settings APIs. Railway has no supported login adapter;
show that blocker honestly. Planner and initial source complete; independent source review underway. First binary
build failed on a shared spacing type; correction/rebuild pending.
No tests or live authentication started. [Scope](inline-server-authentication/info.md).
[Independent source findings](history/2026-10-28_21-20-58-629950-tester-inline-authentication-source-findings.md).
[Coder/first-build record](history/2026-10-28_21-20-38-780249-coder-inline-authentication-source.md).
[Planner record](history/2026-10-28_21-17-07-990957-planner-inline-server-authentication.md).

## Latest requirement — shared UI components in code

The user explicitly requires commonly reused UI elements to be defined once and
shared in code as well as Figma. Seven families now have responsibility/input
definitions and a recorded reuse rule in [the component registry](../docs/SHARED_UI_COMPONENTS.md),
AGENTS.md, the native UI guide and Figma descriptions/guide. Shared theme values
and platform-appropriate rendering helpers are required. Figma instances exist;
the credential editor is already shared; remaining custom UI extraction is
pending. The linked requirement-recording pass changed documentation/Figma only;
the later authentication implementation above added the shared editor.
[Pass record](history/2026-10-28_20-49-03-000001-orchestrator-shared-ui-code-requirement.md).

## Historical delivery — styled designs, subsequently rejected

The official Figma file now contains 17 editable frames: 13 current native
page/state designs and four browser approval/callback states. Populated sample
data covers running servers, active/queued calls, partial tool catalogs,
discovery, installation and retry, settings, credentials and app connection.
Seven shared component families are used as linked instances across pages;
62 native prototype links cover navigation and selected workflow transitions.

Screenshots were inspected and clipped content corrected; all native page body
bounds fit 1080×760 frames. This is Figma design inspection, not independent
review or app/runtime testing. Designs remain proposals; no application source,
personal state, trust, build or managed MCP execution changed.
See [inventory](../docs/FIGMA.md), [scope](figma-app-pages/info.md) and
[pass record](history/2026-10-28_20-46-58-000001-orchestrator-complete-figma-pages.md).
Earlier unfinished runtime checks remain below.

## Latest documentation — official Figma project

On October 28, the user selected [Fetch App Mockup](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=0-1&t=rEUDUra6cAWf3pxc-1) as Fetch's official Figma project for
ongoing design work. Saved in AGENTS.md, PROJECT_CONTEXT.md, NOTES.md and
[docs/FIGMA.md](../docs/FIGMA.md). Application and testing status below are unchanged.
Documentation-only pass; no builds, tests, app launch or MCP connections.
[Pass record](history/2026-10-28_18-35-39-812156-orchestrator-official-figma-project.md).


## Latest change — bounded Install list and bulk selection

The discovery list now owns a bounded scroll area with a stable widget location
and ID; Install selected servers remains in a fixed footer with selected count.
Header/file picker/discovery/Select all remain above the list. Errors/notices and
progress details scroll within it. Select all picks supported definitions only
(including source-disabled supported entries as explicit choice) without changing
source flags, discovering or installing. Other pages retain previous scrolling.

Independent source verification and Evaluator ACCEPT. Locked native binary build
succeeded, exit 0 in10.45seconds; existing warnings only. No new tests or native/UI
execution by agents. Long-list/minimum-window/scroll preservation still need
manual checks after relaunch. The prior user screenshot is evidence of the old
overflow, not verification of the fix. See [scope](install-list-controls/info.md).

Unsupported-entry diagnosis is recorded in the development guide; no importer
behavior was changed or server/network compatibility test performed.

- [Planner report](history/2026-10-28_18-22-18-439052-planner-bounded-install-list-plan.md)
- [Coder report](history/2026-10-28_18-22-18-439053-coder-bounded-install-list-and-select-all.md)
- [Tester report](history/2026-10-28_18-22-18-439054-tester-bounded-install-list-source-verification.md)
- [Evaluator report](history/2026-10-28_18-22-18-439055-evaluator-bounded-install-list-source-acceptance.md)
- [Orchestrator diagnosis/build/integration](history/2026-10-28_18-22-18-439056-orchestrator-install-list-integration.md)


## Latest change — native config-file picker

The Install MCPs screen now has Choose file… beside its editable path field.
It opens a native one-file sheet, includes hidden folders such as .codex, fills
an absolute path and preserves input on Cancel. Selection alone does not read
config content, discover or install. UI/callback lifecycle received independent
source-verification and Evaluator ACCEPT with no requested fixes.

`cargo build --locked --bin fetch --message-format short` succeeded, exit 0 in
11.35seconds. No tests, app/dialog, login, install or MCP connections were run by
agents. Interactive checks remain pending after relaunch; prior development path
separation and all earlier unfinished runtime acceptance remain intact.
See [picker scope](config-file-picker/info.md) and the updated development guide.

- [Planner report](history/2026-10-28_17-31-23-020385-planner-config-picker-plan.md)
- [Coder report](history/2026-10-28_17-31-23-020386-coder-config-picker-source.md)
- [Tester report](history/2026-10-28_17-31-23-020387-tester-config-picker-source-verification.md)
- [Evaluator report](history/2026-10-28_17-31-23-020388-evaluator-config-picker-source-acceptance.md)
- [Orchestrator build/integration](history/2026-10-28_17-31-23-020389-orchestrator-config-picker-integration.md)


## Latest evidence — development binary compiles

`cargo build --locked --bin fetch --message-format short` succeeded with exit 0
in9.74seconds after the development-state changes. The binary was not executed.
Four native Keychain deprecation warnings and one unread protocol-field warning
remain. The four new regression tests remain unexecuted. No native/MCP/client
login, package installation, personal config/trust change, packaging or release.
See [compilation record](history/2026-10-28_17-18-50-885130-orchestrator-development-binary-compilation.md). Older source-only pass descriptions
below are retained as chronology; current verification adds compilation only.

## Current milestone — portable development state

User goal: import supported normal global Codex definitions into development
Fetch, then connect a fresh test Codex client with Fetch as its only MCP. Main
Codex may connect to the same instance. One shared Fetch manages servers/provider
sessions; clients receive distinct grants.

Implemented native debug defaults: current-home-relative
`~/Library/Application Support/Fetch-Development/`, port53672. Release retains
`~/Library/Application Support/Fetch/`, port53671. Installer/runtimes/caches,
repository/credentials/application identity, persistent runner state and
Keychain names follow the selected root. Existing API contracts are unchanged.
Explicit paths/ports win. Null stdio cwd now defaults to private server home;
explicit imported directories and original scripts remain unchanged.

Source and documentation independently reviewed/accepted; four regression tests
added but not run. No new agent-run build/test/app/Keychain/client/MCP/install or
trust result. The user reported running the pre-change development build. No
old-state migration, global client edit, cleanup, commit/push or release.

Next: deliberately rebuild before using defaults; prepare source-backed
single-Fetch test profile, fresh credentials and normal TLS trust, then execute
selected checks when started. See [development guide](../docs/testing/DEVELOPMENT.md)
and [milestone scope](development-state/info.md). Prior history below is retained.

- [Planner report](history/2026-10-28_17-17-15-069401-planner-portable-development-state-plan.md)
- [Coder report](history/2026-10-28_17-17-15-069402-coder-separate-development-defaults.md)
- [Tester report](history/2026-10-28_17-17-15-069403-tester-portable-development-regression-source.md)
- [Evaluator report](history/2026-10-28_17-17-15-069404-evaluator-development-state-source-acceptance.md)
- [Orchestrator integration](history/2026-10-28_17-17-15-069405-orchestrator-integrate-development-state.md)


## October 28 follow-up — testing preparation

The user resumed testing planning, accepted the Xcode license after a reported
compiler failure, and reports running the development build. A successful native
launch has not been observed by agents. Read [the first-session plan](../docs/testing/FIRST_SESSION.md).
Planner and Tester inspected source independently; Evaluator accepted the bounded
plan. No agents ran builds, tests, services, apps or connections in this pass.
The five-server roster is still not run; actual isolation remains unprovisioned.
Previous compile-only evidence below is historical, not current runtime evidence.

- [Planner report](history/2026-10-28_15-14-39-691532-planner-first-session-plan.md)
- [Tester report](history/2026-10-28_15-14-39-691533-tester-first-session-test-inventory.md)
- [Evaluator report](history/2026-10-28_15-14-39-691534-evaluator-first-session-plan-acceptance.md)
- [Orchestrator report](history/2026-10-28_15-14-39-691535-orchestrator-first-session-planning-integration.md)


October 28, 2026. Active milestone: [MCP MVP compilation](mcp-mvp-compilation/info.md). D79 compile-only milestone is complete: the native binary builds and fixes/lockfile were pushed as70e0f78. **Tests, app launch and MCP connections remain prohibited.**

## Current state and boundaries

The selected source exists through f9e3dab, including native UI, shared APIs, manager, raw runner/gateway and protected authority. First compilation resolved Cargo.lock and exposed a gateway syntax error. After the syntax/import fixes, locked native binary compilation succeeded. Native classic-Keychain APIs emit deprecation warnings; storage behavior is unchanged. Module/data boundaries and selected APIs remain those in the architecture records.

No tests/test targets, binary/application/service launch, MCP/provider connections, managed server/runtime setup, Keychain/trust/credential actions, packaging or release. Compilation does not establish runtime/client/native acceptance.

## Verification and next action

First command: `cargo build --bin fetch --message-format short`, exit101 at src/mcp/mod.rs:151. Corrected build: `cargo build --locked --bin fetch --message-format short`, exit 0 in12.61seconds, producing target/debug/fetch. Toolchain1.95.0; dependencies resolved under manifest Rust1.89 compatibility. Four native classic-Keychain deprecation warnings and one unread protocol-error field warning remain. Independent source/build-evidence review ACCEPT. Fixes and Cargo.lock are committed/pushed as70e0f785fcc560f1af234ea6c9c6a9aadb97fe90; local HEAD and origin/main agreed after push. Stop; do not run the executable. Every runtime/test step remains for the user's separate instruction.

## Current pass history

- [Plan compilation-only build fixes](history/2026-10-28_14-59-16-596517-planner-compile-only-plan.md)
- [Run first native binary compilation](history/2026-10-28_14-59-16-596619-orchestrator-first-binary-compilation.md)

- [Compiler fix and successful binary build](history/2026-10-28_15-00-36-928523-coder-binary-build-error-fix.md)

- [Independent compiler-fix acceptance](history/2026-10-28_15-02-42-900848-evaluator-accept-compile-fix-delivery.md)

- [Compile-fix delivery](history/2026-10-28_15-03-44-962679-orchestrator-compile-fix-delivery.md)

## Prior source and future acceptance

[Source milestone](mcp-mvp-implementation/info.md), [source handoff](../docs/testing/HANDOFF.md), [earlier snapshot](milestone-before-d77-source.md), [deferred tasks](to-do-before-d77-source.md) and [isolated testing](isolated-mcp-testing/info.md) retain prior records. D78 custom GitHub CLI deferral and all earlier scope deferrals remain. The five-server roster is still not run and the isolation boundary unprovisioned.
