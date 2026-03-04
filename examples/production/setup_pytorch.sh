#!/bin/bash
set -e

# Create virtual environment if it doesn't exist
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
fi

# Activate virtual environment
source .venv/bin/activate

# Upgrade pip
pip install --upgrade pip

# Install dependencies
# Using CPU version of PyTorch for compatibility and speed in this environment
pip install torch torchvision numpy --index-url https://download.pytorch.org/whl/cpu

echo "Setup complete. PyTorch installed."
python -c "import torch; print(f'PyTorch version: {torch.__version__}')"
