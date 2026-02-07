#!/usr/bin/env python3
"""
Verify for n=7 by direct enumeration of complexity-2 matrices.
Since 2^49 is too large, we enumerate (P, Q, σ) directly.
P, Q ∈ {0,1}^7, σ ∈ {0,1}^7: 128 * 128 * 128 = 2M combinations.
"""

import itertools

def find_comp2_direct(n):
    """Find k values achievable with complexity ≤ 2 by direct construction."""
    achievable = set()
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P >= Q:
                continue
            for sigma in all_vecs:
                # Build matrix
                # M[i][j] = P[j] if sigma[i]=1 else Q[j]
                # Check columns: column j = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                valid = True
                k = 0
                for j in range(n):
                    col = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                    if col != P and col != Q:
                        valid = False
                        break
                    k += sum(col)

                if valid:
                    if 0 < k < n*n:
                        achievable.add(k)

    return achievable

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

for n in [7]:
    print(f"Computing n={n}...")
    direct = find_comp2_direct(n)
    analytic = get_comp2_k_values_v1(n)
    print(f"n={n}: direct count={len(direct)}, analytic count={len(analytic)}")
    if direct != analytic:
        print(f"  DIFF: in direct not analytic: {sorted(direct - analytic)}")
        print(f"       in analytic not direct: {sorted(analytic - direct)}")
    else:
        print("  MATCH")
