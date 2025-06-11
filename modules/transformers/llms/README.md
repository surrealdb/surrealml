# SurrealML LLMs

This README will be formalised later, but for now a running list of future improvements or notes - 
- We use 2 features, 1 for functionality and testing, one just for testing.
- When pulling from hugging face hub we could do a lazy static API, and we could do a custom cache path.
- We aren't supporting CUDA out of the both. This means for each model, the flash_attn boolean is defaulted to false - both when creating the model config and when creating the actual model object. We don't use the CUDA feature in our candle-transformers crate either. We also choose device type CPU both when creating the VarBuilder object and when running the model.
- We only test the model loading methods in each model file with the gemma feature flag for just Gemma. We can look into doing more expansive tests later.
- Store Gemma config.jsons in the binary to support different ones. Right now that's hardcoded.
- Remove need for enum with boxed dyn trait calling?
- Refactor running the models with traits to make dependency injection.
- Remove state, and hold loaded on the struct itself

# Test Notes
- The first thing to look at is ordering on gemma unittest feature flags - fetching unittest running before our other unittests run which depend on the data being already fetched. 
- So we have a gemma test case on the tensor utils file to actually load a VarBuilder. This is hard to get around.
- Then on our model structs/enum, for now we just have a full test for Gemma. Again, it's a feature flag gated unittest which expects our gemma tensor files to be previously loaded. But we're again doing a full test to simulate loading aswell as building the VarBuilder object. Again hard to get around.
- Then our model wrapper state file uses dependency injection via a fake trait implementation to make this much easier.
- Then in our interface we simulate the whole thing and provide logic for fetching via hf vs locally.
- Possibly we can use traits further to reduce the amount of full unittests we're running.
