use std::env;
use std::path::Path;

fn main() {
	// if the ONNX library is statically linked we do not need to do anything
	if cfg!(onnx_statically_linked) {
		return;
	}

	let target_lib = match env::var("CARGO_CFG_TARGET_OS").unwrap() {
		ref s if s.contains("linux") => "libonnxruntime.so",
		ref s if s.contains("macos") => "libonnxruntime.dylib",
		ref s if s.contains("windows") => "onnxruntime.dll",
		// ref s if s.contains("android") => "android", => not building for android
		_ => panic!("Unsupported target os"),
	};
	let profile = match env::var("PROFILE").unwrap() {
		ref s if s.contains("release") => "release",
		ref s if s.contains("debug") => "debug",
		_ => panic!("Unsupported profile"),
	};

	// remove ./modules/utils/target folder if there
	let _ =
		std::fs::remove_dir_all(Path::new("modules").join("core").join("target")).unwrap_or(());

	// create the target module folder for the utils module
	let _ = std::fs::create_dir(Path::new("modules").join("core").join("target"));
	let _ = std::fs::create_dir(Path::new("modules").join("core").join("target").join(profile));

	// copy target folder to modules/utils/target profile for the utils modules
	let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".into());
	std::fs::copy(
		Path::new(target_dir.as_str()).join(profile).join(target_lib),
		Path::new("modules").join("core").join("target").join(profile).join(target_lib),
	)
	.unwrap();
}
