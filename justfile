# Set shell for all commands
set shell := ["bash", "-cu"]

# Variables
DOCKER_IMAGE := "rust-python-shell"
WORKDIR := "/workspace"
PROJECT_DIR := "{{justfile_directory()}}"

# 📋 Default task: list available commands
default:
    @echo "Available tasks:"
    @echo "  just build-image     # Build the Docker image"
    @echo "  just shell           # Start a disposable shell"
    @echo "  just rust-test       # Run Rust tests"
    @echo "  just python-test     # Run Python tests"
    @echo "  just test-all        # Run both Rust and Python tests"

# 🔨 Build the Docker image
build-image:
    docker build -t {{DOCKER_IMAGE}} .

# 🐚 Open a disposable shell inside the container
shell:
    docker run --rm -it --tmpfs /tmp:exec,size=512m -v "{{justfile_directory()}}:/workspace" -w /workspace {{DOCKER_IMAGE}}

test-core:
    cd modules/core && cargo test && cargo test --features sklearn-tests && cargo test --features onnx-tests && cargo test --features torch-tests && cargo test --features tensorflow-tests

test-clib:
    bash modules/c-wrapper/scripts/build-docker.sh

# 🧪 Run Rust tests inside the container
rust-test:
    docker run --rm -v {{PROJECT_DIR}}:{{WORKDIR}} -w {{WORKDIR}} {{DOCKER_IMAGE}} bash -c "cargo test"

# 🧪 Run Python tests inside the container
python-test:
    docker run --rm -v {{PROJECT_DIR}}:{{WORKDIR}} -w {{WORKDIR}} {{DOCKER_IMAGE}} bash -c "pytest"

# ✅ Run both Rust and Python tests
test-all: rust-test python-test
