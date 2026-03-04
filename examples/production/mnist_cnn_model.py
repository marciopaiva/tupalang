import torch
import torch.nn as nn
import torch.nn.functional as F

class SimpleCNN(nn.Module):
    def __init__(self):
        super(SimpleCNN, self).__init__()
        self.conv1 = nn.Conv2d(1, 32, kernel_size=3)
        self.conv2 = nn.Conv2d(32, 64, kernel_size=3)
        self.fc1 = nn.Linear(64 * 5 * 5, 128)
        self.fc2 = nn.Linear(128, 10)

    def forward(self, x):
        # Input shape: [N, 1, 28, 28] or [1, 28, 28]
        if x.dim() == 3:
            x = x.unsqueeze(0)
        
        x = F.relu(F.max_pool2d(self.conv1(x), 2))
        x = F.relu(F.max_pool2d(self.conv2(x), 2))
        x = x.view(-1, 64 * 5 * 5)
        x = F.relu(self.fc1(x))
        x = self.fc2(x)
        return x

# Global model instance
model = SimpleCNN()
# Load weights if available, otherwise random (fine for demo)
# model.load_state_dict(torch.load("mnist_cnn.pt"))
model.eval()

def forward(input_tensor):
    """
    Wrapper for Tupã to call.
    input_tensor: List[List[...]] representing tensor
    Returns: List[float] representing output logits
    """
    with torch.no_grad():
        # Convert list to tensor
        t = torch.tensor(input_tensor, dtype=torch.float32)
        
        # Ensure correct shape [1, 1, 28, 28]
        if t.dim() == 2: # [28, 28]
            t = t.unsqueeze(0).unsqueeze(0)
        elif t.dim() == 3: # [1, 28, 28]
            t = t.unsqueeze(0)
            
        output = model(t)
        
        # Return as list
        return output.squeeze().tolist()
