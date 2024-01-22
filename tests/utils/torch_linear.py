"""
Trains a basic torch model that can be used for testing.
"""
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim


class LinearRegressionModel(nn.Module):
    def __init__(self):
        super(LinearRegressionModel, self).__init__()
        self.linear = nn.Linear(2, 1)  # 2 input features, 1 output

    def forward(self, x):
        return self.linear(x)

def train_model():
    squarefoot = np.array([1000, 1200, 1500, 1800, 2000, 2200, 2500, 2800, 3000, 3200], dtype=np.float32)
    num_floors = np.array([1, 1, 1.5, 1.5, 2, 2, 2.5, 2.5, 3, 3], dtype=np.float32)
    house_price = np.array([200000, 230000, 280000, 320000, 350000, 380000, 420000, 470000, 500000, 520000],
                           dtype=np.float32)
    squarefoot = (squarefoot - squarefoot.mean()) / squarefoot.std()
    num_floors = (num_floors - num_floors.mean()) / num_floors.std()
    house_price = (house_price - house_price.mean()) / house_price.std()
    squarefoot_tensor = torch.from_numpy(squarefoot)
    num_floors_tensor = torch.from_numpy(num_floors)
    house_price_tensor = torch.from_numpy(house_price)

    X = torch.stack([squarefoot_tensor, num_floors_tensor], dim=1)
    # Initialize the model
    model = LinearRegressionModel()

    # Define the loss function and optimizer
    criterion = nn.MSELoss()
    optimizer = optim.SGD(model.parameters(), lr=0.01)

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

    test_squarefoot = torch.tensor([2800, 3200], dtype=torch.float32)
    test_num_floors = torch.tensor([2.5, 3], dtype=torch.float32)
    x = torch.stack([test_squarefoot, test_num_floors], dim=1)
    return model, x
