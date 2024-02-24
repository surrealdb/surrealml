# SurgiSeek

Database for advanced surgical video searching - King's x SurrealDB

# Layout

Right now at the time of writing this, we are in the discovery phase so the layout of the documentation is fairly basic. However, as systems evolve, we will revisit the layout of the machine learning documentation here to introduce systems as they emerge. We aim to have an iterative process to avoid over-engineering. Right now, we are just having flat pages for the subjects. Hopefully we can keep concepts isolated with clean interfaces. Right now we have the following modules:

## data_access

This is where we define code that directly interacts with data either from a database or a file. Right now we do not know how the structure will
form so we just have the `data_access/basic` Rust crate. Here we can explore loading and storing file data. Once themes start to form we can
build out a more defined structure. We have example tags in the `data_access/data_stash` directory.

A big data set could be downloaded using the link below:

```
https://s3.unistra.fr/camma_public/datasets/endoscapes/endoscapes.zip
```

And the data standards could be found in the link below:

```
https://github.com/CAMMA-public/Endoscapes?tab=readme-ov-file
```


## runners

Runners is where we can build engines that run our modules. For instance, we can build a basic `main` that just processes files based on inputs
passed in through the command line terminal. However, we can also create a runner that monitors an input from a cable, or messages over a network like Tokio TCP.
