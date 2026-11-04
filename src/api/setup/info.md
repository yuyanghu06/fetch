# Setup reads

Discovery uses the shared installed-aware backend projection. Rows include an installed flag and existing server ID for an exact saved source file/name match; installed rows are excluded from new selections. The matching MCP tool uses the same method. No process, login or source write occurs during this read.

The discovery GET validates one optional absolute path, then calls the read-only discovery service. Management permission is checked first. This read never installs packages, starts helpers, logs in or migrates credentials. Installation adapters are wired separately through `api/runtime.rs` and the shared `installation`/`manager` backend services. Their source exists; runtime behavior remains unverified until the user starts testing.


November 3: the optional absolute config_path accepts either one file or a base folder (home or `.codex`). No query remains the normal default scan. Authorization still precedes filesystem access. Directory selection returns actual config file paths for later selected-server installation; it does not install all discovered entries.
