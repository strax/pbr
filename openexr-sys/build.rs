fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    pkg_config::Config::new()
        .atleast_version("3.0.0")
        .probe("openexr")
        .unwrap();
}