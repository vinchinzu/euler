#!/usr/bin/env python3
"""
Better constructions for complexity ≤ 3 matrices.

For a matrix to have complexity ≤ 3, it needs at most 3 distinct patterns
in the union of rows and columns.

Key insight: we can use ANY arrangement of rows, not just blocks.
For 2 row types P and Q with σ being the row assignment:
Columns depend on σ and (P[j], Q[j]).

The column patterns are:
- (0,...,0) if some j has P[j]=Q[j]=0
- (1,...,1) if some j has P[j]=Q[j]=1
- σ if some j has P[j]=1,Q[j]=0
- comp(σ) if some j has P[j]=0,Q[j]=1

For complexity ≤ 3: |{P, Q} ∪ {column patterns}| ≤ 3.
This means: exactly 1 column pattern is outside {P, Q} (for comp=3),
or 0 for comp=2.

For complexity = 3: exactly one "new" column pattern.

Possible new patterns: one of {0^n, 1^n, σ, comp(σ)}.

Case A: the new pattern is 0^n.
Need: 0^n ∉ {P,Q} (it's new) and (a > 0) (it's needed).
And all OTHER column patterns must be in {P,Q}:
- 1^n ∈ {P,Q} if b > 0 (or b=0).
- σ ∈ {P,Q} if c > 0 (or c=0).
- comp(σ) ∈ {P,Q} if d > 0 (or d=0).

So: b > 0 => 1^n ∈ {P,Q} (i.e., P=1^n or Q=1^n).
    c > 0 => σ ∈ {P,Q}.
    d > 0 => comp(σ) ∈ {P,Q}.

These are similar to the comp-2 constraints but with 0^n allowed as extra.

Case B: the new pattern is 1^n.
Symmetric to case A.

Case C: the new pattern is σ.
Need: σ ∉ {P,Q} and c > 0.
Other constraints:
- 0^n ∈ {P,Q} if a > 0.
- 1^n ∈ {P,Q} if b > 0.
- comp(σ) ∈ {P,Q} if d > 0.

Case D: the new pattern is comp(σ).
Similar to Case C.

This gives us a much larger set of achievable k values.

Let me implement this systematically.
"""

def get_comp2or3_k_values(n):
    """Find all k values achievable with complexity ≤ 3 using 2 row types."""
    achievable = set()

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    # P = Q. Complexity = |{P} ∪ {0 if a>0} ∪ {1 if b>0}|.
                    # This is 1, 2, or 3.
                    # k = n * (b) (since all rows have weight b).
                    # We need complexity ≤ 3.
                    # If a > 0 and b > 0: comp = 3 (P, 0^n, 1^n all distinct since P has weight b, 0 < b < n).
                    #   k = n*b. If 0 < k < n^2: add it.
                    # If a = 0 or b = 0: comp ≤ 2 (handled elsewhere).
                    wp = b
                    if a > 0 and b > 0 and b < n:
                        k = n * b
                        if 0 < k < n*n:
                            achievable.add(k)
                    continue

                wp = b + c
                wq = b + d

                # For each case of "what is the one new column pattern":
                # We try all valid σ weights for each case.

                # ---- COMP ≤ 2: no new pattern ----
                # (Already covered by the main formula.)
                # I'll include it here for completeness.

                for new_pattern in [None, 'zero', 'one', 'sigma', 'comp_sigma']:
                    # Constraints:
                    # If new_pattern is None: all column patterns ∈ {P,Q}.
                    # If new_pattern is 'zero': 0^n ∉ {P,Q}, a>0, others ∈ {P,Q}.
                    # etc.

                    # Check if 0^n is needed (a>0), in {P,Q}, or is the new pattern
                    zero_handled = True
                    if a > 0:
                        zero_in_PQ = (wp == 0) or (wq == 0)
                        if new_pattern == 'zero':
                            zero_handled = True  # it's the extra pattern
                        elif zero_in_PQ:
                            zero_handled = True
                        else:
                            zero_handled = False
                    # Check for new_pattern consistency
                    if new_pattern == 'zero' and (a == 0 or (wp == 0) or (wq == 0)):
                        continue  # 0^n not needed or already in {P,Q}

                    one_handled = True
                    if b > 0:
                        one_in_PQ = (wp == n) or (wq == n)
                        if new_pattern == 'one':
                            one_handled = True
                        elif one_in_PQ:
                            one_handled = True
                        else:
                            one_handled = False
                    if new_pattern == 'one' and (b == 0 or (wp == n) or (wq == n)):
                        continue

                    if not zero_handled or not one_handled:
                        continue

                    # For σ and comp(σ) constraints:
                    # Enumerate possible s values
                    for s in range(n+1):
                        sigma_in_PQ = (s == wp) or (s == wq)
                        sigma_is_zero = (s == 0)
                        sigma_is_one = (s == n)

                        comp_s = n - s
                        comp_in_PQ = (comp_s == wp) or (comp_s == wq)
                        comp_is_zero = (comp_s == 0)  # s == n
                        comp_is_one = (comp_s == n)  # s == 0

                        # Handle σ constraint
                        sigma_ok = True
                        if c > 0:
                            if sigma_in_PQ:
                                sigma_ok = True
                            elif new_pattern == 'sigma':
                                sigma_ok = True
                            elif new_pattern == 'zero' and sigma_is_zero:
                                sigma_ok = True  # σ = 0^n = the new pattern
                            elif new_pattern == 'one' and sigma_is_one:
                                sigma_ok = True  # σ = 1^n = the new pattern
                            else:
                                sigma_ok = False

                        if new_pattern == 'sigma' and (c == 0 or sigma_in_PQ):
                            continue  # σ must be new

                        # Handle comp(σ) constraint
                        comp_ok = True
                        if d > 0:
                            if comp_in_PQ:
                                comp_ok = True
                            elif new_pattern == 'comp_sigma':
                                comp_ok = True
                            elif new_pattern == 'zero' and comp_is_zero:
                                comp_ok = True
                            elif new_pattern == 'one' and comp_is_one:
                                comp_ok = True
                            else:
                                comp_ok = False

                        if new_pattern == 'comp_sigma' and (d == 0 or comp_in_PQ):
                            continue

                        # Also: σ and comp_sigma are different patterns.
                        # If both are "new", that's 2 new patterns (comp ≥ 4). Not allowed.
                        # But we only have one new pattern. So if c>0 and σ not in {P,Q,new}
                        # and d>0 and comp(σ) not in {P,Q,new}, that's a problem.

                        if not sigma_ok or not comp_ok:
                            continue

                        # Compute k
                        k = s * wp + (n - s) * wq
                        if 0 < k < n*n:
                            achievable.add(k)

                        # Complement
                        k_comp = n*n - k
                        if 0 < k_comp < n*n:
                            achievable.add(k_comp)

    return achievable


n = 20
comp3_set = get_comp2or3_k_values(n)
print(f"n={n}: {len(comp3_set)} k values achievable with comp ≤ 3")

# Now check how many are NOT in this set
all_k = set(range(1, n*n))
not_achieved = all_k - comp3_set
print(f"Not achieved (potentially comp > 3): {len(not_achieved)}")
if len(not_achieved) < 100:
    print(f"  Values: {sorted(not_achieved)[:50]}")

# Compute C(n)
S2 = set()
for cc in range(1, n):
    S2.add(cc * cc)
    S2.add(n * n - cc * cc)
for x in range(1, n):
    y = n - x
    S2.add(x * x + y * y)
    S2.add(2 * x * y)
S2.discard(0)
S2.discard(n * n)

N2 = len(S2)
N3_set = comp3_set - S2
N4_set = not_achieved
N3 = len(N3_set)
N4 = len(N4_set)

C_val = 2 + 2*N2 + 3*N3 + 4*N4
# Verify: 2 + 2*N2 + 3*N3 + 4*N4 = 2 + 2*N2 + 3*(comp3_set_extra) + 4*N4
# Actually: C = sum c(n,k) = 1*2 + 2*N2 + 3*N3 + 4*N4 (if c max is 4)
# where N2 = # of k in [1,n^2-1] with c=2, N3 with c=3, N4 with c=4.
# N2 + N3 + N4 = n^2 - 1.
# C = 2 + 2*N2 + 3*N3 + 4*N4 = 2 + 2*(n^2-1-N3-N4) + 3*N3 + 4*N4
# = 2 + 2n^2 - 2 + N3 + 2*N4 = 2n^2 + N3 + 2*N4.

print(f"\nN2 = {N2}")
print(f"N3 = {N3}")
print(f"N4 = {N4}")
print(f"N2+N3+N4 = {N2+N3+N4} (should be {n*n-1})")
print(f"C({n}) = 2*{n*n} + {N3} + 2*{N4} = {2*n*n + N3 + 2*N4}")

# Also check for smaller n
for nn in [2, 3, 4, 5, 10, 20]:
    cs = get_comp2or3_k_values(nn)
    s2 = set()
    for cc in range(1, nn):
        s2.add(cc*cc)
        s2.add(nn*nn - cc*cc)
    for x in range(1, nn):
        y = nn - x
        s2.add(x*x + y*y)
        s2.add(2*x*y)
    s2.discard(0)
    s2.discard(nn*nn)

    n2 = len(s2)
    n3 = len(cs - s2)
    n4 = nn*nn - 1 - len(cs)
    cn = 2*nn*nn + n3 + 2*n4
    print(f"C({nn}) = {cn}  (N2={n2}, N3={n3}, N4={n4})")
