import json

# MNIST: [1, 28, 28] (all zeros)
mnist_input = [[[0.0]*28 for _ in range(28)]]
with open('examples/input_mnist.json', 'w') as f:
    json.dump(mnist_input, f)

# Linear Regression: [2, 3] (batch size 2)
linear_input = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
with open('examples/input_linear.json', 'w') as f:
    json.dump(linear_input, f)

# Fraud: [1, 5]
fraud_input = [[0.6, 0.1, 0.1, 0.1, 0.1]]
with open('examples/input_fraud.json', 'w') as f:
    json.dump(fraud_input, f)
