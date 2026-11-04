# Record and use installed program folders

- Local timestamp / acting role / milestone: 2026-11-04T13:25:27.092744-05:00 / Coder (recorded by Orchestrator) / private-program-paths
- Requested scope: Implement approved shared helper program discovery, including existing Railway installations.
- Starting repository state: Prior dirty work preserved; independent plan accepted.
- Work performed / changed files: src/installation/mod.rs records private relative program_directories; new src/installation/program_paths.rs owns known npm/Python/helper layouts and contained resolution; src/manager/native_state.rs prefixes folders to configured/default PATH for shared ordinary/helper startup; src/storage/mod.rs admits and validates optional metadata. Orchestrator inspected actual source/diff.
- Decisions: Installer retains existing Railway dependency knowledge; shared startup has no provider branch. Legacy rows resolve in memory without writes/reinstall. No recursive scan, global environment inheritance, public API or account change.
- Checks and evidence: Source inspected; validation rejects traversal/absolute/non-folder escaping paths; missing directories do not become startup prerequisites. Tester owns independent meaningful checks; no pass claimed before execution.
- Resulting state / acceptance: Source implemented, independent test/build/evaluation pending.
- Remaining work and handoff: Actual confined helper execution, metadata/privacy/legacy separation checks, regressions/build and evaluator acceptance. No real provider login tested.
