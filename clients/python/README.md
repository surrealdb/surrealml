
# SurrealML Python Client

The SurrealML Python client using the Rust `surrealml` library without any `PyO3` bindings. 

The SurrealML client relies on the dynamic C lib of the `surrealml-core` package and the `libonnxruntime`. To handle this the installer downloads the correct `libonnxruntime` for the right operating system and dynamic C lib and stores them in the users root folder under the following structure:

```
└── surrealml_deps
    ├── core_ml_lib
    │   └── 1.0.0
    │       └── libc_wrapper.dylib
    └── onnxruntime
        └── 1.20.0
            └── libonnxruntime.dylib
```

If we keep this structure, we can also build other clients written in languages like `JavaScript` or `Ruby` that can also link to the same dependenices in the same place. The versions of the `surrealml-core` package and `libonnxruntime` are denoted as directories to avoid clashes. This means one application on the machine can be running version `1.0.0` with a `libonnxruntime` of `1.20.0` and another application can be pointing to a later version without clashes on the machine. Having this central storage to dependencies means that we do not have to package binaries for multiple different language package managers and we also do not have to keep downloading these binaries and libraries for different languages and applications. They can all dynamically load one set of libraries. This can also improve the interoperability between languages in the future.

# Installation

Currently `surrealml` is not on [PyPi](https://pypi.org/user/tobiemh/) and relies on a dynamic C lib written in Rust. We are working on cross compilation of library binaries that can be downloaded but for now, you will need to install [rust](https://www.rust-lang.org/tools/install) to compile the binary lib. To install the `surrealml` package you need to set the `LOCAL_BUILD` environment variable to `TRUE` with the following command:

```bash
export LOCAL_BUILD="TRUE"
```

We can then install the package with the command below:

```bash
pip install git+https://github.com/surrealdb/surrealml.git@refactor#subdirectory=clients/python
```

This can take some time as the `surrealml` binary lib is being compiled in the background. You will then be able to use `surrealml` in python.