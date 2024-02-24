# SurgiSeek

Database for advanced surgical video searching - King's x SurrealDB

## Git guide
To develop with a git workflow we need to do the following.

### Go to develop branch
We need to go to the develop branch before we make our own branch because the develop branch
should be the most up to date stable branch. We can do this with the following command:

```
git checkout develop 
```

This will switch you to the local branch of develop. However, you need to make sure that the
develop branch is up to date from the git repo online. This can be done with the command below:

```
git pull origin develop
```

Your local branch is now updated.

### Create Our Own branch
You now need to create your own branch for your new feature/update, and this can be done with the following:

```
git checkout -b <your-branch-name>
```

We are now ready to write our code and commit changes to our code.

### Committing Changes to the code 
When you write new code git will track the changes, you can see what is being changed with the command below:

```
git status
```

If you want to revert a change in a file back to the previous commit, you can do this with the command below:

```
git checkout /path/to/file 
```

If you are happy with your changes you can add them to git with the following command:

```
git add -A 
```

You are now ready to commit them. If you commit them it is hard to revert this but can be done. You can
commit all the files that you added with the command below:

```
git commit -m "some message to tell others what you have done in the commit"
```

You are now ready to push

### Pushing committed code to the online repo 
You can push your commit with the following command:

```
git push origin <your-branch-name>
```

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
