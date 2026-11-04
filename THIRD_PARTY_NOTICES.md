# Third-party dependency notices

Fetch's own code is MIT licensed. Dependencies keep their own licenses. The vendored iced_term terminal widget is MIT licensed; its original license is retained at vendor/iced_term/LICENSE. Fetch's external-transport adaptation is described below.

Direct dependencies are declared in Cargo.toml. Important sources include:

| Component | Source | License declared by upstream |
| --- | --- | --- |
| Iced | https://github.com/iced-rs/iced/tree/0.14.0 | MIT |
| Tokio / Axum | https://github.com/tokio-rs | MIT |
| Reqwest | https://github.com/seanmonstar/reqwest | MIT OR Apache-2.0 |
| Rustls / PEM reader | https://github.com/rustls | MIT OR Apache-2.0 OR ISC, depending on component |
| RustCrypto AES-GCM / SHA-2 | https://github.com/RustCrypto | MIT OR Apache-2.0 |
| Serde / serde_json | https://github.com/serde-rs | MIT OR Apache-2.0 |
| shlex2.0.1 | https://github.com/comex/rust-shlex | MIT OR Apache-2.0 |
| rcgen | https://github.com/rustls/rcgen | MIT OR Apache-2.0 |
| ring (TLS/signature dependency) | https://github.com/briansmith/ring | ISC and upstream notices |

This source-stage inventory is not a complete resolved dependency notice bundle. D77 prohibited dependency resolution/build/packaging in the source pass. D79 subsequently resolved Cargo.lock and compiled the native binary, without collecting a final binary license-text bundle or running packaging. Before distributing a binary, the user-controlled release work must collect and include the exact resolved direct/transitive license texts and bundled-runtime notices. Fetch-managed Node, Python and uv downloads retain their own license files under their private runtime installation directories. No runtime or server package has been downloaded here.

## Vendored terminal widget — November4

iced_term0.8.0 is copied under vendor/iced_term from its verified crates.io archive (upstream crateVCS e0e90be2c160bd68f1bf544a51bf772d75b98ffc). Copyright(c)2024IliaShvyrialkin; MIT license retained in that folder. Fetch modifies src/backend.rs, src/terminal.rs and src/lib.rs to add an external-input/output seam so its backend owns shell processes while the widget’s existing Alacritty parser/renderer/keyboard/selection behavior is reused. src/view.rs also tracks terminal identity so a replaced screen receives the actual viewport size, and follows Iced Canvas local coordinates/translation/clipping so the terminal embeds correctly in scrolling server cards. It also fixes active mouse releases outside the pane and pixel-wheel direction/capture for the embedded terminal, and consistently uses the parent-adjusted pointer for mouse movement inside scrolled pages. No provider login code is imported.

Direct terminal dependencies include portable-pty0.9.0 (MIT, https://github.com/wezterm/wezterm) and alacritty_terminal0.25.1 (Apache2.0, https://github.com/alacritty/alacritty). The crate archive’s own source/manifest/README/license remain together; generated/dependency folders do not receive project info.md files. This source notice does not claim a complete distribution license bundle or authorize packaging.

## Adapted Codex startup environment — November 4

Fetch adapts OpenAI Codex's MCP process environment builder into `src/manager/process_environment.rs`, pinned to commit `75424f87ca2b1c482e0509a5b1ceb7459587f396`. Original sources: [builder](https://github.com/openai/codex/blob/75424f87ca2b1c482e0509a5b1ceb7459587f396/codex-rs/rmcp-client/src/utils.rs), [certificate variable names](https://github.com/openai/codex/blob/75424f87ca2b1c482e0509a5b1ceb7459587f396/codex-rs/network-proxy/src/certs.rs), and [blocked internal variable names](https://github.com/openai/codex/blob/75424f87ca2b1c482e0509a5b1ceb7459587f396/codex-rs/protocol/src/shell_environment.rs).

This adapted code is Apache-2.0 licensed. The full upstream [license](LICENSE-CODEX-APACHE-2.0) and [NOTICE](NOTICE-CODEX) are retained in the repository root. Changes remove Codex-specific configuration dependencies, provide an injectable parent environment for tests, preserve Fetch's private HOME/XDG/helper PATH overrides, and convert at Fetch's existing string-valued transport boundary without lossy conversion. No Codex authentication or credential-store implementation is imported. Fetch's own original code remains MIT licensed.
