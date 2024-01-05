use std::path::Path;
use std::env;
use std::fs;


/// works out where the `onnxruntime` library is in the build target and copies the library to the root
/// of the crate so the core library can find it and load it into the binary using `include_bytes!()`.
/// 
/// # Notes
/// This is a workaround for the fact that `onnxruntime` doesn't support `cargo` yet. This build step
/// is reliant on the `ort` crate downloading and building the `onnxruntime` library. This is
/// why the following dependency is required in `Cargo.toml`:
/// ```toml
/// [build-dependencies]
/// ort = { version = "1.16.2", default-features = true }
/// ```
/// Here we can see that the `default-features` is set to `true`. This is because the `ort` crate will download
/// the correct package and build it for the target platform by default. In the main part of our dependencies
/// we have the following:
/// ```toml
/// [dependencies]
/// ort = { version = "1.16.2", features = ["load-dynamic"], default-features = false }
/// ```
/// Here we can see that the `default-features` is set to `false`. This is because we don't want the `ort` crate
/// to download and build the `onnxruntime` library again. Instead we want to use the one that was built in the
/// build step. We also set the `load-dynamic` feature to `true` so that the `ort` crate will load the `onnxruntime`
/// library dynamically at runtime. This is because we don't want to statically link the `onnxruntime`. Our `onnxruntime`
/// is embedded into the binary using `include_bytes!()` and we want to load it dynamically at runtime. This means that
/// we do not need to move the `onnxruntime` library around with the binary, and there is no complicated setup required
/// or linking.
fn unpack_onnx() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir);
    let build_dir = out_path
        .ancestors()  // This gives an iterator over all ancestors of the path
        .nth(3)       // 'nth(3)' gets the fourth ancestor (counting from 0), which should be the debug directory
        .expect("Failed to find debug directory");

    match std::env::var("ONNXRUNTIME_LIB_PATH") {
        Ok(_) => {
            println!("cargo:rustc-cfg=onnx_runtime_env_var_set");
        },
        Err(_) => {
            let target_lib = match env::var("CARGO_CFG_TARGET_OS").unwrap() {
                ref s if s.contains("linux") => "libonnxruntime.so",
                ref s if s.contains("macos") => "libonnxruntime.dylib",
                ref s if s.contains("windows") => "onnxruntime.dll",
                // ref s if s.contains("android") => "android", => not building for android
                _ => panic!("Unsupported target os")
            };

            let lib_path = build_dir.join(target_lib);
            let lib_path = lib_path.to_str().unwrap();

            // put it next to the file of the embedding
            let destination = Path::new(target_lib);
            fs::copy(lib_path, destination)?;
        }
    }
    Ok(())
}


fn main() -> std::io::Result<()> {

    if std::env::var("DOCS_RS").is_ok() {
        // we are not going to be anything here for docs.rs, because we are merely building the docs. When we are just building
        // the docs, the onnx environment variable will not look for the `onnxruntime` library, so we don't need to unpack it.
    } else {
        unpack_onnx()?;
    }
    Ok(())
}
