# await provisional startup child

- Local timestamp / acting role / milestone: 2026-11-04T12:07:13.468393-05:00 / coder / startup-diagnostics.
- Requested scope: User reports genericGoogle startupfailure after missing-input preflightremoved; showactualstartupfailureevidence.
- Starting repository state: Input-pass-through accepted176checks; currentconsole and unrelateddirtysource preserved.
- Work performed / changed files: Provisional localConnection is tracked by Server beforeinitialize, remainsnotreadyforcalls. disconnect/Quit awaitit, idempotentclose recordsconfirmedexit insidechildMutex beforeparallelcloser runs. Fixes actualquit-before-reap race and supportsfailedstartretry. No test weakening orruntimecommands by Coder.
- Decisions: Bounded sanitized configured-local-startupstderr/naturalexit evidence, nativehuman only. No helper/remote-response-body/fulltoolstream promise; no credentialreads ormandatoryinputs.
- Checks and evidence: Source/report inspected byroot; finalcommands/results summarized in scoped evidence. ActualGooglecausecannotberecoveredfromdiscardedoldoutput.
- Resulting state / acceptance: Correctedbackend181checks pass; final UI extension forordinaryStart/Connect has31targetedchecks/build/reviewpending.
- Remaining work and handoff: Finish final UI/build/evaluation docs; usermustrelaunch/retry toobserveactualGoogleerror, no accountsuccessclaimed.
