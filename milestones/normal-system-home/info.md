# Keychain compatibility while preserving instance logins

Current instruction: user requires login for a locally installed Railway CLI to stay with that Fetch instance. Normal-HOME inheritance is withdrawn. A brief source change was restored before runtime tests or launch. Private HOME, XDG, packages/helper PATH and working folders remain.

Apple source establishes that redirected HOME can affect Keychain discovery. The fix remains unresolved; installing a binary privately does not create a separate credential store. Research supported native storage/Keychain options without reading credentials or changing personal Keychain state. No new policy or provider-specific code is selected.

Earlier research/plans/history remain evidence of the withdrawn proposal, not current approval.

Latest request asks HOME matching during local/global PATH lookup. Shell and Fetch source audits show this is not a built-in PATH feature: Fetch controls initial spawn only; shared runtime and absolute helper paths make location-only switching unsafe for instance logins. A deliberate top-level app wrapper is possible with limited coverage but is not selected. All withdrawn implementation/fixture edits are restored. Further code awaits revised bounded behavior; normal private HOME/package lookup remains.
