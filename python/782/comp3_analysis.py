#!/usr/bin/env python3
"""
Compute c(n,k) for all k, allowing complexity up to 3.

For 2 row types P, Q and arbitrary σ:
Union = {P, Q} ∪ col_patterns
col_patterns ⊆ {0^n if a>0, 1^n if b>0, σ if c>0, comp(σ) if d>0}

For complexity ≤ 3: |union| ≤ 3, so |col_patterns \ {P,Q}| ≤ 1.

For each (a,b,c,d) with a+b+c+d = n, (c,d) ≠ (0,0):
The potential new column patterns are:
- 0^n (if a > 0 and 0^n ∉ {P,Q})
- 1^n (if b > 0 and 1^n ∉ {P,Q})
- σ (if c > 0 and σ ∉ {P,Q})
- comp(σ) (if d > 0 and comp(σ) ∉ {P,Q})

P has weight wp = b+c, Q has weight wq = b+d.
0^n = P iff wp = 0 iff b=c=0. 0^n = Q iff wq = 0 iff b=d=0.
1^n = P iff wp = n iff a=d=0. 1^n = Q iff wq = n iff a=c=0.
σ = P iff s = wp (and arrangement matches). σ = Q iff s = wq.
comp(σ) = P iff s = n-wp. comp(σ) = Q iff s = n-wq.

Also σ = 0^n iff s=0. σ = 1^n iff s=n.
comp(σ) = 0^n iff s=n. comp(σ) = 1^n iff s=0.

For complexity ≤ 3, we need at most 1 new pattern outside {P,Q}.
This means: count the number of "needed but not in {P,Q}" patterns, and it must be ≤ 1.

"Needed" patterns:
- 0^n is needed if a > 0
- 1^n is needed if b > 0
- σ is needed if c > 0
- comp(σ) is needed if d > 0

"In {P,Q}": depends on weights.

Let me enumerate all cases.
"""

def compute_comp3_achievable(n):
    """Find all k values achievable with complexity ≤ 3 using 2 row types."""
    achievable = set()

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    # P = Q. Complexity ≤ 2 (P and possibly 0^n, 1^n).
                    # Already handled. Just need k = any s*wp (all have same weight).
                    # P = Q = (0^a, 1^b), wp = b.
                    # k = n * b (all rows same, k = n * b).
                    # Wait, all rows are the same pattern (P=Q), so σ doesn't matter.
                    # k = n * b.
                    # Complexity: {P} ∪ {0^n if a>0} ∪ {1^n if b>0} - overlaps.
                    if a > 0 and b > 0:
                        # {P, 0^n, 1^n}. P = (0^a,1^b). If a>0 and b>0, P ≠ 0 and P ≠ 1.
                        # Complexity = 3.
                        k = n * b
                        if 0 < k < n*n:
                            achievable.add(k)
                    elif a > 0:  # b = 0
                        # P = 0^n. Complexity = 1. k = 0.
                        pass
                    elif b > 0:  # a = 0
                        # P = 1^n. Complexity = 1. k = n^2.
                        pass
                    continue

                wp = b + c
                wq = b + d

                # Check which patterns are automatically in {P,Q}
                zero_in_PQ = (wp == 0) or (wq == 0)  # 0^n = P or 0^n = Q
                one_in_PQ = (wp == n) or (wq == n)    # 1^n = P or 1^n = Q

                # Count "must-add" patterns (needed but not in {P,Q})
                must_add = set()
                if a > 0 and not zero_in_PQ:
                    must_add.add('zero')
                if b > 0 and not one_in_PQ:
                    must_add.add('one')

                # σ and comp(σ) depend on s
                for s in range(n+1):
                    # Can σ be made equal to P, Q, 0^n, or 1^n?
                    sigma_in_PQ = (s == wp) or (s == wq)
                    sigma_is_zero = (s == 0)
                    sigma_is_one = (s == n)

                    comp_in_PQ = (s == n - wp) or (s == n - wq)
                    comp_is_zero = (s == n)
                    comp_is_one = (s == 0)

                    # Additional new patterns from σ and comp(σ)
                    new_from_sigma = set(must_add)  # start with must_add from 0^n, 1^n

                    if c > 0:
                        # σ is needed as column pattern
                        sigma_already_known = sigma_in_PQ
                        # σ could also coincide with 0^n or 1^n
                        if sigma_is_zero and (zero_in_PQ or 'zero' not in new_from_sigma):
                            if zero_in_PQ:
                                sigma_already_known = True
                            elif 'zero' in new_from_sigma:
                                # σ = 0^n, and 0^n is already in new_from_sigma
                                sigma_already_known = True  # will be added as 'zero'
                        if sigma_is_one and (one_in_PQ or 'one' not in new_from_sigma):
                            if one_in_PQ:
                                sigma_already_known = True
                            elif 'one' in new_from_sigma:
                                sigma_already_known = True

                        if not sigma_already_known:
                            # Check if σ coincides with already-needed patterns
                            if sigma_is_zero and 'zero' in new_from_sigma:
                                pass  # σ = 0^n, already counted in must_add
                            elif sigma_is_one and 'one' in new_from_sigma:
                                pass  # σ = 1^n, already counted
                            else:
                                new_from_sigma.add('sigma')

                    if d > 0:
                        comp_already_known = comp_in_PQ
                        if comp_is_zero and zero_in_PQ:
                            comp_already_known = True
                        if comp_is_one and one_in_PQ:
                            comp_already_known = True
                        # comp(σ) could coincide with σ: only if σ = comp(σ), impossible.
                        # comp(σ) could coincide with 0^n or 1^n
                        if comp_is_zero and 'zero' in new_from_sigma:
                            comp_already_known = True  # same as existing new pattern
                        if comp_is_one and 'one' in new_from_sigma:
                            comp_already_known = True
                        # comp(σ) could coincide with σ if both needed: s = n-s => s = n/2
                        # But σ and comp(σ) are different vectors (unless all same, impossible).
                        # So they're always distinct as patterns.

                        if not comp_already_known:
                            if comp_is_zero and 'zero' in new_from_sigma:
                                pass
                            elif comp_is_one and 'one' in new_from_sigma:
                                pass
                            else:
                                new_from_sigma.add('comp_sigma')

                    total_new = len(new_from_sigma)
                    comp_val = 2 + total_new

                    if comp_val <= 3:
                        k = s * wp + (n - s) * wq
                        if 0 < k < n*n:
                            achievable.add(k)
                        k_comp = n*n - k
                        if 0 < k_comp < n*n:
                            achievable.add(k_comp)

    return achievable


# I realize this is getting very hairy. Let me just do it the simplest way:
# directly enumerate (P, Q, σ) as abstract vectors for n up to ~10,
# compute exact complexity, and check.

import itertools

def compute_cnk_2types_direct(n):
    """Compute c(n,k) using 2 row types by direct enumeration."""
    best = {}
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P > Q:
                continue
            for sigma in all_vecs:
                patterns = set()
                k = 0
                # Add row patterns
                if 1 in sigma:
                    patterns.add(P)
                if 0 in sigma:
                    patterns.add(Q)
                # Compute columns and add
                for j in range(n):
                    col = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                    patterns.add(col)
                    k += sum(col)

                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp

    return best


# Check n=5, 6
for n in [5, 6, 7]:
    print(f"\nn={n}:")
    best = compute_cnk_2types_direct(n)

    # Count complexity distribution
    comp_dist = {}
    for k in range(n*n+1):
        c = best.get(k, 999)
        comp_dist[c] = comp_dist.get(c, 0) + 1

    print(f"  Distribution: {comp_dist}")
    C = sum(best.get(k, 999) for k in range(n*n+1))
    print(f"  C({n}) with 2 types = {C}")

    # Check which k values need comp > 3 with 2 types
    need_comp4 = [k for k in range(n*n+1) if best.get(k, 999) > 3]
    print(f"  k values with comp > 3 (2 types): {need_comp4}")
