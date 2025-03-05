const fs = require("fs");
const path = require("path");
const os = require("os");
const { execSync } = require("child_process");
const axios = require("axios");
const tar = require("tar");
const unzipper = require("unzipper");

const ONNX_VERSION = "1.20.0";
const DYNAMIC_LIB_VERSION = "1.0.0";

// Detect OS and architecture
const PLATFORM = os.platform(); // 'linux', 'darwin', 'win32'
const ARCH = os.arch(); // 'x64', 'arm64'
const IS_WINDOWS = PLATFORM === "win32";

// Set library names based on OS
const getLibName = () => {
  if (PLATFORM === "linux") return "libc_wrapper.so";
  if (PLATFORM === "darwin") return "libc_wrapper.dylib";
  if (PLATFORM === "win32") return "libc_wrapper.dll";
  throw new Error(`Unsupported platform: ${PLATFORM}`);
};

// Define paths
const ROOT_DEP_DIR = path.join(os.homedir(), "surrealml_deps");
const ONNX_LIB_DIR = path.join(ROOT_DEP_DIR, "onnxruntime", ONNX_VERSION);
const DYNAMIC_LIB_DIR = path.join(ROOT_DEP_DIR, "core_ml_lib", DYNAMIC_LIB_VERSION);
const ONNX_DOWNLOAD_CACHE = path.join(ONNX_LIB_DIR, "download_cache.tgz");
const BINARY_PATH = path.join(__dirname, "..", "modules", "c-wrapper", "target", "release", getLibName());
const DYNAMIC_LIB_DIST = path.join(DYNAMIC_LIB_DIR, getLibName());

const ONNX_LIB_NAME = IS_WINDOWS ? "onnxruntime.dll" : PLATFORM === "darwin" ? "libonnxruntime.dylib" : "libonnxruntime.so";
const ONNX_LIB_DIST = path.join(ONNX_LIB_DIR, ONNX_LIB_NAME);

const getOnnxRuntimeUrl = () => {
  const baseUrl = `https://github.com/microsoft/onnxruntime/releases/download/v${ONNX_VERSION}/`;
  if (PLATFORM === "linux") {
    return ARCH === "x64" ? `${baseUrl}onnxruntime-linux-x64-${ONNX_VERSION}.tgz` : `${baseUrl}onnxruntime-linux-aarch64-${ONNX_VERSION}.tgz`;
  }
  if (PLATFORM === "darwin") {
    return ARCH === "x64" ? `${baseUrl}onnxruntime-osx-x86_64-${ONNX_VERSION}.tgz` : `${baseUrl}onnxruntime-osx-arm64-${ONNX_VERSION}.tgz`;
  }
  if (PLATFORM === "win32") {
    return `${baseUrl}onnxruntime-win-x64-${ONNX_VERSION}.zip`;
  }
  throw new Error(`Unsupported platform or architecture: ${PLATFORM}`);
};

const downloadAndExtractOnnxRuntime = async () => {
  const url = getOnnxRuntimeUrl();
  console.log(`Downloading ONNX Runtime from ${url}`);

  if (!fs.existsSync(ONNX_DOWNLOAD_CACHE)) {
    const response = await axios({ url, responseType: "stream" });
    const writer = fs.createWriteStream(ONNX_DOWNLOAD_CACHE);
    response.data.pipe(writer);
    await new Promise((resolve, reject) => {
      writer.on("finish", resolve);
      writer.on("error", reject);
    });
  } else {
    console.log(`ONNX Runtime already downloaded at ${ONNX_DOWNLOAD_CACHE}`);
  }

  console.log("Extracting ONNX Runtime...");
  if (url.endsWith(".tgz")) {
    await tar.x({ file: ONNX_DOWNLOAD_CACHE, cwd: ONNX_LIB_DIR });
  } else if (url.endsWith(".zip")) {
    await fs.createReadStream(ONNX_DOWNLOAD_CACHE).pipe(unzipper.Extract({ path: ONNX_LIB_DIR })).promise();
  }

  fs.unlinkSync(ONNX_DOWNLOAD_CACHE);
};

const buildOrDownloadCoreLib = () => {
  if (!fs.existsSync(DYNAMIC_LIB_DIST) && process.env.LOCAL_BUILD === "TRUE") {
    console.log("Building core ML lib locally...");
    execSync("cargo build --release", { cwd: path.join(__dirname, "..", "modules", "c-wrapper"), stdio: "inherit" });
    fs.copyFileSync(BINARY_PATH, DYNAMIC_LIB_DIST);
  } else {
    console.log("Core ML lib already present or downloading...");
  }
};

(async () => {
  try {
    // Ensure directories exist
    fs.mkdirSync(ROOT_DEP_DIR, { recursive: true });
    fs.mkdirSync(ONNX_LIB_DIR, { recursive: true });
    fs.mkdirSync(DYNAMIC_LIB_DIR, { recursive: true });

    // Download ONNX Runtime if needed
    if (!fs.existsSync(ONNX_LIB_DIST)) {
      await downloadAndExtractOnnxRuntime();
    } else {
      console.log("ONNX Runtime already installed.");
    }

    // Build or download the Rust library
    buildOrDownloadCoreLib();

    console.log("Installation complete!");
  } catch (error) {
    console.error("Error during post-install script:", error);
    process.exit(1);
  }
})();
