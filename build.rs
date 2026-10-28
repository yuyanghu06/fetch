fn main() {
    println!("cargo:rerun-if-changed=src/native_auth/bridge.m");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        cc::Build::new()
            .file("src/native_auth/bridge.m")
            .flag("-fobjc-arc")
            .flag("-fblocks")
            .compile("fetch_native_auth");
        for framework in ["Foundation", "Security", "LocalAuthentication", "AppKit"] {
            println!("cargo:rustc-link-lib=framework={framework}");
        }
    }
}
