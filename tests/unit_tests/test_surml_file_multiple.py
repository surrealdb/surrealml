import os
import shutil
from unittest import TestCase

from surrealml import Engine, SurMlFile
from surrealml.model_templates.sklearn.sklearn_linear_multiple import train_model, generate_data


class TestSurMlFileMultipleOutPuts(TestCase):

    def setUp(self):
        x, y = generate_data()
        self.x = x
        self.y = y
        self.model = train_model(self.x, self.y)
        self.file = SurMlFile(model=self.model, name="House Price Prediction", inputs=self.x, engine=Engine.SKLEARN)
    
    def tearDown(self):
        try:
            shutil.rmtree(".surmlcache")
        except OSError as e:
            print(f"Error: surmlcache : {e.strerror}")
        os.remove("./test.surml")
    
    def test_full_sklearn_run(self):
        self.file.add_description(description="Model that predicts the price of a house")
        self.file.add_version(version="1.0.0")

        self.file.save("./test.surml")

        new_file = SurMlFile.load("./test.surml", Engine.SKLEARN)

        outcome = new_file.raw_compute(self.x[0])
        self.assertEqual(3, len(outcome))
