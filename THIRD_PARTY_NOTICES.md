# Third-party dependency notices

Fetch's own code is MIT licensed. Dependencies keep their own licenses. No third-party implementation source has been copied into this repository for this pass.

Direct dependencies are declared in Cargo.toml. Important sources include:

| Component | Source | License declared by upstream |
| --- | --- | --- |
| Iced | https://github.com/iced-rs/iced/tree/0.14.0 | MIT |
| Tokio / Axum | https://github.com/tokio-rs | MIT |
| Reqwest | https://github.com/seanmonstar/reqwest | MIT OR Apache-2.0 |
| Rustls / PEM reader | https://github.com/rustls | MIT OR Apache-2.0 OR ISC, depending on component |
| RustCrypto AES-GCM / SHA-2 | https://github.com/RustCrypto | MIT OR Apache-2.0 |
| Serde / serde_json | https://github.com/serde-rs | MIT OR Apache-2.0 |
| rcgen | https://github.com/rustls/rcgen | MIT OR Apache-2.0 |
| ring (TLS/signature dependency) | https://github.com/briansmith/ring | ISC and upstream notices |

This source-stage inventory is not a complete resolved dependency notice bundle. D77 prohibited dependency resolution/build/packaging in the source pass. D79 subsequently resolved Cargo.lock and compiled the native binary, without collecting a final binary license-text bundle or running packaging. Before distributing a binary, the user-controlled release work must collect and include the exact resolved direct/transitive license texts and bundled-runtime notices. Fetch-managed Node, Python and uv downloads retain their own license files under their private runtime installation directories. No runtime or server package has been downloaded here.
