#!/usr/bin/env python3
"""
For each claimed complexity-2 k value at n=20, build the actual matrix
and verify it really has complexity 2.
"""

def build_matrix_and_check(n, a, b, c, d, s, sigma_choice):
    """
    Build an n×n matrix with complexity 2.

    Column types:
    - Positions 0..a-1: type A (P[j]=0, Q[j]=0)
    - Positions a..a+b-1: type B (P[j]=1, Q[j]=1)
    - Positions a+b..a+b+c-1: type C (P[j]=1, Q[j]=0)
    - Positions a+b+c..n-1: type D (P[j]=0, Q[j]=1)

    P = [0]*a + [1]*b + [1]*c + [0]*d
    Q = [0]*a + [1]*b + [0]*c + [1]*d

    sigma_choice determines σ:
    'P': σ = P (σ[i] = P[i] for each position i)
    'Q': σ = Q
    """
    P = [0]*a + [1]*b + [1]*c + [0]*d
    Q = [0]*a + [1]*b + [0]*c + [1]*d

    if sigma_choice == 'P':
        sigma = P[:]
    elif sigma_choice == 'Q':
        sigma = Q[:]
    else:
        return None

    if sum(sigma) != s:
        return None

    # Build matrix
    matrix = []
    for i in range(n):
        if sigma[i]:
            matrix.append(P[:])
        else:
            matrix.append(Q[:])

    # Compute complexity
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)

    comp = len(patterns)
    k = sum(sum(row) for row in matrix)

    return comp, k


def get_all_configs(n):
    """Get all (a,b,c,d,s,sigma_choice) configs that my formula produces."""
    configs = {}  # k -> list of configs

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

                possible_configs = []
                if c > 0 and d > 0:
                    for sc in ['P', 'Q']:
                        for cc in ['P', 'Q']:
                            s1 = (b + c) if sc == 'P' else (b + d)
                            s2 = (a + d) if cc == 'P' else (a + c)
                            if s1 == s2:
                                possible_configs.append((s1, sc))
                elif c > 0:
                    possible_configs.append((b + c, 'P'))
                    possible_configs.append((b + d, 'Q'))
                elif d > 0:
                    # If d > 0, constraint is comp(σ) ∈ {P, Q}
                    # comp(σ) = P means σ = comp(P), weight = a+d
                    # comp(σ) = Q means σ = comp(Q), weight = a+c
                    # But comp(P) and comp(Q) aren't P or Q in general.
                    # We need to handle this differently!

                    # Hmm wait, in my original code I used sigma_choice 'P' and 'Q'
                    # but for d>0, c=0: we need comp(σ) ∈ {P,Q}.
                    # comp(σ) = P means σ = comp(P).
                    # comp(σ) = Q means σ = comp(Q).
                    # The s values are a+d and a+c.
                    # But sigma is NOT P or Q; it's comp(P) or comp(Q)!

                    # In my original code, I used s=a+d and s=a+c but built sigma as P or Q.
                    # This is WRONG for the d>0,c=0 case!
                    # When c=0, there's no constraint that σ ∈ {P,Q}.
                    # The constraint is comp(σ) ∈ {P,Q} (from d>0).
                    # So σ = comp(P) (weight a+d) or σ = comp(Q) (weight a+c).

                    # But in my "analytic" formula, I just use s to compute k.
                    # k = s*(b+c) + (n-s)*(b+d) = s*b + (n-s)*(b+d) since c=0.
                    # For s = a+d: k = (a+d)*b + (n-a-d)*(b+d) = (a+d)*b + (b+c)*(b+d)
                    # Wait, c=0 so n-a-d = b. So k = (a+d)*b + b*(b+d) = b*(a+d+b+d) = b*(n+d).
                    # Hmm, let me just compute.

                    # For comp(σ)=P: s=a+d, σ=comp(P)=[1]*a+[0]*b+[0]*c+[1]*d (c=0)=[1]*a+[0]*b+[1]*d
                    possible_configs.append((a + d, 'compP'))
                    possible_configs.append((a + c, 'compQ'))  # c=0 so s=a

                for s, sc in possible_configs:
                    if 0 <= s <= n:
                        k = s * (b + c) + (n - s) * (b + d)
                        if k not in configs:
                            configs[k] = []
                        configs[k].append((a, b, c, d, s, sc))

    return configs


# Test for small n first
for n in [3, 4, 5]:
    print(f"\n=== n={n} ===")
    configs = get_all_configs(n)

    for k in sorted(configs.keys()):
        if not (0 < k < n*n):
            continue
        # Check each config
        any_valid = False
        for cfg in configs[k]:
            a, b, c, d, s, sc = cfg
            if sc in ['P', 'Q']:
                result = build_matrix_and_check(n, a, b, c, d, s, sc)
            elif sc == 'compP':
                # Build with σ = complement(P)
                P = [0]*a + [1]*b + [1]*c + [0]*d
                Q = [0]*a + [1]*b + [0]*c + [1]*d
                sigma = [1-x for x in P]
                if sum(sigma) != s:
                    result = None
                else:
                    matrix = []
                    for i in range(n):
                        if sigma[i]:
                            matrix.append(P[:])
                        else:
                            matrix.append(Q[:])
                    patterns = set()
                    for row in matrix:
                        patterns.add(tuple(row))
                    for j in range(n):
                        col = tuple(matrix[i][j] for i in range(n))
                        patterns.add(col)
                    comp = len(patterns)
                    actual_k = sum(sum(row) for row in matrix)
                    result = (comp, actual_k)
            elif sc == 'compQ':
                P = [0]*a + [1]*b + [1]*c + [0]*d
                Q = [0]*a + [1]*b + [0]*c + [1]*d
                sigma = [1-x for x in Q]
                if sum(sigma) != s:
                    result = None
                else:
                    matrix = []
                    for i in range(n):
                        if sigma[i]:
                            matrix.append(P[:])
                        else:
                            matrix.append(Q[:])
                    patterns = set()
                    for row in matrix:
                        patterns.add(tuple(row))
                    for j in range(n):
                        col = tuple(matrix[i][j] for i in range(n))
                        patterns.add(col)
                    comp = len(patterns)
                    actual_k = sum(sum(row) for row in matrix)
                    result = (comp, actual_k)
            else:
                result = None

            if result and result[0] <= 2 and result[1] == k:
                any_valid = True
                break
            elif result:
                pass  # print(f"    Config {cfg}: comp={result[0]}, k={result[1]}")

        if not any_valid:
            print(f"  k={k}: NO VALID COMP-2 MATRIX FOUND! configs={configs[k]}")
