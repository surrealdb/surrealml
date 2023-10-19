use std::process::Command;

fn main() {
    let _ = Command::new("sh")
        .arg("-c")
        .arg("cd ../onnx_driver && cargo clean")
        .status()
        .expect("failed to execute process");

    let _ = Command::new("sh")
        .arg("-c")
        .arg("cd ../onnx_driver && cargo build")
        .status()
        .expect("failed to execute process");

}
