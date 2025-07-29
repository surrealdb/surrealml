from transformers import AutoModelForSequenceClassification
from safetensors.torch import save_file
import torch, json

repo = "textattack/bert-base-uncased-SST-2"      # any SST‑2 model you like
model = AutoModelForSequenceClassification.from_pretrained(repo, torch_dtype=torch.float32)

# ① save tensors
state = model.state_dict()
fn = "./sent_model.safetensors"
save_file({k: v.cpu() for k, v in state.items()}, fn)

# ② save config (keep only what Candle needs)
sub = {k: v for k, v in model.config.to_dict().items()
       if k in (
           "vocab_size", "hidden_size", "num_hidden_layers",
           "num_attention_heads", "intermediate_size",
           "hidden_dropout_prob", "attention_probs_dropout_prob",
           "max_position_embeddings", "type_vocab_size",
           "initializer_range", "layer_norm_eps", "hidden_act"
       )}
# add the label map for pretty printing
sub["id2label"] = model.config.id2label
with open("./sent_config.json", "w") as f:
    json.dump(sub, f, indent=2)