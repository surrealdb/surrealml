# Glue
In this crate we define the glue code between the different modules. This saves developers having to repeat themselves and also
will enable developers to glue two two modules together. For instance, if both modules have functions that return the same errors
from the `glue` crate, then functions from both of these modules will be able to return the same error and use the `?` operator.