#!/usr/bin/env python3
"""
Verify comp-2 k values for n=20 by actually building matrices.
"""

def get_comp2_k_values(n):
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
                    for sc in ['P', 'Q']:
                        for cc in ['P', 'Q']:
                            s1 = (b + c) if sc == 'P' else (b + d)
                            s2 = (a + d) if cc == 'P' else (a + c)
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


def build_and_verify(n, a, b, c, d, s, sigma_type):
    """Build a complexity-2 matrix and verify it.

    sigma_type: 'P' means σ=P, 'Q' means σ=Q,
                'compP' means σ=complement(P), 'compQ' means σ=complement(Q)
    """
    # Column arrangement: first a type A, b type B, c type C, d type D
    P_vec = [0]*a + [1]*b + [1]*c + [0]*d
    Q_vec = [0]*a + [1]*b + [0]*c + [1]*d

    if sigma_type == 'P':
        sigma = P_vec[:]
    elif sigma_type == 'Q':
        sigma = Q_vec[:]
    elif sigma_type == 'compP':
        sigma = [1-x for x in P_vec]
    elif sigma_type == 'compQ':
        sigma = [1-x for x in Q_vec]

    if sum(sigma) != s:
        return False, 0, 0

    matrix = []
    for i in range(n):
        if sigma[i]:
            matrix.append(P_vec[:])
        else:
            matrix.append(Q_vec[:])

    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    comp = len(patterns)

    actual_k = sum(sum(row) for row in matrix)
    return True, comp, actual_k


def find_all_verified_comp2(n):
    """Find all k values that can be verified with an actual complexity-2 matrix."""
    verified = set()

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
                sigma_types = []
                if c > 0 and d > 0:
                    for sc in ['P', 'Q']:
                        for cc in ['P', 'Q']:
                            s1 = (b + c) if sc == 'P' else (b + d)
                            s2 = (a + d) if cc == 'P' else (a + c)
                            if s1 == s2:
                                possible_s.add(s1)
                                # Determine sigma type
                                if sc == 'P':
                                    sigma_types.append(('P', s1))
                                else:
                                    sigma_types.append(('Q', s1))
                elif c > 0:
                    possible_s.add(b + c)
                    sigma_types.append(('P', b + c))
                    possible_s.add(b + d)
                    sigma_types.append(('Q', b + d))
                elif d > 0:
                    possible_s.add(a + d)
                    sigma_types.append(('compP', a + d))
                    possible_s.add(a + c)
                    sigma_types.append(('compQ', a + c))

                for stype, s in sigma_types:
                    if 0 <= s <= n:
                        ok, comp, k = build_and_verify(n, a, b, c, d, s, stype)
                        if ok and comp <= 2 and 0 < k < n*n:
                            verified.add(k)
                        # Also complement
                        k_comp = n*n - k
                        if ok and comp <= 2 and 0 < k_comp < n*n:
                            verified.add(k_comp)

    return verified


# Compare
for n in [10, 15, 20]:
    analytic = get_comp2_k_values(n)
    verified = find_all_verified_comp2(n)

    print(f"\nn={n}:")
    print(f"  Analytic: {len(analytic)} k-values: {sorted(analytic)[:30]}")
    print(f"  Verified: {len(verified)} k-values: {sorted(verified)[:30]}")

    if analytic != verified:
        in_a_not_v = sorted(analytic - verified)
        in_v_not_a = sorted(verified - analytic)
        print(f"  In analytic but NOT verified: {in_a_not_v}")
        print(f"  In verified but NOT analytic: {in_v_not_a}")
    else:
        print("  MATCH")

    N2 = len(verified)
    N3 = (n*n - 1) - N2
    Cn = 2*n*n + N3
    print(f"  C({n}) with verified = {Cn}")
