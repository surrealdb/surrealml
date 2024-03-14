# Batch Runner

This is where the image loading is still written in Rust, but there a python bindings meaning that the ML engineer
can train the model in pure python importing and using the Rust code as a python library. This is done my merely
pointing to the `data_access` library and performing a pip install. 

This runner is specifically used to test if the model can handle batching. It uses both batches and epochs to train the model.

# Running the runner

We can run the runner by running the following command:

```bash
sh ./scripts/run.sh
```

This will import the Rust image loading code which will then load the image giving us the raw resized and flatterned
image data in python. This can be directly fed into the ML model for training.
