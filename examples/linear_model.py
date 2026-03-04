def predict(X):
    # X is shape [batch, 3]
    # Return shape [batch, 1]
    # Simple sum of features as prediction
    return [[sum(row)] for row in X]
