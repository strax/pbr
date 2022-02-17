use std::env;

fn feature_enabled(feature: &'static str) -> bool {
    env::var_os(format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"))).is_some()
}

fn get_library_name(profile: &str) -> &'static str {
    let secure = feature_enabled("secure");
    match (profile, secure) {
        ("Debug", true) => "mimalloc-secure-debug",
        ("Debug", false) => "mimalloc-debug",
        (_, true) => "mimalloc-secure",
        (_, false) => "mimalloc"
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    eprintln!("Secure mode: {}", feature_enabled("secure"));
    let mut build = cmake::Config::new("mimalloc");
    build
        .define("MI_OVERRIDE", "OFF")
        .define("MI_OSX_INTERPOSE", "OFF")
        .define("MI_OSX_ZONE", "OFF")
        .define("MI_BUILD_OBJECT", "OFF")
        .define("MI_BUILD_SHARED", "OFF")
        .define("MI_BUILD_TESTS", "OFF")
        .define("MI_INSTALL_TOPLEVEL", "ON");
    if feature_enabled("secure") {
        build.define("MI_SECURE", "ON");
    }
    let out_dir = build.build();
    let include_dir = out_dir.join("include");
    println!("cargo:rustc-link-search=native={}", out_dir.join("lib").to_str().unwrap());
    println!("cargo:include={}", include_dir.to_str().unwrap());

    println!("cargo:rustc-link-lib=static={}", get_library_name(build.get_profile()));
}