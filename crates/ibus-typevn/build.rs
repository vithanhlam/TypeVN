fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("1.5")
        .probe("ibus-1.0")
        .expect("ibus-1.0 development files are required (apt install libibus-1.0-dev)");

    let mut build = cc::Build::new();
    build.file("c/main.c").file("c/engine.c");
    build.include("c");
    for p in &lib.include_paths {
        build.include(p);
    }
    for (k, v) in &lib.defines {
        match v {
            Some(val) => {
                build.define(k, Some(val.as_str()));
            }
            None => {
                build.define(k, None);
            }
        }
    }
    build.compile("ibus_typevn_c");

    for p in &lib.link_paths {
        println!("cargo:rustc-link-search=native={}", p.display());
    }
    for libn in &lib.libs {
        println!("cargo:rustc-link-lib={libn}");
    }
    println!("cargo:rerun-if-changed=c/main.c");
    println!("cargo:rerun-if-changed=c/engine.c");
    println!("cargo:rerun-if-changed=c/engine.h");
}
