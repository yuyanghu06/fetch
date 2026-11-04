# Private programs available during startup

Goal: a managed MCP can run helper programs installed for that instance, including Railway’s privately installed CLI.

Approved scope: installer records executable folders as relative private metadata; shared native startup and recorded setup prefix these folders to PATH (the folders searched for named programs). Existing rows use the installer’s known layout in memory when metadata is absent. Configured PATH remains the suffix; default executable/system folders remain otherwise. No recursive scan, global inheritance, public API, credential migration or login-code changes.

Completion requires a real confined disposable helper launch, installation separation, fresh/legacy metadata checks, existing startup/storage regressions, locked build and independent evaluation. Real Railway account sign-in is outside these fixture checks.

Status: complete for the scoped startup repair. Independent 60 checks (10 new), locked native build and Evaluator ACCEPT. See [evidence](evidence.md). Real Railway account sign-in and app relaunch remain untested. Orchestrator owns journal/docs; Coder and Tester own separate source/test files.


November4 follow-up note: user reports GitHub's install script does not bundle gh and selects globally installed gh as its default executable. This is a recorded follow-up, not a change to the completed private-helper repair or a claim that global lookup has been tested. Credential/account-folder rules remain unchanged.

Read-only follow-up confirms the existing development GitHub entry already invokes global gh by absolute path through its imported launcher, not a downloaded package. Generated terminal PATH omits Homebrew and private account folders still apply; no lookup/account/login behavior changed. Details and versioned upstream alternatives are recorded in architecture research/NOTES.md.

November4 user-approved follow-up now preserves the inherited default PATH after private folders and executable parent. This supersedes the historical system-only default/no-global-inheritance note above for command discovery only; private account folders remain. Scoped implementation and current evidence belong in milestones/normal-program-path/.
