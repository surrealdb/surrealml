use std::process::Command;

fn main() {

    let _ = Command::new("sh")
        .arg("-c")
        .arg("cargo new onnx_driver && cd onnx_driver && echo 'ort = \"1.16.2\"' >> Cargo.toml
        ")
        .status()
        .expect("failed to execute process");

    let _ = Command::new("sh")
        .arg("-c")
        .arg("cd onnx_driver && cargo build")
        .status()
        .expect("failed to execute process");

    // let _ = Command::new("sh")
    //     .arg("-c")
    //     .arg("cd ../onnx_driver && cargo build")
    //     .status()
    //     .expect("failed to execute process");

}
