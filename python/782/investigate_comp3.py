#!/usr/bin/env python3
"""
For n=5, find actual comp-3 matrices for k ∈ {7, 11, 14, 18}.
"""
import itertools

def find_matrix_with_comp(n, target_k, max_comp=3):
    """Find a matrix with exactly target_k ones and complexity ≤ max_comp."""
    all_vecs = list(itertools.product([0,1], repeat=n))

    # Try 3 row types
    for i_P in range(len(all_vecs)):
        P = all_vecs[i_P]
        for i_Q in range(i_P, len(all_vecs)):
            Q = all_vecs[i_Q]
            for i_R in range(i_Q, len(all_vecs)):
                R = all_vecs[i_R]
                types = [P, Q, R]

                # Try all σ ∈ {0,1,2}^n
                for sigma in itertools.product(range(3), repeat=n):
                    k = 0
                    patterns = set()
                    for i in range(n):
                        patterns.add(types[sigma[i]])
                    for j in range(n):
                        col = tuple(types[sigma[i]][j] for i in range(n))
                        patterns.add(col)
                        k += sum(1 for i in range(n) if types[sigma[i]][j] == 1)

                    if k == target_k and len(patterns) <= max_comp:
                        return types, sigma, patterns

    return None, None, None


n = 5
for target_k in [7, 11, 14, 18]:
    types, sigma, patterns = find_matrix_with_comp(n, target_k, 3)
    if types:
        print(f"k={target_k}: types={types}, sigma={sigma}, patterns={patterns}")

        # Build and display matrix
        matrix = []
        for i in range(n):
            matrix.append(types[sigma[i]])
        print("  Matrix:")
        for row in matrix:
            print(f"    {row}")
        print(f"  k = {sum(sum(row) for row in matrix)}")
        print(f"  Complexity = {len(patterns)}")
        print()
    else:
        print(f"k={target_k}: NO COMP-3 MATRIX FOUND!")
