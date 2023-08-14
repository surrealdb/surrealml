# SurrealMl
This package is for storing machine learning models with meta data in Rust so they can be used on the SurrealDB server.

## Python tutorial
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

### Saving our model

Our model is now trained and we need to trace the trained model and save it in the C format with the code below:

```python
test_squarefoot = torch.tensor([2800, 3200], dtype=torch.float32)
test_num_floors = torch.tensor([2.5, 3], dtype=torch.float32)
test_inputs = torch.stack([test_squarefoot, test_num_floors], dim=1)

traced_script_module = torch.jit.trace(model, test_inputs)
traced_script_module.save("./stash")
```

### Saving our `.surml` file

We now have the raw C model and this can be used to construct our `.surml` file. It is advised to use a different script to avoid
clashes with the python pytorch as our rust module is also using pytorch. First we load the C model with the following code:

```python
from surrealml.rust_surrealml import load_cached_raw_model, add_column, add_output, add_normaliser, save_model

file_id = load_cached_raw_model("./stash")
```

We now have a `file_id` which can be passed through our `surrealml` functions to access the right loaded model. We need to add some
meta data to the file such as our inputs and outputs with the following code:

```python
add_column(file_id, "squarefoot")
add_column(file_id, "num_floors")
add_output(file_id, "house_price", "z_score", house_price_mean, house_price_std)
```

It must be stressed that the `add_column` order needs to be consistent with the input tensors that the model was trained on as these
now act as key bindings to convert dictionary inputs into the model. We need to also add the normalisers for our column but these will
be automatically mapped therefore we do not need to worry about the order they are inputed:

```python
add_normaliser(file_id, "squarefoot", "z_score", squarefoot_mean, squarefoot_std)
add_normaliser(file_id, "num_floors", "z_score", num_floors_mean, num_floors_std)
```

We then save the model with the following code:

```python
save_model("./test.surml", file_id)
```

### Loading our `.surml` file in Rust

We can now load our `.surml` file with the following code:

```rust
use crate::storage::surml_file::SurMlFile;

let mut file = SurMlFile::from_file("./test.surml").unwrap();
```

### Raw computation
You can have an empty header if you want. This makes sense if you're doing something novel, or complex such as convolutional neural networks
for image processing. To perform a raw computation you can merely just do the following:

```rust
file.model.set_eval();
let x = Tensor::f_from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0]).unwrap().reshape(&[2, 2]);
let outcome = file.model.forward_ts(&[x]);
println!("{:?}", outcome);
```

However if you want to use the header you need to perform a buffered computer

### Buffered computation

This is where the computation utilises the data in the header. We can do this by wrapping our `File` struct in a `ModelComputation` struct
with the code below:

```rust
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
