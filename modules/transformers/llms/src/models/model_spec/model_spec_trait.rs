use crate::utils::error::SurrealError;
use candle_transformers::models::mimi::candle_nn::VarBuilder;
use surrealml_tokenizers::Tokenizer;

/// Trait every preset model enum or struct implements.
///
/// * `type Cfg`- the type of the config for this model.
/// * `fn config(&self)` — returns a ready-to-use instance of that struct.
/// * `fn return_tensor_filenames(&self)` — returns a vector of tensor filenames associated with the model.
/// * `fn return_loaded_model(&self, vb)` — returns the loaded model object.
pub trait ModelSpec {
    type Cfg;
    type LoadedModel;

    fn config(&self) -> Self::Cfg;
    fn return_tensor_filenames(&self) -> Vec<String>;
    fn return_loaded_model(&self, vb: VarBuilder) -> Result<Self::LoadedModel, SurrealError>;
    fn run_model(
        &self,
        model: &mut Self::LoadedModel,
        input_ids: &[u32],
        max_steps: usize,
        tokenizer: &Tokenizer,
    ) -> Result<String, SurrealError>;
}
