# Basic Runner

This section is a basic runner that loads a jpg image and a label. It then reshapes the image and flatterns the
image to a one dimensional array. The one-dimensional array is mapped exactly to the same way numpy arrays are.
Once the image is reshaped and flatterned, the array is then piped over to a python listener using binary format
and the python listener will then convert it to python ints. This is the basis of how we will be sending data
to the python ML trainer.

# Why Bother? Shouldn't it all be in Python?

We are going to use Rust and ONNX to infer the models once they are trained because Rust is fast and low memory/energy
usage. We are using `python` to train the model due to the skillset available for training models in python. Python
also has a lot of support for GPU and tensor operations when training. However, we have fine grain control over how the
`jgp` data is loaded and reshaped in `rust`. We also want to ensure that the data is loaded and reshaped in the same way
in training as it will be in production inference. The `rust` code loading the images and passing them into the training
model will also be hooked up to networking interfaces and directly inferring.

# Running the Runner

The runner loads a single jpg image, processes, and pipes it into the python listener. To manually do such a thing you can
run the following command:

```bash
./basic_training_runner | python listener.py
```

We are streaming to reduce the amount of memory used. For instance, if you have a `60GB` folder of images, you are unlikely
to be able to load all of that into memory at once. Instead we can stream batches. Depending on the size of the RAM, you can
increase or decrease the batch size. To setup the environment and run it, you can run the following script:

```bash
sh ./scripts/run.sh
```

This will build the rust binary, create an isolated folder, and run the binary, python file, with the image in that folder.

# What's Next?

We will have to add commands such as batch size, image reshaping size, pointers to files, training params, and a ML model
selection to train.
