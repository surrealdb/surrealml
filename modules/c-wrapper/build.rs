use std::env;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use reqwest::blocking::get;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    let version = "1.20.1";
    let root_dir_str = env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&root_dir_str);
    let current_working_dir = std::env::current_dir().unwrap();
    let out_dir = std::env::current_dir().unwrap().join("onnx_lib");

    // Create output directory
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // Detect OS and architecture
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Failed to get target OS");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Failed to get target architecture");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default(); // Optional: For specific environments like MSVC

    // Map to appropriate URL
    let file_extension = match target_os.as_str() {
        "windows" => "zip",
        _ => "tgz",
    };

    // Construct the directory name
    // linux aarch64
    let directory_name = match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "aarch64") => format!("onnxruntime-linux-aarch64-{version}"),
        ("linux", "x86_64") => format!("onnxruntime-linux-x64-{version}"),
        ("macos", "aarch64") => format!("onnxruntime-osx-arm64-{version}"),
        ("macos", "x86_64") => format!("onnxruntime-osx-x86_64-{version}"),
        ("windows", "aarch64") => format!("onnxruntime-win-arm64-{version}"),
        ("windows", "x86_64") => format!("onnxruntime-win-x64-{version}"),
        ("windows", "x86") => format!("onnxruntime-win-x86-{version}"),
        _ => panic!("Unsupported OS/architecture combination"),
    };
    println!("build directory defined: {}", directory_name);

    let filename = match (target_os.as_str(), target_arch.as_str(), target_env.as_str()) {
        ("linux", "aarch64", _) => format!("onnxruntime-linux-aarch64-{version}.{file_extension}"),
        ("linux", "x86_64", _) => {
            if cfg!(feature = "gpu") {
                format!("onnxruntime-linux-x64-gpu-{version}.{file_extension}")
            } else {
                format!("onnxruntime-linux-x64-{version}.{file_extension}")
            }
        }
        ("macos", "aarch64", _) => format!("onnxruntime-osx-arm64-{version}.{file_extension}"),
        ("macos", "x86_64", _) => format!("onnxruntime-osx-x86_64-{version}.{file_extension}"),
        ("windows", "x86_64", _) => {
            if cfg!(feature = "gpu") {
                format!("onnxruntime-win-x64-gpu-{version}.{file_extension}")
            } else {
                format!("onnxruntime-win-x64-{version}.{file_extension}")
            }
        }
        ("windows", "x86", _) => format!("onnxruntime-win-x86-{version}.{file_extension}"),
        ("windows", "aarch64", _) => format!("onnxruntime-win-arm64-{version}.{file_extension}"),
        _ => panic!("Unsupported OS/architecture combination"),
    };
    println!("build filename defined: {}", filename);

    let url = format!(
        "https://github.com/microsoft/onnxruntime/releases/download/v{version}/{filename}"
    );

    // Download and extract
    println!("Downloading ONNX Runtime from {}", url);
    let response = get(&url).expect("Failed to send request");
    if !response.status().is_success() {
        panic!("Failed to download ONNX Runtime: HTTP {}", response.status());
    }
    println!("Downloaded ONNX Runtime successfully");

    if file_extension == "tgz" {
        let tar_gz = GzDecoder::new(Cursor::new(response.bytes().expect("Failed to read response bytes")));
        let mut archive = Archive::new(tar_gz);
        archive.unpack(&out_dir).expect("Failed to extract archive");
    } else if file_extension == "zip" {
        let mut archive = zip::ZipArchive::new(Cursor::new(
            response.bytes().expect("Failed to read response bytes"),
        ))
        .expect("Failed to open ZIP archive");
        archive.extract(&out_dir).expect("Failed to extract ZIP archive");
    }
    println!("Extracted ONNX Runtime successfully");

    let lib_filename = match target_os.as_str() {
        "windows" => "onnxruntime.dll",
        "macos" => "libonnxruntime.dylib",
        _ => "libonnxruntime.so",
    };
    println!("lib filename defined: {}", lib_filename);

    let output_dir = Path::new(&out_dir);
    let lib_path = output_dir.join(directory_name.clone()).join("lib").join(lib_filename);

    // copy the library to the output directory
    fs::copy(&lib_path, Path::new(&out_dir).join("onnxruntime")).expect("Failed to copy library");

    let path_data = format!("Copied library to output directory {} -> {}", lib_path.display(), out_dir.display());

    let mut file = File::create(current_working_dir.join("build_output.txt")).unwrap();
    file.write_all(path_data.as_bytes()).unwrap();

    println!("{}", path_data);
    // remove the out_dir
    fs::remove_dir_all(&output_dir.join(directory_name)).expect("Failed to remove output directory");

    let output_lib = Path::new(&out_dir).join("onnxruntime");

    // link the library
    println!("cargo:rustc-env=ORT_LIB_LOCATION={}", output_lib.display());
}
