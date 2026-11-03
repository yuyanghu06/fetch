# Direct installation inputs — backend implementation

Goal: let users paste an MCP launch command which Fetch translates into its
existing internal definition and shared installer. Keep source-file imports;
direct remote URLs are included in the approved extension.

User clarified command-to-definition behavior. The approved public
contract is in docs/architecture/api-contract.md under the November3 direct
command/URL installation amendment. It accepts server_name plus exactly one
of config_path/command/url; existing supported launch forms, quote-aware command
parsing without a shell, stable direct-definition reuse, existing authority and
progress/connect/error behavior. No README interpreter or credential-value input.
User approved adding these inputs to the existing POST body after the full
proposal, explicitly without UI. Backend source is implemented and inspected. Independent Tester/Evaluator
ACCEPT;142 unique checks and locked native compilation pass.
[Verification evidence](evidence.md) records commands and live-acceptance limits.

Completion criteria: implement through shared classification and
preparation; preserve config import identities and per-instance encrypted state;
independently test direct forms/progress/reuse/errors and evaluate. Preserve
Google deferral and unrelated concurrent UI/startup source changes.


Independent research selected already-locked shlex2.0.1 (MIT OR Apache-2.0),
with explicit restricted command-syntax checks before quote removal. Shared
source capture must preserve import behavior; direct credential-bearing bridge
headers must not be persisted as nonsecret identity. No shell or helper runs to
parse commands. Planner completed a bounded implementation/acceptance plan.


Implemented source: shared discovery capture, restricted quote-aware direct.rs,
three selection forms in installation, validated direct_identity digest metadata
and matching MCP schema. Direct progress retains config_path:null and hides
submitted text. No UI or manager edits in this feature. Exact shlex dependency
was already locked; no package/runtime command ran during source implementation.


Status: selected backend scope accepted, scoped Git delivery next. No real package
installation, Connect or provider login was performed. Existing Google deferral
and broader native/provider/client acceptance remain; no UI is added here.
