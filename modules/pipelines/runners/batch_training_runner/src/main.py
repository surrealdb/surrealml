from data_access_layer.data_access_layer import read_rgb_image
import torch
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
import numpy as np


class SimpleCNN(nn.Module):

    def __init__(self):
        super(SimpleCNN, self).__init__()
        # Input shape: (batch_size, 3, 480, 853)
        self.conv1 = nn.Conv2d(3, 16, kernel_size=3, stride=2, padding=1) # Output: (batch_size, 16, 240, 427)
        self.conv2 = nn.Conv2d(16, 32, kernel_size=3, stride=2, padding=1) # Output: (batch_size, 32, 120, 214)
        self.conv3 = nn.Conv2d(32, 64, kernel_size=3, stride=2, padding=1) # Output: (batch_size, 64, 60, 107)
        self.conv4 = nn.Conv2d(64, 128, kernel_size=3, stride=2, padding=1) # Output: (batch_size, 128, 30, 54)
        self.flatten = nn.Flatten() # Flatten the output for the fully connected layer
        self.fc1 = nn.Linear(128 * 30 * 54, 512) # First fully connected layer
        self.fc2 = nn.Linear(512, 3) # Output layer with 3 disease classes

    def forward(self, x):
        x = F.relu(self.conv1(x))
        x = F.relu(self.conv2(x))
        x = F.relu(self.conv3(x))
        x = F.relu(self.conv4(x))
        x = self.flatten(x)
        x = F.relu(self.fc1(x))
        x = self.fc2(x)
        return x


def main():
    #  Define parameters
    height = 480
    width = 853
    channels = 3
    num_epochs = 100

    # Create data
    image = read_rgb_image("./assets/test.jpg", width, height)
    image_array = np.array(image, dtype=np.float32).reshape((channels, height, width))
    image_tensor = torch.from_numpy(image_array)

    # Create batch data
    batch_data = torch.stack([image_tensor for _ in range(3)], dim=0)
    batch_data_2 = batch_data.clone()
    all_batches = [batch_data, batch_data_2]

    # Create tags and classes
    disease_names = ['Colon Cancer', 'IBS', 'IBD']  # Adjust as necessary
    num_classes = len(disease_names)

    # Convert disease names to indices for use as tags
    disease_to_index = {disease: index for index, disease in enumerate(disease_names)}
    tags = torch.tensor([disease_to_index[disease] for disease in disease_names])

    # Instantiate the model and adjust the final layer to match the number of classes/tags
    model = SimpleCNN()
    criterion = nn.CrossEntropyLoss()  # Loss function suitable for classification
    optimizer = optim.Adam(model.parameters(), lr=0.001)  # Optimizer
    model.train() # Sets the model to training mode

    # This trains the model, using both batches and epochs
    for epoch in range(num_epochs):
        model.train()  # Set the model to training mode
        # Iterate through the data in batches
        for i, data in enumerate(all_batches):
            # Forward pass
            outputs = model(data)
            loss = criterion(outputs, tags)

            # Calculate accuracy
            _, predicted = torch.max(outputs.data, 1)
            correct = (predicted == tags).sum().item()
            accuracy = correct / len(tags)

            # Backward pass and optimization
            optimizer.zero_grad()
            loss.backward()
            optimizer.step()

            # Now, print out loss and accuracy for each batch
            print(f"Batch [{i + 1}/{len(all_batches)}], Loss: {loss.item():.4f}, Accuracy: {accuracy:.4f}")

            # After finishing all batches for the epoch, print a simple message
        print(f"Epoch {epoch + 1} finished")


if __name__ == "__main__":
    main()
