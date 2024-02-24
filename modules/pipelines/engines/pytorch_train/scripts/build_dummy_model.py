import torch
import torch.nn as nn
import torch.nn.functional as F
import os


# Get the full path of the current script
script_path = os.path.abspath(__file__)

# Get the directory containing the script
script_dir = os.path.dirname(script_path)


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
        self.fc2 = nn.Linear(512, 100) # Output layer

    def forward(self, x):
        x = F.relu(self.conv1(x))
        x = F.relu(self.conv2(x))
        x = F.relu(self.conv3(x))
        x = F.relu(self.conv4(x))
        x = self.flatten(x)
        x = F.relu(self.fc1(x))
        x = self.fc2(x)
        return x


if __name__ == "__main__":
    # Instantiate the model
    model = SimpleCNN()

    # Optionally, export the model using TorchScript for deployment
    model_scripted = torch.jit.script(model)

    file_path = os.path.join(script_dir, "..", "assets", "simple_cnn_model.pt")

    if os.path.exists(file_path):
        os.remove(file_path)
    
    model_scripted.save(file_path)
