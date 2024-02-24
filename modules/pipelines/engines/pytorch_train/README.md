
# Torch Engine 

Seeing as the support for training using torch is best matched using python we are going to use python for the training
engine. Do not worry this does not mean that the production system will be relying on python. The ONNX inference will be
handled using pure Rust.

## Assets

To test and develop the developer needs assets. An asset is an object that can either be a trained model, fake data etc
that we need to check to see if the system runs in an isolated way. Assets are thein the `assets` directory. However, 
the `assets` directory is in the `.gitignore` so you will have to run the following script to set everything up. Do not have reference to a `venv` when running the script as the script connects to the `venv` and installs the necessary packages.

```bash
sh scripts/build_assets.sh
``````


