"""
This file trains and saves the sklearn linear model to the model stash directory for the core to test against
"""
from tests.model_builder.utils import install_package
install_package("skl2onnx==1.16.0")
install_package("scikit-learn==1.4.0")
import os

import onnx

# for a linear regression model with one output
from surrealml.model_templates.sklearn.sklearn_linear import export_model_onnx as linear_sklearn_export_model_onnx
from surrealml.model_templates.sklearn.sklearn_linear import export_model_surml as linear_sklearn_export_model_surml
from surrealml.model_templates.sklearn.sklearn_linear import train_model as linear_sklearn_train_model

# for a linear regression model with multiple outputs
from surrealml.model_templates.sklearn.sklearn_linear_multiple import export_model_onnx as multiple_linear_sklearn_export_model_onnx
from surrealml.model_templates.sklearn.sklearn_linear_multiple import export_model_surml as multiple_linear_sklearn_export_model_surml
from surrealml.model_templates.sklearn.sklearn_linear_multiple import train_model as multiple_linear_sklearn_train_model
from surrealml.model_templates.sklearn.sklearn_linear_multiple import generate_data as multiple_linear_sklearn_generate_data

from tests.model_builder.utils import delete_directory, create_directory, MODEL_STASH_DIRECTORY

sklearn_stash_directory = os.path.join(MODEL_STASH_DIRECTORY, "sklearn")
sklearn_surml_stash_directory = os.path.join(sklearn_stash_directory, "surml")
sklearn_onnx_stash_directory = os.path.join(sklearn_stash_directory, "onnx")

# create the model stash directory if it does not exist
create_directory(dir_path=MODEL_STASH_DIRECTORY)

# delete the directories if they exist
delete_directory(dir_path=sklearn_stash_directory)
delete_directory(dir_path=sklearn_surml_stash_directory)
delete_directory(dir_path=sklearn_onnx_stash_directory)

# create directories for the sklearn models
create_directory(sklearn_stash_directory)
create_directory(sklearn_surml_stash_directory)
create_directory(sklearn_onnx_stash_directory)

# train and stash sklearn model with single output
sklearn_linear_model = linear_sklearn_train_model()
sklearn_linear_surml_file = linear_sklearn_export_model_surml(sklearn_linear_model)
sklearn_linear_onnx_file = linear_sklearn_export_model_onnx(sklearn_linear_model)

sklearn_linear_surml_file.save(
    path=str(os.path.join(sklearn_surml_stash_directory, "linear.surml"))
)
onnx.save(
    sklearn_linear_onnx_file,
    os.path.join(sklearn_onnx_stash_directory, "linear.onnx")
)

# train and stash sklearn model with multiple outputs
X, Y = multiple_linear_sklearn_generate_data()
sklearn_multiple_linear_model = multiple_linear_sklearn_train_model(X, Y)
sklearn_multiple_linear_surml_file = multiple_linear_sklearn_export_model_surml(sklearn_multiple_linear_model, X)
sklearn_multiple_linear_onnx_file = multiple_linear_sklearn_export_model_onnx(sklearn_multiple_linear_model, X)

sklearn_multiple_linear_surml_file.save(
    path=str(os.path.join(sklearn_surml_stash_directory, "multiple_linear.surml"))
)
onnx.save(
    sklearn_multiple_linear_onnx_file,
    os.path.join(sklearn_onnx_stash_directory, "multiple_linear.onnx")
)
