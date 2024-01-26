try:
    import skl2onnx
except ImportError:
    skl2onnx = None

from surrealml.engine.utils import create_file_cache_path


class SklearnOnnxAdapter:

    @staticmethod
    def check_dependency() -> None:
        """
        Checks if the sklearn dependency is installed raising an error if not.
        Please call this function when performing any sklearn related operations.
        """
        if skl2onnx is None:
            raise ImportError("sklearn feature needs to be installed to use sklearn features")

    @staticmethod
    def save_model_to_onnx(model, inputs) -> str:
        """
        Saves a sklearn model to an onnx file.

        :param model: the sklearn model to convert.
        :param inputs: the inputs to the model needed to trace the model
        :return: the path to the cache created with a unique id to prevent collisions.
        """
        SklearnOnnxAdapter.check_dependency()
        file_path = create_file_cache_path()
        # the below check is to satisfy type checkers
        if skl2onnx is not None:
            onnx = skl2onnx.to_onnx(model, inputs)

            with open(file_path, "wb") as f:
                f.write(onnx.SerializeToString())

            return file_path
