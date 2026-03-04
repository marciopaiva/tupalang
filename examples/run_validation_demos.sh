#!/bin/bash
set -e

export PYTHONPATH=$PYTHONPATH:$(pwd)/examples

echo "Running MnistClassifier..."
cargo run -q -p tupa-cli -- run --pipeline MnistClassifier --input examples/input_mnist.json examples/validation_demos.tp

echo "Running LinearRegression..."
cargo run -q -p tupa-cli -- run --pipeline LinearRegression --input examples/input_linear.json examples/validation_demos.tp

echo "Running FraudCheck..."
cargo run -q -p tupa-cli -- run --pipeline FraudCheck --input examples/input_fraud.json examples/validation_demos.tp

echo "All validation demos passed!"
