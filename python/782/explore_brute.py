#!/usr/bin/env python3
"""
Brute force exploration of Problem 782.

For small n, enumerate all n×n binary matrices (or a smart subset),
compute the complexity (distinct rows + distinct columns),
and find c(n,k) = min complexity for exactly k ones.
Then C(n) = sum of c(n,k) for k=0..n^2.
"""

import itertools
import numpy as np
from collections import defaultdict

def complexity(matrix):
    """Compute distinct rows + distinct columns of a binary matrix."""
    n = len(matrix)
    # Distinct rows
    row_set = set()
    for row in matrix:
        row_set.add(tuple(row))
    # Distinct columns
    col_set = set()
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        col_set.add(col)
    return len(row_set) + len(col_set)

def brute_force_cnk(n):
    """Compute c(n,k) for all k by enumerating all n×n binary matrices."""
    # c(n,k) = min complexity over all matrices with exactly k ones
    cnk = {}  # k -> min complexity

    total = n * n
    # Enumerate all 2^(n^2) matrices
    for bits in range(2 ** total):
        # Count ones
        k = bin(bits).count('1')
        # Build matrix
        matrix = []
        b = bits
        for i in range(n):
            row = []
            for j in range(n):
                row.append((b >> (i * n + j)) & 1)
            matrix.append(row)

        c = complexity(matrix)
        if k not in cnk or c < cnk[k]:
            cnk[k] = c

    return cnk

def brute_force_Cn(n):
    """Compute C(n) = sum of c(n,k) for k=0..n^2."""
    cnk = brute_force_cnk(n)
    total = sum(cnk[k] for k in range(n*n + 1))
    return total, cnk

# Compute for small n
for n in range(1, 5):
    print(f"\n=== n = {n} ===")
    Cn, cnk = brute_force_Cn(n)
    print(f"C({n}) = {Cn}")
    print(f"c({n},k) values:")
    for k in range(n*n + 1):
        print(f"  c({n},{k}) = {cnk[k]}")
