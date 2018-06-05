extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Hacks
    println!("cargo:rustc-link-lib=static=numa");

    println!("cargo:rustc-link-lib=static=spdk_env_dpdk");

    // SPDK_LIB_FILES:
    println!("cargo:rustc-link-lib=static=spdk_event_bdev");
    println!("cargo:rustc-link-lib=static=spdk_event_copy");
    println!("cargo:rustc-link-lib=static=spdk_blobfs");
    println!("cargo:rustc-link-lib=static=spdk_blob");
    println!("cargo:rustc-link-lib=static=spdk_bdev");
    println!("cargo:rustc-link-lib=static=spdk_blob_bdev");
    println!("cargo:rustc-link-lib=static=spdk_copy");
    println!("cargo:rustc-link-lib=static=spdk_event");
    println!("cargo:rustc-link-lib=static=spdk_util");
    println!("cargo:rustc-link-lib=static=spdk_conf");
    println!("cargo:rustc-link-lib=static=spdk_trace");
    println!("cargo:rustc-link-lib=static=spdk_log");
    println!("cargo:rustc-link-lib=static=spdk_jsonrpc");
    println!("cargo:rustc-link-lib=static=spdk_json");
    println!("cargo:rustc-link-lib=static=spdk_rpc");

    // BLOCKDEV_MODULES_FILES:
    println!("cargo:rustc-link-lib=static=spdk_vbdev_lvol");
    println!("cargo:rustc-link-lib=static=spdk_blob");
    println!("cargo:rustc-link-lib=static=spdk_blob_bdev");
    println!("cargo:rustc-link-lib=static=spdk_lvol");
    println!("cargo:rustc-link-lib=static=spdk_bdev_malloc");
    println!("cargo:rustc-link-lib=static=spdk_bdev_null");
    println!("cargo:rustc-link-lib=static=spdk_bdev_nvme");
    println!("cargo:rustc-link-lib=static=spdk_nvme");
    println!("cargo:rustc-link-lib=static=spdk_vbdev_passthru");
    println!("cargo:rustc-link-lib=static=spdk_vbdev_error");
    println!("cargo:rustc-link-lib=static=spdk_vbdev_gpt");
    println!("cargo:rustc-link-lib=static=spdk_vbdev_split");
    println!("cargo:rustc-link-lib=static=spdk_bdev_aio");
    println!("cargo:rustc-link-lib=static=spdk_bdev_virtio");
    println!("cargo:rustc-link-lib=static=spdk_virtio");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    // See .cargo/config for the remaining libraries.

    // Don't rerun the whole thing every time
    println!("cargo:rerun-if-changed=./build.rs");

    generate("nvme");
    generate("event");
    generate("bdev");
    generate("env");
    generate("blob_bdev");
    generate("blob");
    generate("log");
}

fn generate(name: &str) {
    let mut codegen_config = bindgen::CodegenConfig::nothing();
    codegen_config.functions = true;
    codegen_config.types = true;

    let bindings = bindgen::Builder::default()
        .header(format!("/usr/local/include/spdk/{}.h", name))
        .derive_default(true)
        .with_codegen_config(codegen_config)
        .clang_arg("-I/tmp/spdk/include")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("spdk_{}_bindings.rs", name)))
        .expect("Couldn't write bindings!");
}