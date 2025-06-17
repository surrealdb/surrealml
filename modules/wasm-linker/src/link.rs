//! Here is where we link ML modules with the wasmtime runtime. 
//! 
//! # Engine + Config
//! Global compilation configurations such as CPU features, caching, pooling, debug info
//! One engine can compile many modules and create many stores. It is not a per-process inside the guest
//! 
//! # Store
//! per instance execution sandbox: keeps the guest's memories, tables, and host "context" `WasiCtx`.
//! exists as long as you keep the store alive
//! 
//! # WasiCtx
//! built by WasiCtxBuilder this is the guest's visable "process environment"
//! lives inside a `Store`. Immutable once built, but sticks around for every call into the instance until
//! you drop the store. 
//! 
//! ## Inside the context
//! - standard streams => `stdin`, `stdout`, `stderr`
//! 

use wasmtime::{Result, Engine, Linker, Module, Store, Config};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::WasiCtxBuilder;


// maybe put in an interface via a trait so we don't need the wasmtime runtimes for the module


// pass in a config
// pass in an engine
// create a new module
// 
