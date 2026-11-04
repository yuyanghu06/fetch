# Repeated development Keychain prompts

Goal: explain approximately five identical login-Keychain dialogs and identify
whether normal access can require fewer approvals. Scope is read-only source,
public Apple/implementation research and binary/file-existence metadata; no secret
reads, Keychain/signing/trust changes or native runtime experiments.

Status: source and binary metadata reviewed; dedicated Researcher completed
primary-source comparison. Duplicate key lookups, identical labels and ad-hoc
rebuild identity are established; exact observed dialog sequence remains untraced.
Recommendations are stable dev signing and optional per-process cache; one
wrapping-key alternative needs a new storage/migration decision. No fix selected
or implemented. Canonical [research/options](../../docs/architecture/keychain-prompts.md).
Independent Evaluator ACCEPTs scoped findings and proposal limits.
Completion: report concrete reasons and limits, document recommendations and keep
personal state untouched. Future implementation needs selected scope and actual
protected native prompt evidence; do not assume approval from this investigation.


User subsequently selected stable DEVELOPMENT signing plus process caching and
requested latest main pull. Main d19d891 and prior local work are integrated;
implementation planning is active. Shared wrapping-key migration is not selected.
Any OS password/approval must remain under actual human control.
