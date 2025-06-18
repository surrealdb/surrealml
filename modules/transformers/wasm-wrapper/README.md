# wasm-wrapper

A minimal **WebAssembly Component Model** wrapper around the `tokenizer` Rust crate.

## Prerequisites

Install the WebAssembly target and the component build tool:

```bash
rustup target add wasm32-wasip1 # Install the wasm32-wasi target
```

## Build

From the workspace root, run - 

```bash
cargo build --release --target wasm32-wasip1 --package wasm-wrapper # Builds just the wasm-wrapper
```

This produces the below output file `target/wasm32-wasip1/release/wasm-wrapper.wasm`.


## Improvements

Because of sret, in our functions which take in args and return a struct, on our host side we have to call
these functions with an extra param of the memory in which we're to write the struct to. This solution is good,
although we could possible define these functions to take in a pointer to the memory to be allocated from the getgo.
This would mean the functions wouldn't return a struct, they'd just write it to that pointer. 

We could also return pointers to return structs, not the actual struct. This might make freeing cleaner too.

We definitely will want to provide a set of host interop functions too allow the host to allocate/deallocate memory in 
wasm linear memory. This will make calling our wasm functions much easier.
