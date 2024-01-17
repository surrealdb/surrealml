# SurrealMl
This package is for storing machine learning models with meta data in Rust so they can be used on the SurrealDB server.

## What is SurrealML?
SurrealML is a feature that allows you to store trained machine learning models in a special format called 'surml'. This enables you to run these models in either Python or Rust, and even upload them to a SurrealDB node to run the models on the server

## Prerequisites
1. A basic understanding of Machine Learning: You should be familiar with ML concepts, algorithms, and model training processes.
2. Knowledge of Python: Proficiency in Python is necessary as SurrealML involves working with Python-based ML models.
3. Familiarity with SurrealDB: Basic knowledge of how SurrealDB operates is required since SurrealML integrates directly with it.
4. Python Environment Setup: A Python environment with necessary libraries installed, including SurrealML, PyTorch or SKLearn (depending on your model preference).
5. SurrealDB Installation: Ensure you have SurrealDB installed and running on your machine or server

## Installation
To install SurrealML, make sure you have Python installed. Then, install the SurrealML library and either PyTorch or SKLearn, based on your model choice. You can install these using pip: 

```
pip install surrealml
pip install torch  # If using PyTorch
pip install scikit-learn  # If using SKLearn 
```
After that, you can train your model and save it in the SurrealML format.


## Compilation config
If nothing is configured the crate will compile the ONNX runtime into the binary. This is the default behaviour. However, if you want to use an ONNX runtime that is installed on your system, you can set the environment variable `ONNXRUNTIME_LIB_PATH` before you compile the crate. This will make the crate use the ONNX runtime that is installed on your system.

## Quick start with Sk-learn

Sk-learn models can also be converted and stored in the `.surml` format enabling developers to load them in any
python version as we are not relying on pickle. Metadata in the file also enables other users of the model to use them
out of the box without having to worry about the normalisation of the data or getting the right inputs in order. You
will also be able to load your sk-learn models in Rust and run them meaning you can use them in your SurrealDB server.
Saving a model is as simple as the following:

```python
import numpy as np
from sklearn.ensemble import RandomForestClassifier
from surrealml import SurMlFile

num_classes = 2
X = np.random.rand(100, 28)
y = np.random.randint(num_classes, size=100)

skl_model = RandomForestClassifier(n_estimators=10, max_depth=10)
skl_model.fit(X, y)
test_file = SurMlFile(model=skl_model, name="random forrest classifier", inputs=X, sklearn=True)
test_file.save("./test_forrest.surml")

# load model and execute a calculation
random_floats = list(np.random.rand(28))
test_load = SurMlFile.load("./test_forrest.surml")
print(test_load.raw_compute(random_floats, [1, -1]))
```

## Python tutorial using Pytorch
To carry out this example we need the following:

- pytorch (pip installed for python)
- numpy
- surrealml

First we need to have one script where we create and store the model. In this example we will merely do a linear regression model
to predict the house price using the number of floors and the square feet. 

### Defining the data

We can create some fake data with the following python code:

```python
import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np


squarefoot = np.array([1000, 1200, 1500, 1800, 2000, 2200, 2500, 2800, 3000, 3200], dtype=np.float32)
num_floors = np.array([1, 1, 1.5, 1.5, 2, 2, 2.5, 2.5, 3, 3], dtype=np.float32)
house_price = np.array([200000, 230000, 280000, 320000, 350000, 380000, 420000, 470000, 500000, 520000], dtype=np.float32)
```

We then get the parameters to perform normalisation to get better convergance with the following"

```python
squarefoot_mean = squarefoot.mean()
squarefoot_std = squarefoot.std()
num_floors_mean = num_floors.mean()
num_floors_std = num_floors.std()
house_price_mean = house_price.mean()
house_price_std = house_price.std()
```

We then normalise our data with the code below:

```python
squarefoot = (squarefoot - squarefoot.mean()) / squarefoot.std()
num_floors = (num_floors - num_floors.mean()) / num_floors.std()
house_price = (house_price - house_price.mean()) / house_price.std()
```

We then create our tensors so they can be loaded into our model and stack it together with the following:

```python
squarefoot_tensor = torch.from_numpy(squarefoot)
num_floors_tensor = torch.from_numpy(num_floors)
house_price_tensor = torch.from_numpy(house_price)

X = torch.stack([squarefoot_tensor, num_floors_tensor], dim=1)
```

### Defining our model

We can now define our linear regression model with loss function and an optimizer with the code below:

```python
# Define the linear regression model
class LinearRegressionModel(nn.Module):
    def __init__(self):
        super(LinearRegressionModel, self).__init__()
        self.linear = nn.Linear(2, 1)  # 2 input features, 1 output

    def forward(self, x):
        return self.linear(x)


# Initialize the model
model = LinearRegressionModel()

# Define the loss function and optimizer
criterion = nn.MSELoss()
optimizer = optim.SGD(model.parameters(), lr=0.01)
```

### Training our model

We are now ready to train our model on the data we have generated with 100 epochs with the following loop:

```python
num_epochs = 1000
for epoch in range(num_epochs):
    # Forward pass
    y_pred = model(X)

    # Compute the loss
    loss = criterion(y_pred.squeeze(), house_price_tensor)

    # Backward pass and optimization
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()

    # Print the progress
    if (epoch + 1) % 100 == 0:
        print(f"Epoch [{epoch+1}/{num_epochs}], Loss: {loss.item():.4f}")
```

### Saving our `.surml` file

Our model is now trained and we need some example data to trace the model with the code below:

```python
test_squarefoot = torch.tensor([2800, 3200], dtype=torch.float32)
test_num_floors = torch.tensor([2.5, 3], dtype=torch.float32)
test_inputs = torch.stack([test_squarefoot, test_num_floors], dim=1)
```

We can now wrap our model in the `SurMlFile` object with the following code:

```python
from surrealml import SurMlFile

file = SurMlFile(model=model, name="House Price Prediction", inputs=test_inputs)
```
The name is optional but the inputs and model are essential. We can now add some meta data to the file such as our inputs and outputs with the following code, however meta data is not essential, it just helps with some types of computation:

```python
file.add_column("squarefoot")
file.add_column("num_floors")
file.add_output("house_price", "z_score", house_price_mean, house_price_std)
```

It must be stressed that the `add_column` order needs to be consistent with the input tensors that the model was trained on as these
now act as key bindings to convert dictionary inputs into the model. We need to also add the normalisers for our column but these will
be automatically mapped therefore we do not need to worry about the order they are inputed, again, normalisers are optional, you can
normalise the data yourself:

```python
file.add_normaliser("squarefoot", "z_score", squarefoot_mean, squarefoot_std)
file.add_normaliser("num_floors", "z_score", num_floors_mean, num_floors_std)
```

We then save the model with the following code:

```python
file.save("./test.surml")
```

### Loading our `.surml` file in Python

If you have followed the previous steps you should have a `.surml` file saved with all our meta data. We load it with the following code:

```python
from surrealml import SurMlFile

new_file = SurMlFile.load("./test.surml")
```

Our model is now loaded. We can now perform computations.

### Raw computation in Python
If you haven't put any meta data into the file then don't worry, we can just perform a raw computation with the following command:

```python
print(new_file.raw_compute([1.0, 2.0]))
```

This will just give you the outcome from the model. If you have put in the metadata then we can perform a buffered computation.
We can also input dimensions for the raw compute which will perform a batch computation. This can be done with the
following code:

```python
print(new_file.raw_compute([1.0, 2.0, 3.0, 4.0]), dims=[2, 2])
```

### Buffered computation in Python

This is where the computation utilises the data in the header. We can do this by merely passing in a dictionary as seen below:

```python
print(new_file.buffered_compute({
    "squarefoot": 1.0,
    "num_floors": 2.0
}))
```

### Uploading our model to SurrealDB

We can upload our trained model with the following code:

```python
url = "http://0.0.0.0:8000/ml/import"
SurMlFile.upload(
    path="./linear_test.surml", 
    url=url, 
    chunk_size=36864, 
    namespace="test", 
    database="test", 
    username="root", 
    password="root"
)
```

### Running SurrealQL operations against our trained model

With this, we can perform SQL statements in our database. To test this, we can create the following rows:

```sql
CREATE house_listing SET squarefoot_col = 500.0, num_floors_col = 1.0;
CREATE house_listing SET squarefoot_col = 1000.0, num_floors_col = 2.0;
CREATE house_listing SET squarefoot_col = 1500.0, num_floors_col = 3.0;

SELECT * FROM (
		SELECT 
			*, 
			ml::house-price-prediction<0.0.1>({ 
				squarefoot: squarefoot_col, 
				num_floors: num_floors_col 
			}) AS price_prediction 
		FROM house_listing
	) 
	WHERE price_prediction > 177206.21875;
```

What is happening here is that we are feeding the columns from the table `house_listing` into a model we uploaded 
called `house-price-prediction` with a version of `0.0.1`. We then get the results of that trained ML model as the column 
`price_prediction`. We then use the calculated predictions to filter the rows giving us the following result:

```json
[
	{
		"id": "house_listing:7bo0f35tl4hpx5bymq5d",
		"num_floors_col": 3,
		"price_prediction": 406534.75,
		"squarefoot_col": 1500
	},
	{
		"id": "house_listing:8k2ttvhp2vh8v7skwyie",
		"num_floors_col": 2,
		"price_prediction": 291870.5,
		"squarefoot_col": 1000
	}
]
```

### Loading our `.surml` file in Rust

We can now load our `.surml` file with the following code:

```rust
use crate::storage::surml_file::SurMlFile;

let mut file = SurMlFile::from_file("./test.surml").unwrap();
```

### Raw computation in Rust
You can have an empty header if you want. This makes sense if you're doing something novel, or complex such as convolutional neural networks
for image processing. To perform a raw computation you can merely just do the following:

```rust
file.model.set_eval();
let x = Tensor::f_from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0]).unwrap().reshape(&[2, 2]);
let outcome = file.model.forward_ts(&[x]);
println!("{:?}", outcome);
```

However if you want to use the header you need to perform a buffered computer

### Buffered computation in Rust

This is where the computation utilises the data in the header. We can do this by wrapping our `File` struct in a `ModelComputation` struct
with the code below:

```rust
use crate::execution::compute::ModelComputation;

let computert_unit = ModelComputation {
    surml_file: &mut file
};
```

Now that we have this wrapper we can create a hashmap with values and keys that correspond to the key bindings. We can then pass this into
a `buffered_compute` that maps the inputs and applies normalisation to those inputs if normalisation is present for that column with the
following:

```rust
let mut input_values = HashMap::new();
input_values.insert(String::from("squarefoot"), 1.0);
input_values.insert(String::from("num_floors"), 2.0);

let outcome = computert_unit.buffered_compute(&mut input_values);
println!("{:?}", outcome);
```
