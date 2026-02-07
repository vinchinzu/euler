#!/usr/bin/env python3
"""
For n=20, for each k value claimed to be in S (complexity 2),
build the actual matrix and verify.
"""

def build_and_verify_full(n, a, b, c, d, sigma_vec):
    """Build the matrix and verify complexity = 2."""
    # Column arrangement: positions 0..a-1 type A, a..a+b-1 type B,
    # a+b..a+b+c-1 type C, a+b+c..n-1 type D.
    P = [0]*a + [1]*b + [1]*c + [0]*d
    Q = [0]*a + [1]*b + [0]*c + [1]*d

    # Build matrix
    matrix = []
    for i in range(n):
        if sigma_vec[i]:
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


def get_all_configs_detailed(n):
    """Get all configurations and verify each one."""
    results = {}  # k -> (comp, config)

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue

                wp = b + c
                wq = b + d

                # Check zero/ones constraints
                zero_is_P = (wp == 0)
                zero_is_Q = (wq == 0)
                ones_is_P = (wp == n)
                ones_is_Q = (wq == n)

                if a > 0 and not (zero_is_P or zero_is_Q):
                    continue
                if b > 0 and not (ones_is_P or ones_is_Q):
                    continue

                # Determine possible sigma types
                configs = []  # (s, sigma_description)

                if c > 0 and d > 0:
                    # Need σ ∈ {P, Q} AND comp(σ) ∈ {P, Q}
                    # This means {σ, comp(σ)} = {P, Q}, so Q = comp(P).
                    # σ = P: s = wp
                    # σ = Q: s = wq
                    P_vec = [0]*a + [1]*b + [1]*c + [0]*d
                    Q_vec = [0]*a + [1]*b + [0]*c + [1]*d
                    comp_P = [1-x for x in P_vec]
                    if tuple(comp_P) == tuple(Q_vec):
                        configs.append((wp, 'P', P_vec))
                        configs.append((wq, 'Q', Q_vec))
                    else:
                        # Q ≠ comp(P), so this (a,b,c,d) doesn't work for c>0,d>0
                        pass

                elif c > 0 and d == 0:
                    # σ ∈ {P, Q}
                    P_vec = [0]*a + [1]*b + [1]*c + [0]*d
                    Q_vec = [0]*a + [1]*b + [0]*c + [1]*d
                    configs.append((wp, 'P', P_vec))
                    configs.append((wq, 'Q', Q_vec))

                elif c == 0 and d > 0:
                    # comp(σ) ∈ {P, Q}, so σ ∈ {comp(P), comp(Q)}
                    P_vec = [0]*a + [1]*b + [1]*c + [0]*d
                    Q_vec = [0]*a + [1]*b + [0]*c + [1]*d
                    comp_P = [1-x for x in P_vec]
                    comp_Q = [1-x for x in Q_vec]
                    configs.append((sum(comp_P), 'compP', comp_P))
                    configs.append((sum(comp_Q), 'compQ', comp_Q))

                for s, desc, sigma_vec in configs:
                    if s < 0 or s > n or sum(sigma_vec) != s:
                        continue

                    comp, k = build_and_verify_full(n, a, b, c, d, sigma_vec)

                    if comp <= 2 and 0 < k < n*n:
                        if k not in results or results[k][0] > comp:
                            results[k] = (comp, (a,b,c,d,s,desc))

                    # Complement
                    k_comp = n*n - k
                    if comp <= 2 and 0 < k_comp < n*n:
                        if k_comp not in results or results[k_comp][0] > comp:
                            results[k_comp] = (comp, (a,b,c,d,s,desc,'complement'))

    return results


n = 20
results = get_all_configs_detailed(n)
verified_S = set(results.keys())

# Compare with formula
def compute_comp2_formula(n):
    s = set()
    for c in range(1, n):
        s.add(c * c)
        s.add(n * n - c * c)
    for x in range(1, n):
        y = n - x
        s.add(x * x + y * y)
        s.add(2 * x * y)
    s.discard(0)
    s.discard(n * n)
    return s

formula_S = compute_comp2_formula(n)

print(f"Formula S: {len(formula_S)} values")
print(f"Verified S: {len(verified_S)} values")

in_formula_not_verified = formula_S - verified_S
in_verified_not_formula = verified_S - formula_S

print(f"In formula but NOT verified: {sorted(in_formula_not_verified)}")
print(f"In verified but NOT formula: {sorted(in_verified_not_formula)}")

if in_formula_not_verified:
    print("\nDetails of unverified k values:")
    for k in sorted(in_formula_not_verified):
        print(f"  k={k}: not found by construction")
        # Find which set it came from
        for c in range(1, n):
            if c*c == k:
                print(f"    From S1: c={c}, k=c^2={c*c}")
            if n*n - c*c == k:
                print(f"    From S2: c={c}, k=n^2-c^2={n*n-c*c}")
        for x in range(1, n):
            y = n - x
            if x*x + y*y == k:
                print(f"    From S3: x={x}, y={y}, k=x^2+y^2={x*x+y*y}")
            if 2*x*y == k:
                print(f"    From S4: x={x}, y={y}, k=2xy={2*x*y}")
