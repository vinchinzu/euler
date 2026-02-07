#!/usr/bin/env python3
"""
Direct verification: for small n, enumerate all complexity-2 matrices directly.
"""

import itertools
from collections import defaultdict

def complexity(matrix):
    n = len(matrix)
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    return len(patterns)

def find_comp2_direct(n):
    """Find k values achievable with complexity ≤ 2 by direct construction.

    A complexity-2 matrix has exactly 2 patterns P, Q in rows∪cols.
    Each row is P or Q, each column is P or Q.

    Enumerate all possible (P, Q, σ) where P, Q ∈ {0,1}^n, σ ∈ {0,1}^n.
    Build the matrix: M[i][j] = P[j] if σ[i]=1 else Q[j].
    Check that all columns are P or Q.
    """
    achievable = set()

    # To reduce search space, iterate over P, Q (with P < Q lex), and σ
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P >= Q:
                continue  # avoid duplicates

            for sigma_bits in range(2**n):
                sigma = [(sigma_bits >> i) & 1 for i in range(n)]

                # Build matrix
                matrix = []
                for i in range(n):
                    if sigma[i]:
                        matrix.append(list(P))
                    else:
                        matrix.append(list(Q))

                # Check columns
                valid = True
                for j in range(n):
                    col = tuple(matrix[i][j] for i in range(n))
                    if col != P and col != Q:
                        valid = False
                        break

                if valid:
                    k = sum(sum(row) for row in matrix)
                    if 0 < k < n*n:
                        achievable.add(k)

    return achievable

# Compare with analytic
def get_comp2_k_values_v1(n):
    achievable = set()
    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue
                zero_is_P = (b + c == 0)
                zero_is_Q = (b + d == 0)
                ones_is_P = (a + d == 0)
                ones_is_Q = (a + c == 0)
                if a > 0 and not (zero_is_P or zero_is_Q):
                    continue
                if b > 0 and not (ones_is_P or ones_is_Q):
                    continue
                possible_s = set()
                if c > 0 and d > 0:
                    for sigma_choice in ['P', 'Q']:
                        for comp_choice in ['P', 'Q']:
                            s1 = (b + c) if sigma_choice == 'P' else (b + d)
                            s2 = (a + d) if comp_choice == 'P' else (a + c)
                            if s1 == s2:
                                possible_s.add(s1)
                elif c > 0:
                    possible_s.add(b + c)
                    possible_s.add(b + d)
                elif d > 0:
                    possible_s.add(a + d)
                    possible_s.add(a + c)
                for s in possible_s:
                    if 0 <= s <= n:
                        k = s * (b + c) + (n - s) * (b + d)
                        if 0 < k < n * n:
                            achievable.add(k)
                        k_comp = n * n - k
                        if 0 < k_comp < n * n:
                            achievable.add(k_comp)
    return achievable

for n in range(2, 7):
    direct = find_comp2_direct(n)
    analytic = get_comp2_k_values_v1(n)
    print(f"n={n}: direct={sorted(direct)}")
    print(f"      analytic={sorted(analytic)}")
    if direct != analytic:
        print(f"  DIFF: in direct not analytic: {sorted(direct - analytic)}")
        print(f"       in analytic not direct: {sorted(analytic - direct)}")
    else:
        print("  MATCH")
    print()
