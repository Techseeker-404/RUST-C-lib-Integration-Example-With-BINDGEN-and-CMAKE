use std::env;
use std::path::PathBuf;
use bindgen;
use cmake::Config;

fn main() {
    let dst = Config::new("libsimplemath").build();
    println!("cargo:rustc-link-lib=dylib=simplemath");
    println!("cargo:rustc-link-search=native={}", dst.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
