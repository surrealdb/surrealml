"""
Trains a linear regression model using sklearn for multiple outputs for testing.
"""
from sklearn.linear_model import LinearRegression
import numpy as np


def generate_data():
    """
    Generates random data for testing.

    :return: the generated data.
    """
    n_samples = 100
    n_features = 3
    n_outputs = 3
    X = np.array(np.random.rand(n_samples, n_features), dtype=np.float32)
    Y = np.array(np.random.rand(n_samples, n_outputs), dtype=np.float32)
    return X, Y


def train_model(X, Y):
    """
    Trains a linear regression model using sklearn for multiple outputs for testing.
    """
    model = LinearRegression()
    model.fit(X,Y)
    return model


def export_model_onnx(model, inputs):
    """
    Exports the model to ONNX format.

    :param model: the model to export.
    :param inputs: the inputs to the model.
    :return: the path to the exported model.
    """
    import skl2onnx
    return skl2onnx.to_onnx(model, inputs[:1])


def export_model_surml(model, inputs):
    """
    Exports the model to SURML format.

    :param model: the model to export.
    :param inputs: the inputs to the model.
    :return: the path to the exported model.
    """
    from surrealml import SurMlFile, Engine
    file = SurMlFile(
        model=model, 
        name="testprediction", 
        inputs=inputs[:1], 
        engine=Engine.SKLEARN
    )
    file.add_version(version="0.0.1")
    return file
