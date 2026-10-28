# Server preparation

`mod.rs` captures selected source entries without retaining credential values. It keeps one installation group, prepares entries sequentially, reuses saved successful imports and retains progress for the current run. Packages and runtimes belong under Fetch's private Application Support root; durable server records remain separate.

Source includes pinned Node 24.22.1, uv 0.13.5 and Python 3.13.18 provisioning, HTTPS download limits, published SHA-256 comparison, private preparation homes/caches and isolated npm/Python environments. No runtime setup or package command was executed during implementation. Fresh-install behavior and all failure/cleanup paths await user-controlled testing.

Primary references: [Node distribution](https://nodejs.org/download/release/latest-v24.x/), [Python release](https://blog.python.org/), [uv release](https://github.com/astral-sh/uv/releases/tag/0.13.5), [uv storage](https://docs.astral.sh/uv/reference/storage/) and [managed Python](https://docs.astral.sh/uv/guides/install-python/). No third-party implementation was copied.
