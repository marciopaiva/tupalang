import json
import math

# Simulating a PyTorch model
# In production, this would import torch and load a .pt file

class ViperModel:
    def __init__(self):
        self.weights = [0.5, -0.2, 0.8, 0.1]
        self.bias = 0.05
    
    def forward(self, x):
        # Simple linear layer simulation
        # x is expected to be a list of floats (candles)
        # We'll just take the last 4 values
        if len(x) < 4:
            return 0.0
            
        features = x[-4:]
        activation = sum(f * w for f, w in zip(features, self.weights)) + self.bias
        # Sigmoid
        return 1.0 / (1.0 + math.exp(-activation))

model = ViperModel()

def predict_signal(input_json):
    """
    Input: {"normalized_data": {"candles": [100.1, 100.2, ...]}}
    Output: {"signal_strength": 0.85}
    """
    # Extract from Tupã state structure
    norm_data = input_json.get("normalized_data", {})
    data = norm_data.get("candles", [])
    
    score = model.forward(data)
    
    return {
        "signal_strength": score,
        "action": "BUY" if score > 0.7 else ("SELL" if score < 0.3 else "HOLD")
    }
