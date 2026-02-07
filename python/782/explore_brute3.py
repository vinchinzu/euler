#!/usr/bin/env python3
"""
Brute force with corrected complexity definition.

Complexity = number of distinct binary strings that appear as either a row or a column.
i.e., |set(rows) ∪ set(columns)|
"""

from collections import defaultdict

def complexity(matrix):
    """Complexity = |distinct rows ∪ distinct columns|."""
    n = len(matrix)
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    return len(patterns)

def brute_force_cnk(n):
    """Compute c(n,k) for all k by enumerating all n×n binary matrices."""
    cnk = {}
    total = n * n
    for bits in range(2 ** total):
        k = bin(bits).count('1')
        matrix = []
        for i in range(n):
            row = []
            for j in range(n):
                row.append((bits >> (i * n + j)) & 1)
            matrix.append(row)
        c = complexity(matrix)
        if k not in cnk or c < cnk[k]:
            cnk[k] = c
    return cnk

def brute_force_Cn(n):
    cnk = brute_force_cnk(n)
    total = sum(cnk[k] for k in range(n*n + 1))
    return total, cnk

# Verify examples
print("=== Verify examples ===")
A = [[1,0,1],[0,0,0],[1,0,1]]
print(f"Matrix A complexity: {complexity(A)}")  # Should be 2

B = [[0,0,0],[0,0,0],[1,1,1]]
print(f"Matrix B complexity: {complexity(B)}")  # Should be 3

# Compute for small n
for n in range(1, 5):
    print(f"\n=== n = {n} ===")
    Cn, cnk = brute_force_Cn(n)
    print(f"C({n}) = {Cn}")
    print(f"c({n},k) values:")
    for k in range(n*n + 1):
        print(f"  c({n},{k}) = {cnk[k]}")
