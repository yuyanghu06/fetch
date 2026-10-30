# Generic credential inputs and reusable verification

## Goal and scope

October 30, 2026: implement the user-approved shared authentication design. Read
credential input names/references from imported server configs, reuse existing
encrypted storage and delivery, and execute trusted verification descriptions
through one shared runner. Keep standard remote OAuth and existing lifecycle
special cases. The prior per-server UI concurrency work remains intact.

## Completion criteria

- Input collection/storage/delivery accepts imported names without a provider
  variable-name allowlist. Non-auth inputs are not guessed to be bearer tokens.
- Trusted data describes destination/method/placement and semantic success; the
  engine resolves actual saved references and supports joint multi-value checks.
- POST authenticate and credential-save success still require D25/D31 proof from
  encrypted readback; every submitted auth SET participates and old OAuth cannot
  approve a bad new key. Missing/ambiguous proof never becomes success.
- Production targets cannot come from imported/model/provider-controlled values;
  there is no new public route/config schema or saved-data migration.
- Independent fixtures, native binary build and final evaluation accept the
  scoped change. Record limits and remaining actual provider/native/client work.

## Status and boundaries

Complete for the bounded source/fixture scope after independent Evaluator ACCEPT.
Feature source/tests/docs are committed and pushed as df094cc with matching
live refs and a clean checkpoint. This delivery-record follow-up is separate;
the parent local working MVP remains incomplete.
The writer separated login mode from verification descriptions, excluded unrelated
headers from auth evidence and preserved manual-only remote proof. Three new
regressions pass. Independent 35 targeted manager/110 combined cases and native
binary build pass. Root owns all tracking and docs. Existing research is
reused. Process-scoped installed CLT tools work without changing Mac settings.
No live connector/provider/native/Keychain/personal-client work, packaging or
release is part of this refactor. Generic delivery does not invent a verifier
or fresh wrapper login for unknown providers.
