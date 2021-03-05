fn main() {
    let config = system_deps::Config::new().probe().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=haxx.c");

    cc::Build::new()
        .file("src/native-shims.c")
        .includes(config.all_include_paths())
        .warnings(false)
        .compile("native-shims");
}
