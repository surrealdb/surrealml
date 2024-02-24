"""
This test assures that the reshaping of the numpy array maps exactly the same indexes as the original data
in the rust transformation.
"""
import json
import os
from unittest import TestCase, main

import numpy as np


class TestNumpyQualityControl(TestCase):

    def setUp(self):
        script_path = os.path.dirname(os.path.abspath(__file__))
        self.data_path = os.path.join(script_path, '..', "..", "..", "data_access", "data_stash", "images", "dummy_rgb_data.json")

    def tearDown(self):
        pass

    def test_indexes(self):
        # (z, y, x)
        with open(self.data_path, 'r') as f:
            data = json.loads(f.read())

        numpy_data = np.array(data["data"])
        reshaped_data = numpy_data.reshape(3, 5, 10)

        self.assertEqual(111, reshaped_data[0][0][0])
        self.assertEqual(208, reshaped_data[1][0][0])
        self.assertEqual(12, reshaped_data[2][0][0])

        self.assertEqual(65, reshaped_data[0][3][5])
        self.assertEqual(7, reshaped_data[1][3][5])
        self.assertEqual(193, reshaped_data[2][3][5])

        self.assertEqual(253, reshaped_data[0][2][8])
        self.assertEqual(133, reshaped_data[1][2][8])
        self.assertEqual(115, reshaped_data[2][2][8])


if __name__ == '__main__':
    main()