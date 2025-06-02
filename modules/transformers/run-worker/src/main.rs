// worker/src/main.rs
use anyhow::{bail, Context, Result};
use wasmtime::{Engine, Module, Config, Store, Linker, TypedFunc, Memory};
use wasmtime_wasi::preview1::{add_to_linker_async, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;
use tokio; 
use std::{fs, path::Path};


/// WebAssembly page size
const WASM_PAGE: usize = 64 * 1024;


#[tokio::main]
async fn main() -> Result<()> {
    // Setup 1 Read the compiled .wasm
    let wasm_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("target/wasm32-wasip1/release/wasm_wrapper.wasm");
    let wasm = fs::read(&wasm_path)
        .with_context(|| format!("failed to read `{}`", wasm_path.display()))?;

    // Setup 2 - Engine + enable async + WASI linker
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let module = Module::new(&engine, &wasm)?;

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    add_to_linker_async(&mut linker, |ctx| ctx)?;
    let pre = linker.instantiate_pre(&module)?;

    // build a WASI context (inherit stdio + env)
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();
    let mut store = Store::new(&engine, wasi);

    // actually instantiate the module _asynchronously_
    let instance = pre.instantiate_async(&mut store).await?;

    // Setup 3 - Grab `memory` and initialize bump pointer
    let memory: Memory = instance
        .get_memory(&mut store, "memory")
        .context("export `memory` not found")?;
    let mut pages = memory.size(&store) as usize;
    let mut heap_top = pages * WASM_PAGE;

    // Helpers - write to linear wasm memory
    fn reserve(
        store: &mut Store<WasiP1Ctx>,
        memory: &Memory,
        pages: &mut usize,
        heap_top: usize,
        extra: usize,
    ) -> Result<()> {
        let used = heap_top;
        let avail = (*pages * WASM_PAGE).saturating_sub(used);
        if avail < extra {
            let need = (extra - avail + WASM_PAGE - 1) / WASM_PAGE;
            memory.grow(store, need as u64)
                .context("memory.grow failed")?;
            *pages += need;
        }
        Ok(())
    }

    fn alloc_cstr(
        store: &mut Store<WasiP1Ctx>,
        memory: &Memory,
        pages: &mut usize,
        heap_top: &mut usize,
        s: &str,
    ) -> Result<i32> {
        let mut buf = s.as_bytes().to_vec();
        buf.push(0);
        let len = buf.len();
        reserve(store, memory, pages, *heap_top, len)?;
        let ptr = *heap_top as i32;
        memory.write(store, ptr as usize, &buf)
            .context("writing C string")?;
        *heap_top += len;
        Ok(ptr)
    }

    fn alloc_raw(
        store: &mut Store<WasiP1Ctx>,
        memory: &Memory,
        pages: &mut usize,
        heap_top: &mut usize,
        size: usize,
    ) -> Result<i32> {
        // identical to alloc_cstr but without the trailing zero
        reserve(store, memory, pages, *heap_top, size)?;
        let ptr = *heap_top as i32;
        *heap_top += size;
        Ok(ptr)
    }

    fn read_cstr(
        store: &mut Store<WasiP1Ctx>,
        memory: &Memory,
        ptr: i32,
    ) -> Result<String> {
        let mut buf = Vec::new();
        let mut off = ptr as usize;
        loop {
            let mut byte = [0u8];
            // re-borrow the store here:
            memory
                .read(&mut *store, off, &mut byte)
                .context("reading C string")?;
            if byte[0] == 0 {
                break;
            }
            buf.push(byte[0]);
            off += 1;
        }
        Ok(String::from_utf8(buf)?)
    }

    // FFI function bindings with correct sret-aware signatures:
    let load_tok: TypedFunc<(i32, i32, i32), ()> =
        instance.get_typed_func(&mut store, "load_tokenizer")?;
    let free_tok: TypedFunc<(i32,), ()> =
        instance.get_typed_func(&mut store, "free_tokenizer_return")?;

    let encode: TypedFunc<(i32, i32, i32), ()> =
        instance.get_typed_func(&mut store, "encode")?;
    let free_vec: TypedFunc<(i32,), ()> =
        instance.get_typed_func(&mut store, "free_vec_u32_return")?;

    let decode: TypedFunc<(i32, i32, i32, i32), ()> =
        instance.get_typed_func(&mut store, "decode")?;
    let free_str: TypedFunc<(i32,), ()> =
        instance.get_typed_func(&mut store, "free_string_return")?;

    // load_tokenizer("gpt2", NULL)
    let model_ptr = alloc_cstr(&mut store, &memory, &mut pages, &mut heap_top, "gpt2")?;
    let sret = alloc_raw(&mut store, &memory, &mut pages, &mut heap_top, 3 * 4)?;
    load_tok.call_async(&mut store, (sret, model_ptr, 0)).await?;

    // unpack the TokenizerReturn from sret
    let mut buf = [0u8; 12];
    memory.read(&mut store, sret as usize, &mut buf)?;
    let handle  = i32::from_le_bytes(buf[0..4].try_into().unwrap());
    let is_err  = i32::from_le_bytes(buf[4..8].try_into().unwrap());
    let err_ptr = i32::from_le_bytes(buf[8..12].try_into().unwrap());

    if is_err != 0 {
        let msg = read_cstr(&mut store, &memory, err_ptr)?;
        free_tok.call_async(&mut store, (sret,)).await?;
        bail!("load_tokenizer failed: {}", msg);
    }
    println!("✅ tokenizer handle = 0x{:x}", handle);

    // encode(handle, "hello wasm!")
    let text_ptr = alloc_cstr(&mut store, &memory, &mut pages, &mut heap_top, "hello wasm!")?;
    let enc_sret = alloc_raw(&mut store, &memory, &mut pages, &mut heap_top, 5 * 4)?;
    encode.call_async(&mut store, (enc_sret, handle, text_ptr)).await?;

    // unpack the VecU32Return from enc_sret
    let mut buf = vec![0u8; 20];
    memory.read(&mut store, enc_sret as usize, &mut buf)?;
    let data_ptr    = i32::from_le_bytes(buf[0..4].try_into().unwrap());
    let length      = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
    let _capacity    = u32::from_le_bytes(buf[8..12].try_into().unwrap()) as usize;
    let enc_err     = i32::from_le_bytes(buf[12..16].try_into().unwrap());
    let enc_err_ptr = i32::from_le_bytes(buf[16..20].try_into().unwrap());

    if enc_err != 0 {
        let msg = read_cstr(&mut store, &memory, enc_err_ptr)?;
        free_vec.call_async(&mut store, (enc_sret,)).await?;
        bail!("encode failed: {}", msg);
    }

    // read u32 array
    let mut raw = vec![0u8; length * 4];
    memory.read(&mut store, data_ptr as usize, &mut raw)?;
    let ids: Vec<u32> = raw
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    println!("✅ encode → {:?}", ids);

    // decode(handle, data_ptr, length)
    let dec_sret = alloc_raw(&mut store, &memory, &mut pages, &mut heap_top, 3 * 4)?;
    let length_i32 = i32::try_from(length).expect("length overflow when casting to i32");
    decode.call_async(&mut store, (dec_sret, handle, data_ptr, length_i32)).await?;

    // unpack the StringReturn from dec_sret
    let mut buf = [0u8; 12];
    memory.read(&mut store, dec_sret as usize, &mut buf)?;
    let s_ptr       = i32::from_le_bytes(buf[0..4].try_into().unwrap());
    let dec_err     = i32::from_le_bytes(buf[4..8].try_into().unwrap());
    let dec_err_ptr = i32::from_le_bytes(buf[8..12].try_into().unwrap());

    if dec_err != 0 {
        let msg = read_cstr(&mut store, &memory, dec_err_ptr)?;
        free_str.call_async(&mut store, (dec_sret,)).await?;
        bail!("decode failed: {}", msg);
    }
    let roundtrip = read_cstr(&mut store, &memory, s_ptr)?;
    println!("✅ decode → \"{}\"", roundtrip);

    // final cleanup
    free_str.call_async(&mut store, (dec_sret,)).await?;
    free_vec.call_async(&mut store, (enc_sret,)).await?;
    free_tok.call_async(&mut store, (sret,)).await?;


    Ok(())
}
