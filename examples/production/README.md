# Tupã Production Pipeline Example (MNIST)

This example demonstrates a complete end-to-end production pipeline using Tupã language integrated with PyTorch for deep learning inference.

## Prerequisites

- Rust (latest stable)
- Python 3.8+
- Tupã Compiler (built from source)

## Setup

1. Run the setup script to create a virtual environment and install dependencies:
   ```bash
   chmod +x setup_pytorch.sh
   ./setup_pytorch.sh
   ```

## Files

- `mnist_pipeline.tp`: The Tupã pipeline definition, including external function declarations and audit constraints.
- `mnist_cnn_model.py`: PyTorch model definition (SimpleCNN) and forward pass wrapper.
- `mnist_utils.py`: Helper functions for image decoding and preprocessing.
- `mnist_sample.json`: Sample input data (simulated).

## Running the Pipeline

Activate the virtual environment and run the pipeline using `tupa-cli`:

```bash
source .venv/bin/activate
export PYTHONPATH=$PYTHONPATH:$(pwd)
cargo run -p tupa-cli -- run --pipeline MNISTAudit --input mnist_sample.json mnist_pipeline.tp
```

## Expected Output

The pipeline should execute the following steps:
1. `preprocess`: Decode and normalize the input image.
2. `inference`: Run the PyTorch model to get logits.
3. `postprocess`: Apply softmax and argmax to get the predicted digit.

Finally, it runs validation checks (confidence > 0.7).
