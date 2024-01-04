use std::path::Path;
use std::env;
use std::fs;


fn main() -> std::io::Result<()> {

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
            let profile = match env::var("PROFILE").unwrap() {
                ref s if s.contains("release") => "release",
                ref s if s.contains("debug") => "debug",
                _ => panic!("Unsupported profile")
            };
            let lib_path = Path::new("target").join(profile).join(target_lib);
            // put it next to the file of the embedding
            let destination = Path::new(target_lib);
            fs::copy(lib_path, destination)?;
        }

    }
    Ok(())
}
