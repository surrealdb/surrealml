use std::process::Command;

fn main() {

    match std::env::var("ONNXRUNTIME_LIB_PATH") {
        Ok(_) => {
            println!("cargo:rustc-cfg=onnx_runtime_env_var_set");
        },
        Err(_) => {
            #[cfg(not(windows))]
            let _ = Command::new("sh")
                .arg("-c")
                .arg("cargo new onnx_driver && cd onnx_driver && echo 'ort = \"1.16.2\"' >> Cargo.toml
                ")
                .status()
                .expect("failed to execute process");

            #[cfg(windows)]
            {
                // let _ = Command::new("cmd")
                // .args(&["/C", "cargo new onnx_driver && cd onnx_driver && echo ort = \"1.16.2\" >> Cargo.toml"])
                // .status()
                // .expect("failed to execute process");
                let _ = Command::new("powershell")
                    .arg("-Command")
                    .arg("cargo new onnx_driver; Set-Location onnx_driver; Add-Content -Path .\\Cargo.toml -Value 'ort = \"1.16.2\"'")
                    .status()
                    .expect("failed to execute process");
            }

            #[cfg(not(windows))]
            {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg("cd onnx_driver && cargo build")
                    .status()
                    .expect("failed to execute process");
            }

            #[cfg(windows)]
            {
                let _ = Command::new("cmd")
                    .args(&["/C", "cd onnx_driver && cargo build"])
                    .status()
                    .expect("failed to execute process");
            }
        }
    }
}
