# SurrealML Tokenizers

This crate is an interface to hugging face tokenizer files. It allows us to easily pull tokenizer files from hugging facen and load them into **Tokenizer** objects.


## Why tokenizers? 

Large-language-model text must be converted to integer “tokens” before a model can
process it. A tokenizer defines that mapping (encode) and its reverse (decode),
and different models use different rules.


## What this crate does

* **Loads** a tokenizer by name:
  * *Preset* → compiled into the binary (`include_str!`).
  * Anything else → lazily pulled from the HF Hub and cached.
* **Encodes / decodes** text to `Vec<u32>` and back.
* Provides a **Bash helper script** to download and vendor a set of public tokenizers.
* Clean **error handling** with `thiserror`.
* **Feature-gated** integration tests that can hit the real network.


## Installation 🔧

Add to *Cargo.toml*:

```toml
[dependencies]
surrealml-tokenizers = "0.1"
```

The library is re-exported under `surrealml_transformers` (see `[lib]` in
*Cargo.toml*), so you `use surrealml_transformers::...` in code.


## Optional features

| Feature            | What it does                                                                           |
|--------------------|----------------------------------------------------------------------------------------|
| `tokio`            | Enables **async** downloads via `hf-hub/tokio`.                                        |
| `integration-net`  | Allows `cargo test` to hit the real HF Hub (useful for CI or smoke tests).             |

Enable a feature:

```toml
surrealml-tokenizers = { version = "0.1", features = ["tokio"] }
```


## Quick start

```rust
use surrealml_transformers::{load_tokenizer, encode, decode};

fn main() -> anyhow::Result<()> {
    // 1. Load a tokenizer (preset or HF repo id)
    let tok = load_tokenizer("gpt2".into(), None)?; // No HF access token as this is a public repo

    // 2. Encode some text
    let ids = encode(&tok, "Hello world!")?;
    println!("Token IDs: {ids:?}");

    // 3. Decode back
    let text = decode(&tok, &ids)?;
    println!("Round-trip: {text}");

    Ok(())
}
```


## Built-in vs remote tokenizers 

**Built in tokenizers**
| Preset enum              | `model` string to pass     | Embedded file                                        |
|--------------------------|----------------------------|------------------------------------------------------|
| `Gpt2`                   | `gpt2`                     | `tokenizers/gpt2-tokenizer.json`                     |
| `DistilGpt2`             | `distilgpt2`               | `tokenizers/distilgpt2-tokenizer.json`               |
| `GptNeo125M`             | `EleutherAI/gpt-neo-125M`  | `tokenizers/EleutherAI-gpt-neo-125M-tokenizer.json`  |
| `BertBaseUncased`        | `bert-base-uncased`        | `tokenizers/bert-base-uncased-tokenizer.json`        |

**Remote tokenizers**
Anything else (e.g. `"meta-llama/Meta-Llama-3-8B"`) triggers a download. So if we did the below, we'd pull the `tokenizer.json` file from hugging face and cache it in the standard HF directory `~/.cache/huggingface/hub.

```rust
let tok = load_tokenizer(
    "meta-llama/Meta-Llama-3-8B".to_string(),
    Some("hf_XXXXXXXXXXXXXXXX".to_string())   // Pass in HF token if gated model
)?;
```


## `scripts/tokenizer_download.sh` 

A convenience script that bulk-downloads the **public** presets to
`./tokenizers/` so they are bundled into the crate on the next build
(handy for offline environments).

```bash
# Pass token as arg …
./scripts/tokenizer_download.sh hf_XXXXXXXXXXXXXXXX

# …or via env var.
export HF_TOKEN=hf_XXXXXXXXXXXXXXXX
./scripts/tokenizer_download.sh
```

What it does:

1. Creates a `tokenizers/` folder next to the script.
2. Downloads `tokenizer.json` for each model in its `models=( … )` array.
3. Writes files like `gpt2-tokenizer.json`, `EleutherAI-gpt-neo-125M-tokenizer.json`, …


## Running tests

| Command                                             | What runs                                              |
|-----------------------------------------------------|--------------------------------------------------------|
| `cargo test`                                        | All **unit & offline** tests (only presets).           |
| `cargo test --features integration-net`             | Adds an **integration** test that fetches `gpt2` live. |


## Public API summary 

| Function / enum                                | Use-case                                                                    |
|------------------------------------------------|-----------------------------------------------------------------------------|
| `load_tokenizer(model, hf_token)`              | Load preset or remote tokenizer, returning `tokenizers::Tokenizer`.         |
| `encode(&tokenizer, text)`                     | Convert `&str` → `Vec<u32>` token IDs.                                      |
| `decode(&tokenizer, ids)`                      | Convert token IDs back to `String`.                                         |


## Future improvements

* Custom cache directory – allow users to override the HF cache path.
* Lazy global `Api` instance – share a single `hf_hub::Api` across calls.
* Trait-based hugging face hub fetcher – make network access swappable for easy mocking in tests.
* Add error macros to reduce repetitiveness.
* Add smoke test.
* Add WASM wrapper to enable this library to be called in WASM.


## License 

See the *LICENSE* file for details.
