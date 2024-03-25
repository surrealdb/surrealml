"""
Trains a linear regression model in TensorFlow. Should be used for testing certain processes
for linear regression and TensorFlow.
"""
import numpy as np
import tensorflow as tf

from surrealml.model_templates.datasets.house_linear import HOUSE_LINEAR


class LinearRegressionModel(tf.keras.Model):
    def __init__(self):
        super(LinearRegressionModel, self).__init__()
        self.linear = tf.keras.layers.Dense(1, input_shape=(2,))  # 2 input features, 1 output

    def call(self, inputs):
        return self.linear(inputs)


def train_model():
    """
    Trains a linear regression model in TensorFlow. Should be used for testing certain processes.
    """
    inputs = np.array(HOUSE_LINEAR["inputs"], dtype=np.float32)
    outputs = np.array(HOUSE_LINEAR["outputs"], dtype=np.float32)

    # Initialize the model
    model = LinearRegressionModel()

    # Compile the model
    model.compile(loss='mean_squared_error')

    # Train the model
    model.fit(inputs, outputs, epochs=1000)
    return model


def export_model_tf(model):
    """
    Exports the model to TensorFlow SavedModel format.
    """
    model.save('linear_regression_model')
    return 'linear_regression_model'


def export_model_onnx(model):
    """
    Exports the model to ONNX format.

    :param model: the model to export.
    :return: the path to the exported model.
    """
    import tf2onnx
    # input_shape = model.layers[0].input_shape
    # Adjust the input shape for dynamic batch size
    # dynamic_input_shape = [None] + list(input_shape[1:])

    input_buffer = []

    for i in model.weights[0].shape:
        if i == 1:
            input_buffer.append(None)
        else:
            input_buffer.append(i)

    # Create the input signature using the dynamically determined shape
    input_spec = tf.TensorSpec(input_buffer, tf.float32, name="input")
    onnx_model, _ = tf2onnx.convert.from_keras(model, input_signature=input_spec, output_path=None)

    # # Create a concrete function from the model for conversion
    # # model_function = tf.function(model)
    # concrete_function = tf.function(model).get_concrete_function(input_spec)
    #
    # # Convert the TensorFlow model to ONNX using the concrete function
    # onnx_model, _ = tf2onnx.convert.from_keras(model, input_signature=input_spec, output_path=None)
    # onnx_model, _ = tf2onnx.convert.from_function(concrete_function,
    #                                               input_signature=[input_spec],
    #                                               output_path=None)  # Set to None to get the ONNX model as bytes
    return onnx_model


def export_model_surml(model):
    """
    Placeholder for exporting the model to a hypothetical SURML format. Details would depend on
    specific implementation requirements for SURML format export.
    """
    pass

# Example usage
# model, test_inputs = train_model()
# export_model_tf(model)
# For ONNX and SURML exports, additional implementation would be required.


if __name__ == "__main__":
    # print(list((2, 1)))
    model = train_model()
    # print(model.weights[0].shape)
    onnx_trace = export_model_onnx(model)
    # print(onnx_trace)
