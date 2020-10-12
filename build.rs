extern crate pkg_config;

fn main() {
    pkg_config::probe_library("libsystemd").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
