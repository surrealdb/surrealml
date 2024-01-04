//! This build file downloads the prebuilt binaries for ONNX Runtime and places them in the root of the crate
//! to be pointed at by the ort environment so we can run the ONNX models.
use ureq;
use std::path::Path;


static CACHE_FILE: &str = "./downloaded_onnx_package";


fn extract_tgz(buf: &[u8], output: &Path) {
	let buf: std::io::BufReader<&[u8]> = std::io::BufReader::new(buf);
	let tar = flate2::read::GzDecoder::new(buf);
	let mut archive = tar::Archive::new(tar);
	archive.unpack(output).expect("Failed to extract .tgz file");
}


fn hex_str_to_bytes(c: impl AsRef<[u8]>) -> Vec<u8> {
	fn nibble(c: u8) -> u8 {
		match c {
			b'A'..=b'F' => c - b'A' + 10,
			b'a'..=b'f' => c - b'a' + 10,
			b'0'..=b'9' => c - b'0',
			_ => panic!()
		}
	}

	c.as_ref().chunks(2).map(|n| nibble(n[0]) << 4 | nibble(n[1])).collect()
}


fn verify_file(buf: &[u8], hash: impl AsRef<[u8]>) -> bool {
	use sha2::Digest;
	sha2::Sha256::digest(buf)[..] == hex_str_to_bytes(hash)
}


/// Fetches a file from the given URL and returns it as a vector of bytes.
/// 
/// # Arguments
/// * `source_url` - The URL to fetch the file from.
/// 
/// # Returns
/// A vector of bytes containing the file.
fn fetch_file(source_url: &str) -> Vec<u8> {
	let resp = ureq::get(source_url)
		.timeout(std::time::Duration::from_secs(1800))
		.call()
		.unwrap_or_else(|err| panic!("Failed to GET `{source_url}`: {err}"));

	let len = resp
		.header("Content-Length")
		.and_then(|s| s.parse::<usize>().ok())
		.expect("Content-Length header should be present on archive response");
	let mut reader = resp.into_reader();
	let mut buffer = Vec::new();
	reader
		.read_to_end(&mut buffer)
		.unwrap_or_else(|err| panic!("Failed to download from `{source_url}`: {err}"));
	assert_eq!(buffer.len(), len);
	buffer
}



fn main() {

    match std::env::var("ONNXRUNTIME_LIB_PATH") {
        Ok(_) => {
            println!("cargo:rustc-cfg=onnx_runtime_env_var_set");
        },
        Err(_) => {
            let target = std::env::var("TARGET").unwrap();

            let (prebuilt_url, prebuilt_hash) = match target.as_str() {
                "aarch64-apple-darwin" => (
                    "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-aarch64-apple-darwin.tgz",
                    "188E07B9304CCC28877195ECD2177EF3EA6603A0B5B3497681A6C9E584721387"
                ),
                "aarch64-pc-windows-msvc" => (
                    "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-aarch64-pc-windows-msvc.tgz",
                    "B35F6526EAF61527531D6F73EBA19EF09D6B0886FB66C14E1B594EE70F447817"
                ),
                "aarch64-unknown-linux-gnu" => (
                    "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-aarch64-unknown-linux-gnu.tgz",
                    "C1E315515856D7880545058479020756BC5CE4C0BA07FB3DD2104233EC7C3C81"
                ),
                "wasm32-unknown-emscripten" => (
                    "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-wasm32-unknown-emscripten.tgz",
                    "468F74FB4C7451DC94EBABC080779CDFF0C7DA0617D85ADF21D5435A96F9D470"
                ),
                "x86_64-apple-darwin" => (
                    "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-x86_64-apple-darwin.tgz",
                    "0191C95D9E797BF77C723AD82DC078C6400834B55B8465FA5176BA984FFEAB08"
                ),
                "x86_64-pc-windows-msvc" => {
                    if cfg!(any(feature = "cuda", feature = "tensorrt")) {
                        (
                            "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_dylib_cuda-v1.16.3-x86_64-pc-windows-msvc.tgz",
                            "B0F08E93E580297C170F04933742D04813C9C3BAD3705E1100CA9EF464AE4011"
                        )
                    } else {
                        (
                            "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-x86_64-pc-windows-msvc.tgz",
                            "32ADC031C0EAA6C521680EEB9A8C39572C600A5B4F90AFE984590EA92B99E3BE"
                        )
                    }
                }
                "x86_64-unknown-linux-gnu" => {
                    if cfg!(any(feature = "cuda", feature = "tensorrt")) {
                        (
                            "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_dylib_cuda-v1.16.3-x86_64-unknown-linux-gnu.tgz",
                            "0F0651D10BA56A6EA613F10B60E5C4D892384416C4D76E1F618BE57D1270993F"
                        )
                    } else {
                        (
                            "https://parcel.pyke.io/v2/delivery/ortrs/packages/msort-binary/1.16.3/ortrs-msort_static-v1.16.3-x86_64-unknown-linux-gnu.tgz",
                            "D0E63AC1E5A56D0480009049183542E0BB1482CE29A1D110CC8550BEC5D994E2"
                        )
                    }
                }
                x => panic!("downloaded binaries not available for target {x}\nyou may have to compile ONNX Runtime from source")
            };

            // download the prebuilt binary that is compatible with the target
            let downloaded_file = fetch_file(prebuilt_url);

            // verify the hash of the downloaded file to ensure that we have downloaded the correct file
            assert!(verify_file(&downloaded_file, prebuilt_hash));

            // delete the ONNX cache file if it exists
            let _ = std::fs::remove_dir_all(CACHE_FILE);

            // extract the downloaded file to the cache file
            extract_tgz(&downloaded_file, Path::new(CACHE_FILE));

            // remove the downloaded bundled file
            let _ = std::fs::remove_file("./msort.tar.gz");  
        }
    }
}
