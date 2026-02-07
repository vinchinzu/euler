#!/usr/bin/env python3
"""
For each k in [1, n^2-1], find minimum complexity by trying many matrix constructions.

Construction types:
1. Two row types P, Q in "block" arrangement (already gives comp-2 set).
2. Two row types P, Q with various σ arrangements (gives comp ≤ 6 in general).
3. Single row type (all identical rows) - gives comp ≤ 3.
4. Generalized block with permutations.

Focus on achieving comp ≤ 3 for all k.
For k not in comp-2 set, try to build comp-3 matrices.

KEY INSIGHT: For a comp-3 matrix, we need exactly 3 patterns in rows∪cols.
This means we need a matrix where the set of distinct row vectors and
distinct column vectors together form a set of 3 elements.

Approach: enumerate all possible triples of patterns (A, B, C) and all possible
row/column assignments.
For efficiency: parameterize by row types and use arrangement to control columns.

For 2 row types with block arrangement:
Row types: R1 = (1^w1, 0^{n-w1}), R2 = (1^w2, 0^{n-w2}).
a rows of R1, (n-a) rows of R2 (block: R1 rows first).
k = a*w1 + (n-a)*w2.

Columns:
If w1 ≥ w2: (WLOG)
  cols 0..w2-1: all ones (1^n)
  cols w2..w1-1: (1^a, 0^{n-a})
  cols w1..n-1: all zeros (0^n)

  Col types: ⊆ {1^n, (1^a, 0^{n-a}), 0^n}
  Row types: {R1, R2}

  Union: {R1, R2} ∪ {col types}

  Coincidences:
  R1 = 1^n iff w1 = n.
  R2 = 0^n iff w2 = 0.
  R1 = (1^a, 0^{n-a}) iff w1 = a.
  R2 = (1^a, 0^{n-a}) iff w2 = a.
  1^n = 0^n: never.
  (1^a,0^{n-a}) = 0^n iff a = 0.
  (1^a,0^{n-a}) = 1^n iff a = n.

For comp ≤ 3: |union| ≤ 3.
"""

def compute_min_comp_block(n):
    """Minimum complexity for each k using block construction."""
    best = [999] * (n*n + 1)
    best[0] = 1
    best[n*n] = 1

    for w1 in range(n+1):
        for w2 in range(n+1):
            for a in range(n+1):
                k = a * w1 + (n - a) * w2
                if k < 0 or k > n*n:
                    continue

                # Determine distinct patterns
                patterns = set()

                # Row types
                R1 = (1,) * w1 + (0,) * (n - w1)
                R2 = (1,) * w2 + (0,) * (n - w2)

                if a > 0:
                    patterns.add(R1)
                if n - a > 0:
                    patterns.add(R2)

                # Column types (block arrangement: first a rows R1, rest R2)
                w_lo = min(w1, w2)
                w_hi = max(w1, w2)

                # Cols 0..w_lo-1: all 1s
                if w_lo > 0:
                    patterns.add((1,)*n)
                # Cols w_hi..n-1: all 0s
                if w_hi < n:
                    patterns.add((0,)*n)

                # Cols w_lo..w_hi-1: mixed
                if w1 > w2:
                    # R1 has 1, R2 has 0 -> col = (1^a, 0^{n-a})
                    if w_lo < w_hi:
                        patterns.add((1,)*a + (0,)*(n-a))
                elif w1 < w2:
                    # R1 has 0, R2 has 1 -> col = (0^a, 1^{n-a})
                    if w_lo < w_hi:
                        patterns.add((0,)*a + (1,)*(n-a))

                comp = len(patterns)
                best[k] = min(best[k], comp)

    return best


def C_val(best, n):
    return sum(best[k] for k in range(n*n+1))


# Test block construction
for nn in [3, 4, 5, 10, 20]:
    best = compute_min_comp_block(nn)
    c = C_val(best, nn)
    max_comp = max(best[k] for k in range(nn*nn+1))
    comp_gt3 = sum(1 for k in range(nn*nn+1) if best[k] > 3)
    print(f"C({nn}) = {c}, max comp = {max_comp}, #{'>3'} = {comp_gt3}")


# Now: for block construction, some k values have comp > 3.
# For those k, try OTHER constructions.
# Specifically, try non-block arrangements of 2 row types.

def compute_min_comp_2types(n):
    """Min complexity using 2 row types with ANY σ arrangement.

    For row types R1 = (1^w1, 0^{n-w1}), R2 = (1^w2, 0^{n-w2}):
    σ determines which rows use R1.

    Column j depends on (w1, w2, σ):
    - j < min(w1,w2): entry = 1 for both types -> col = (1,...,1)
    - j ∈ [w2, w1) if w1 > w2: R1 has 1, R2 has 0 -> col[i] = σ[i]
    - j ∈ [w1, w2) if w2 > w1: R1 has 0, R2 has 1 -> col[i] = 1-σ[i]
    - j ≥ max(w1,w2): entry = 0 for both -> col = (0,...,0)

    So column patterns are:
    - 1^n (if min(w1,w2) > 0)
    - 0^n (if max(w1,w2) < n)
    - σ (if w1 > w2)
    - comp(σ) (if w2 > w1)
    - σ and comp(σ) can both appear if w1 > w2 and w2 > w1, which is impossible.
      So either σ or comp(σ) appears, or neither (if w1=w2, meaning R1=R2).

    Union = {R1, R2} ∪ {1^n if min(w1,w2)>0} ∪ {0^n if max(w1,w2)<n} ∪ {σ or comp(σ) if w1≠w2}

    For w1 > w2 (WLOG):
    Union = {R1, R2, 1^n [if w2>0], 0^n [if w1<n], σ}

    Now σ is an arbitrary binary vector with weight s = a (number of R1-rows).
    We want to choose σ to minimize complexity.

    σ coincides with:
    - R1 iff σ = R1 iff σ = (1^w1, 0^{n-w1}) iff s = w1
    - R2 iff σ = R2 iff σ = (1^w2, 0^{n-w2}) iff s = w2
    - 0^n iff s = 0
    - 1^n iff s = n
    - R1 = 1^n iff w1 = n
    - R2 = 0^n iff w2 = 0

    But here σ is not (1^s, 0^{n-s}) in general! σ is any vector with weight s.
    So σ = R1 = (1^w1, 0^{n-w1}) requires s = w1 AND σ has 1s in first w1 positions.
    Since we're free to choose σ, we CAN make σ = R1 if s = w1.

    But wait: σ[i] = 1 means row i uses R1. σ = R1 = (1^w1, 0^{n-w1}) means
    the first w1 rows use R1 and the rest use R2. This is valid!
    And column j ∈ [w2, w1): col = σ = R1.

    So for s = w1: σ = R1, and col pattern σ = R1 ∈ {R1, R2}. So NO new column pattern.
    Union = {R1, R2, 1^n [if w2>0], 0^n [if w1<n]}.
    This has ≤ 4 elements, potentially ≤ 3.

    For s = w2: σ = R2, col pattern σ = R2 ∈ {R1, R2}. Same as above.

    For s = 0 (no R1 rows): only R2 rows. Col σ has weight 0 but...
    Wait, if s = 0, all rows use R2. So row pattern = {R2}.
    Col pattern: for j < w2: 1^n. For j ≥ w2: 0^n. (Since w1 > w2 but all rows use R2, so
    col j ∈ [w2, w1): R2 has 0 -> col = 0^n. Wait, that's not right.)

    Hmm, let me reconsider. If s = 0 (no R1 rows), then ALL rows are R2 = (1^w2, 0^{n-w2}).
    Column j = (R2[j], R2[j], ..., R2[j]) = (R2[j])^n.
    If j < w2: col = 1^n. If j ≥ w2: col = 0^n.
    Union: {R2, 1^n [if w2>0], 0^n [if w2<n]}.
    If w2 > 0 and w2 < n: comp = 3 (R2, 1^n, 0^n all distinct if w2 ∈ (0,n)).
    """
    best = [999] * (n*n + 1)
    best[0] = 1
    best[n*n] = 1

    for w1 in range(n+1):
        for w2 in range(w1+1):  # w1 >= w2
            R1 = (1,) * w1 + (0,) * (n - w1)
            R2 = (1,) * w2 + (0,) * (n - w2)

            for s in range(n+1):
                k = s * w1 + (n - s) * w2
                if k < 0 or k > n*n:
                    continue

                # Column patterns:
                patterns = set()

                # Row patterns
                if s > 0: patterns.add(R1)
                if n - s > 0: patterns.add(R2)

                # Constant columns
                if w2 > 0:
                    patterns.add((1,)*n)
                if w1 < n:
                    patterns.add((0,)*n)

                # Variable columns (exist if w1 > w2):
                if w1 > w2:
                    # Col = σ. We choose σ to minimize patterns.
                    # Best: σ coincides with an existing pattern.
                    # Possible:
                    # σ = R1: s = w1.
                    # σ = R2: s = w2.
                    # σ = 0^n: s = 0.
                    # σ = 1^n: s = n.

                    # If any of these apply, σ is not new.
                    if s == w1:
                        patterns.add(R1)  # σ = R1, already in patterns
                    elif s == w2:
                        patterns.add(R2)  # σ = R2, already in patterns
                    elif s == 0:
                        patterns.add((0,)*n)  # σ = 0^n
                    elif s == n:
                        patterns.add((1,)*n)  # σ = 1^n
                    else:
                        # σ is a new pattern.
                        # Can we choose σ to equal some other needed constant?
                        # No, σ is determined by row assignment.
                        # But we CAN choose σ = (1^s, 0^{n-s}) which is a
                        # specific vector. Is this one of the existing patterns?
                        # (1^s, 0^{n-s}) = R1 iff s = w1, = R2 iff s = w2.
                        # Already checked.
                        # So σ is genuinely new.
                        sigma_vec = (1,)*s + (0,)*(n-s)
                        patterns.add(sigma_vec)

                comp = len(patterns)
                best[k] = min(best[k], comp)

            # Also try w2 > w1
            if w1 != w2:
                w1_new, w2_new = w2, w1
                R1_new = (1,) * w1_new + (0,) * (n - w1_new)
                R2_new = (1,) * w2_new + (0,) * (n - w2_new)

                for s in range(n+1):
                    k = s * w1_new + (n - s) * w2_new
                    if k < 0 or k > n*n:
                        continue

                    patterns = set()
                    if s > 0: patterns.add(R1_new)
                    if n - s > 0: patterns.add(R2_new)
                    if w1_new > 0:
                        patterns.add((1,)*n)
                    if w2_new < n:
                        patterns.add((0,)*n)
                    if w2_new > w1_new:
                        if s == w1_new:
                            pass  # σ = R1_new
                        elif s == w2_new:
                            pass  # σ = R2_new
                        elif s == 0:
                            patterns.add((0,)*n)
                        elif s == n:
                            patterns.add((1,)*n)
                        else:
                            # comp(σ) appears. comp(σ) = (0^s, 1^{n-s}).
                            patterns.add((0,)*s + (1,)*(n-s))

                    comp = len(patterns)
                    best[k] = min(best[k], comp)

    return best


# Actually, I realize the issue: when w1 > w2, the column pattern at mixed positions is σ
# (the row assignment vector). But σ = (1^s, 0^{n-s}) only if we arrange R1-rows first!
# With σ = R1 arrangement, σ = (1^w1, 0^{n-w1}), which has weight w1.
# With σ = R2 arrangement, σ = (1^w2, 0^{n-w2}), weight w2.
# With σ = P general arrangement, σ can be ANY vector with weight s.
# The column pattern at mixed positions is σ itself.
# So σ = (1^s, 0^{n-s}) gives a specific column pattern.
# But σ could also be, say, (0,1,0,1,...) if arranged differently.

# The key point: σ determines the column pattern. We want to choose σ
# (subject to weight constraint) to minimize the total number of distinct patterns.

# If σ can be ANY vector with weight s, the column pattern is σ.
# We want σ to equal an existing pattern (R1, R2, 0^n, or 1^n).
# If that's not possible, σ is a new pattern.

# But the column pattern at type C positions is σ. If we can make σ = R1
# (when s = w1), the column pattern merges with the row pattern.

# What if we DON'T use block arrangement but instead interleave?
# E.g., for n=20, w1=11, w2=9, s=7:
# σ is some vector with 7 ones. Column pattern at type C positions = σ.
# σ = (1^7, 0^13) ≠ R1 and ≠ R2. New pattern.

# BUT: what if we arrange σ cleverly?
# σ = R2 if s = w2 = 9. But s = 7 ≠ 9.
# σ = R1 if s = w1 = 11. But s = 7 ≠ 11.

# So for s ∉ {0, w1, w2, n}, σ is genuinely a new pattern.
# Complexity ≥ 4 in general. With the right coincidences, possibly 3.

# Wait: σ might coincide with (1^a, 0^{n-a}) (column pattern) even if σ ≠ R1 and σ ≠ R2.
# If σ = (1^s, 0^{n-s}) and a = s. But (1^a, 0^{n-a}) is not a pattern that appears
# unless it's R1 or R2.

# The column patterns are: 1^n, 0^n, and σ. σ is a new pattern if not one of these.
# So for s ∉ {0, w2, w1, n}: complexity = |{R1, R2, 1^n [if w2>0], 0^n [if w1<n], σ}|.
# This is 4 or 5.

# Hmm. So for many k values, 2-row-type block matrices give comp ≥ 4.
# The question is whether more complex arrangements or 3+ row types help.

# For 3 row types: we get more flexibility.
# E.g., 3 row types: R1=1^n, R2=(1^w, 0^{n-w}), R3=0^n.
# a1 rows of R1, a2 rows of R2, a3 rows of R3. a1+a2+a3 = n.
# k = a1*n + a2*w.

# Columns: col j < w: (1^{a1+a2}, 0^{a3}).
#           col j ≥ w: (1^{a1}, 0^{a2+a3}).
# If a1+a2 = n (a3=0): col < w is 1^n. col ≥ w is (1^{a1},0^{a2}).
# If a1 = 0 (no R1): col < w is (1^{a2},0^{a3}). col ≥ w is 0^n.

# Rows: {1^n, (1^w,0^{n-w}), 0^n} minus those with 0 multiplicity.
# Cols: {(1^{a1+a2},0^{a3}), (1^{a1},0^{a2+a3})}.

# Union: rows ∪ cols.
# For all 3 row types present and cols:
# {1^n, (1^w,0^{n-w}), 0^n, (1^{a1+a2},0^{a3}), (1^{a1},0^{a2+a3})}

# For comp ≤ 3: at most 3 patterns total.
# Need 2 of the 5 to coincide with others.

# Example: (1^{a1+a2},0^{a3}) = 1^n iff a3=0 (no R3 rows).
#   Then only 2 row types. Handled.
# (1^{a1},0^{a2+a3}) = 0^n iff a1=0.
# (1^{a1+a2},0^{a3}) = (1^w,0^{n-w}) iff a1+a2 = w.
#   Since a1+a2+a3 = n: a3 = n-w. Also a1+a2 = w.
# (1^{a1},0^{a2+a3}) = (1^w,0^{n-w}) iff a1 = w.

# For comp = 3 with 3 row types and both cols matching rows:
# Need both column types to be among {1^n, (1^w,0^{n-w}), 0^n}.
# (1^{a1+a2},0^{a3}) ∈ {1^n, (1^w,0^{n-w}), 0^n}
# (1^{a1},0^{a2+a3}) ∈ {1^n, (1^w,0^{n-w}), 0^n}

# Option 1: first col = 1^n (a3=0) -> only 2 row types. Skip.
# Option 2: first col = (1^w,0^{n-w}) -> a1+a2 = w, a3 = n-w.
#   Second col: (1^{a1},0^{a2+a3}) = (1^{a1},0^{n-a1}).
#   Must be in {1^n, (1^w,0^{n-w}), 0^n}.
#   (1^{a1},0^{n-a1}) = 1^n iff a1 = n. Then a2 = w-n < 0 unless w=n. Skip.
#   (1^{a1},0^{n-a1}) = (1^w,0^{n-w}) iff a1 = w. Then a2 = w-a1 = 0.
#     k = a1*n + 0*w = w*n. Only multiples of n. (comp = 3 for this: rows = {1^n, 0^n, R2=0^n}? Hmm.)
#     Wait, a1 = w, a2 = 0, a3 = n-w. No R2 rows. Only 2 row types: 1^n and 0^n.
#     k = w*n. Comp = 2 or 3 depending on w.
#   (1^{a1},0^{n-a1}) = 0^n iff a1 = 0.
#     Then a2 = w, a3 = n-w.
#     k = 0*n + w*w = w^2. Comp = 3: rows = {(1^w,0^{n-w}), 0^n}, cols = {(1^w,0^{n-w}), 0^n}.
#     Union = {(1^w,0^{n-w}), 0^n}. Wait, that's comp 2. Already in comp-2 set!

# Option 3: first col = 0^n (a1+a2 = 0, a3 = n).
#   All rows are 0^n. k = 0. Trivial.

# So for 3 row types {1^n, (1^w,0^{n-w}), 0^n} in block arrangement:
# comp ≤ 3 requires specific constraints.

# The general idea: with 3 row types, we have more flexibility for the row patterns
# to absorb column patterns. But columns still depend on the arrangement.

# Let me just try ALL possible (a1, a2, a3, w) for n=20 and compute complexity.

def compute_min_comp_3types_block(n):
    """Min complexity using 3 row types: 1^n, (1^w,0^{n-w}), 0^n in block arrangement."""
    best = [999] * (n*n + 1)
    best[0] = 1
    best[n*n] = 1

    for w in range(1, n):  # weight of middle type
        for a1 in range(n+1):  # rows of all-ones
            for a2 in range(n+1-a1):  # rows of middle type
                a3 = n - a1 - a2  # rows of all-zeros
                k = a1 * n + a2 * w
                if k < 0 or k > n*n:
                    continue

                # Build matrix (block: a1 rows of 1^n, then a2 of R2, then a3 of 0^n)
                patterns = set()
                if a1 > 0: patterns.add((1,)*n)
                if a2 > 0: patterns.add((1,)*w + (0,)*(n-w))
                if a3 > 0: patterns.add((0,)*n)

                # Columns:
                # cols 0..w-1: R1 has 1, R2 has 1, R3 has 0.
                #   col = (1^{a1+a2}, 0^{a3})
                if w > 0:
                    patterns.add((1,)*(a1+a2) + (0,)*a3)
                # cols w..n-1: R1 has 1, R2 has 0, R3 has 0.
                #   col = (1^{a1}, 0^{a2+a3})
                if w < n:
                    patterns.add((1,)*a1 + (0,)*(n-a1))

                comp = len(patterns)
                best[k] = min(best[k], comp)

    return best


nn = 20
best_block = compute_min_comp_block(nn)
best_3type = compute_min_comp_3types_block(nn)
combined = [min(best_block[k], best_3type[k]) for k in range(nn*nn+1)]
total = sum(combined)
comp_gt3 = sum(1 for k in range(nn*nn+1) if combined[k] > 3)
print(f"C({nn}) upper bound (block + 3-type) = {total}")
print(f"k values still with comp > 3: {comp_gt3}")

# List them
gt3 = [k for k in range(nn*nn+1) if combined[k] > 3]
print(f"Values: {gt3[:50]}")
