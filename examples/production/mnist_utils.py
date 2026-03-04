import numpy as np
import torch
import torchvision.transforms as T
from PIL import Image
import io
import base64

def decode_png(input_bytes):
    """
    Decodes base64 string or bytes to a normalized 28x28 grayscale image.
    Returns: List[List[float]]
    """
    if isinstance(input_bytes, str):
        # Handle base64 if needed
        # b = base64.b64decode(input_bytes)
        # pass
        pass
    
    # Simulate PNG decoding for this demo using random or provided bytes
    # Real code: img = Image.open(io.BytesIO(input_bytes)).convert('L').resize((28,28))
    # For demo, just return a 28x28 random array if bytes are placeholder
    img_data = np.random.rand(28, 28).astype(np.float32)
    return img_data.tolist()

def normalize_wrapper(img_list):
    """
    img_list: List[List[float]]
    Returns: List[List[float]]
    """
    img_tensor = torch.tensor(img_list).unsqueeze(0) # [1, 28, 28]
    # Normalize with MNIST mean/std
    norm = T.Normalize((0.1307,), (0.3081,))
    normalized = norm(img_tensor)
    return normalized.squeeze().tolist()

def softmax(logits_list):
    """
    logits_list: List[float]
    Returns: List[float]
    """
    logits = torch.tensor(logits_list)
    probs = torch.softmax(logits, dim=0)
    return probs.tolist()

def argmax(probs_list):
    """
    probs_list: List[float]
    Returns: int
    """
    probs = torch.tensor(probs_list)
    return int(torch.argmax(probs).item())

def get_max_prob(probs_list):
    """
    probs_list: List[float]
    Returns: float
    """
    probs = torch.tensor(probs_list)
    return float(torch.max(probs).item())
