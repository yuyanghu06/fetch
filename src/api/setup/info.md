# Setup reads

The discovery GET validates one optional absolute path, then calls the read-only discovery service. Management permission is checked first. This read never installs packages, starts helpers, logs in or migrates credentials. Installation adapters are wired separately through `api/runtime.rs` and the shared `installation`/`manager` backend services. Their source exists; runtime behavior remains unverified until the user starts testing.
