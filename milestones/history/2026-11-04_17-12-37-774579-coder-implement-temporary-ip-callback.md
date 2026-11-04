# Implement temporary ip callback

- Local timestamp / acting role / milestone: 2026-11-04T17:12:37.774579-05:00 / coder, serialized by Orchestrator / client-identity.
- Requested scope: DeployFetchpublicidentity onpersonalwebsite and configure rmcp; user explicitlyselects temporaryHTTP127callback insteadlocalhost.
- Starting repository state: ExistingdirtyFetchsource/docs andsite dev work preserved. Isolatedsiteworktree basedactualproductionaf22b2c; selectedcanonicalwwwmetadataURL.
- Work performed / changed files: Inspected new manager/remote_callback.rs and targeted Manager/sharedAPIchanges. Bind127.0.0.1:0 beforeSDKbegin; state-bound callback-only router; one trackedtask ownsserving+expiry; ProviderFlowclose/Drop, browserfailure andQuit gracefullyclose. MainHTTPS/cert unchanged; optionalissuer stillSDKvalidated.
- Decisions: SDKownsclientselection/state/issuer/PKCE/tokenhandling. FetchownsminimalHTTPcallbackreceiver andpendingattemptcleanup. No fullHTTPproductrouter, TLSidentity/trustmigration, guessedlogincommand or newprovideradapter. User'sfocusedtestingpriority observed.
- Checks and evidence: Source saved; noCoder tests/liveproviders. Tester owns onefeaturetest. Productioninitialdeploymentdpl_DAaZAMoRvu9ZgAHc37R4xkbjGut1READY; sitebuild18staticpagesPASS. Testcommands/evidence in milestones/client-identity/evidence.md. No realaccount/browser/nativeKeychain/clientsetting changes or Fetchlaunch/signing/release.
- Resulting state / acceptance: Scopedsource/sitechangeaccepted; user realVercelsign-in remains separate acceptance. No universalprovidercompatibility claimed.
- Remaining work and handoff: ExplicitlyQuitFetch and reopenthrough ./scripts/run-development.sh before actualretry. Personal-siteURL canmove laterwithcorrespondingclientidentityconfiguration; newauthorization maybeneeded. Preserve separateMVP/native/providerwork.
