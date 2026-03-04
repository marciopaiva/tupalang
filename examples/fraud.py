def check_transaction(txn):
    # txn is shape [1, 5]
    # Simple rule: if first element > 0.5, return True
    if len(txn) > 0 and len(txn[0]) > 0 and txn[0][0] > 0.5:
        return [True]
    return [False]
