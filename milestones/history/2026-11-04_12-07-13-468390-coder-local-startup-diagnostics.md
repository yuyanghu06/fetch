# local startup diagnostics

- Local timestamp / acting role / milestone: 2026-11-04T12:07:13.468390-05:00 / coder / startup-diagnostics.
- Requested scope: User reports genericGoogle startupfailure after missing-input preflightremoved; showactualstartupfailureevidence.
- Starting repository state: Input-pass-through accepted176checks; currentconsole and unrelateddirtysource preserved.
- Work performed / changed files: Runner captures startupstderr into sanitizedboundedmemory (32lines/16KiBtotal/1KiBline), stripsHTTPURLs/knownenv+args/credentialassignments before retention, exposes separate startup_diagnostics getter and actualnaturalexitcode/signal. Frozenready/failure/resetnextspawn. No generalRunner snapshot changes or runtimeexecution by Coder.
- Decisions: Bounded sanitized configured-local-startupstderr/naturalexit evidence, nativehuman only. No helper/remote-response-body/fulltoolstream promise; no credentialreads ormandatoryinputs.
- Checks and evidence: Source/report inspected byroot; finalcommands/results summarized in scoped evidence. ActualGooglecausecannotberecoveredfromdiscardedoldoutput.
- Resulting state / acceptance: Correctedbackend181checks pass; final UI extension forordinaryStart/Connect has31targetedchecks/build/reviewpending.
- Remaining work and handoff: Finish final UI/build/evaluation docs; usermustrelaunch/retry toobserveactualGoogleerror, no accountsuccessclaimed.
