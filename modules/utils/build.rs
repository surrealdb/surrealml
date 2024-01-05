use std::path::Path;
use std::env;
use std::fs;


fn main() -> std::io::Result<()> {

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
