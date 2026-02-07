#!/usr/bin/env python3
"""
Thorough search for minimum complexity.

For each k value, try all possible 2-row-type and 3-row-type matrices.
For 2-row-type: try all (P, Q) patterns and all possible row permutations.
Since row permutation only affects column structure, and columns depend on
which rows use which type, we enumerate all possible σ vectors.

For efficiency, note that:
- Two matrices with the same (P, Q) and same σ-weight give the same k.
- But different σ with the same weight can give different column patterns.
- However, for 2 row types, σ just determines how the rows are assigned.
  Column j = (P[j] if σ[i] else Q[j] for i).
  If P[j]=Q[j], column j is constant regardless of σ.
  If P[j]≠Q[j], column j has 1s where σ[i]=1 if P[j]=1,Q[j]=0 (column = σ)
                 or 1s where σ[i]=0 if P[j]=0,Q[j]=1 (column = complement(σ)).
  So all columns are {all-zeros, all-ones, σ, complement(σ)} (a subset).
  Plus row patterns {P, Q}.
  Complexity = |{P, Q} ∪ {column patterns}|.

So for fixed P, Q, σ:
k = |σ| * |P| + (n-|σ|) * |Q|  (depends only on weights)
complexity = |{P, Q} ∪ column_patterns|

The column patterns are:
- all-zeros (0^n) if any j has P[j]=Q[j]=0
- all-ones (1^n) if any j has P[j]=Q[j]=1
- σ if any j has P[j]=1,Q[j]=0
- complement(σ) if any j has P[j]=0,Q[j]=1

So the union is:
{P, Q} ∪ {0^n if a>0} ∪ {1^n if b>0} ∪ {σ if c>0} ∪ {complement(σ) if d>0}

where a=#{j: P[j]=0,Q[j]=0}, b=#{j: P[j]=1,Q[j]=1}, c=#{j: P[j]=1,Q[j]=0}, d=#{j: P[j]=0,Q[j]=1}.

The maximum number of elements in this set is 6 (P, Q, 0^n, 1^n, σ, comp(σ)).
But many can coincide.

For a given (a,b,c,d), P and Q are determined up to permutation:
P = (0^a, 1^b, 1^c, 0^d) as a type-vector.
Q = (0^a, 1^b, 0^c, 1^d).

But σ can be ANY binary vector of length n. And different σ give different
column patterns (σ and comp(σ)), which changes what's in the union.

So for fixed (a,b,c,d), the complexity depends on σ through which of
{0^n, 1^n, σ, comp(σ)} coincide with P or Q.

This is subtle because σ is a length-n vector, and P is also a length-n vector,
but they represent different things (σ = row assignment, P = column pattern).
However, as abstract binary vectors they can be compared.

So complexity = |{P, Q} ∪ S| where S ⊆ {0^n, 1^n, σ, comp(σ)}.

The size of this set depends on:
1. Which elements of S are present (depends on a,b,c,d being >0)
2. Which coincidences exist among {P, Q, 0^n, 1^n, σ, comp(σ)}

For P and Q:
P = (0^a, 1^{b+c}, 0^d) [as a pattern, up to arrangement]
Actually, P has weight b+c and Q has weight b+d.

P = 0^n iff b+c = 0 iff b=c=0
P = 1^n iff a+d = 0 iff a=d=0
Q = 0^n iff b+d = 0 iff b=d=0
Q = 1^n iff a+c = 0 iff a=c=0
P = σ iff they match as vectors (depends on arrangement)
Q = σ iff they match as vectors

The key insight: we're free to choose both:
1. The arrangement of column types (which positions are A, B, C, D)
2. The σ vector (which rows use P vs Q)

And we want to minimize complexity = |{P, Q} ∪ S|.

This means we should try to maximize overlaps.

Let me enumerate: for each (a,b,c,d) with a+b+c+d=n and (c+d)>0:
and for each σ of weight s:
Compute the set {P, Q, 0^n if a>0, 1^n if b>0, σ if c>0, comp(σ) if d>0}
and count distinct elements.

But σ can be any vector! However, we can choose σ to maximize overlaps.
The possible overlaps between σ and {P, Q, 0^n, 1^n}:
- σ = P: need s = b+c and arrangement such that σ has 1s at B,C positions
- σ = Q: need s = b+d and arrangement such that σ has 1s at B,D positions
- σ = 0^n: s = 0
- σ = 1^n: s = n
- comp(σ) = P: need n-s = b+c, i.e., s = a+d
- comp(σ) = Q: need n-s = b+d, i.e., s = a+c
- comp(σ) = 0^n: s = n
- comp(σ) = 1^n: s = 0

For minimum complexity, we want σ to match as many things as possible.
But k = s*(b+c) + (n-s)*(b+d) is fixed for given (a,b,c,d,s).

Let me just enumerate all cases computationally for given n.
"""

def compute_min_complexity_all_k(n):
    """For each k in 0..n^2, find minimum complexity."""
    best = [float('inf')] * (n*n + 1)
    best[0] = 1
    best[n*n] = 1

    # For each (a,b,c,d) decomposition
    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c

                # For each weight s of σ
                for s in range(n+1):
                    k = s * (b + c) + (n - s) * (b + d)
                    if k < 0 or k > n*n:
                        continue

                    # Determine which auxiliary patterns exist
                    aux = set()
                    if a > 0:
                        aux.add('zero')
                    if b > 0:
                        aux.add('one')
                    if c > 0:
                        aux.add('sigma')
                    if d > 0:
                        aux.add('comp_sigma')

                    # P has weight b+c, Q has weight b+d
                    wp = b + c
                    wq = b + d

                    # Try to find σ that maximizes overlaps.
                    # The base set is {P, Q} (size 2 if P ≠ Q, else 1).
                    # P=Q iff c=d=0, which means no aux sigma or comp_sigma.
                    # If c=d=0 but a>0 or b>0, P=Q but there are constant columns.

                    if c == 0 and d == 0:
                        # P = Q. Complexity = |{P} ∪ {0 if a>0} ∪ {1 if b>0}|
                        # P = (0^a, 1^b) with weight b.
                        base = {('P', wp)}
                        if a > 0:
                            base.add(('zero', 0))
                        if b > 0:
                            base.add(('one', n))
                        # Now check if P coincides with 0^n or 1^n
                        if wp == 0:  # P = 0^n
                            base.discard(('zero', 0))
                            base.add(('P', 0))
                        if wp == n:  # P = 1^n
                            base.discard(('one', n))
                            base.add(('P', n))
                        comp = len(base)
                        best[k] = min(best[k], comp)
                        continue

                    # P ≠ Q. Base set has {P, Q}.
                    # Additional: {0^n, 1^n, σ, comp(σ)} depending on aux.
                    # We want to choose σ to maximize overlap with {P, Q, 0^n, 1^n}.

                    # The patterns and their weights:
                    # P: weight wp = b+c
                    # Q: weight wq = b+d
                    # 0^n: weight 0
                    # 1^n: weight n
                    # σ: weight s
                    # comp(σ): weight n-s

                    # Two patterns can coincide only if they have the same weight.
                    # And for σ, we can choose the arrangement to match any specific
                    # pattern with the right weight.

                    # So σ can match P if s = wp (and we arrange σ accordingly).
                    # σ can match Q if s = wq.
                    # σ can match 0^n if s = 0.
                    # σ can match 1^n if s = n.
                    # Similarly comp(σ) can match P if n-s = wp, i.e., s = n-wp.
                    # comp(σ) can match Q if n-s = wq, i.e., s = n-wq.
                    # comp(σ) can match 0^n if n-s = 0, i.e., s = n.
                    # comp(σ) can match 1^n if n-s = n, i.e., s = 0.

                    # Build the set of patterns as "abstract" items:
                    items = {'P', 'Q'}  # always present

                    # Add required items
                    if a > 0:
                        items.add('zero')
                    if b > 0:
                        items.add('one')
                    if c > 0:
                        items.add('sigma')
                    if d > 0:
                        items.add('comp_sigma')

                    # Now find coincidences:
                    # P = zero iff wp = 0
                    # P = one iff wp = n
                    # Q = zero iff wq = 0
                    # Q = one iff wq = n
                    # sigma = P iff s = wp (and arrangement matches)
                    # sigma = Q iff s = wq
                    # sigma = zero iff s = 0
                    # sigma = one iff s = n
                    # comp_sigma = P iff s = n - wp
                    # comp_sigma = Q iff s = n - wq
                    # comp_sigma = zero iff s = n
                    # comp_sigma = one iff s = 0

                    # Build equivalence classes
                    # Use union-find or just compute the distinct count

                    # Map each abstract item to its "identity" (weight, plus whether it's a row/col pattern)
                    # Actually, two items are the same pattern iff:
                    # - They have the same weight, AND
                    # - We can arrange things so they literally have the same bit pattern.

                    # For the arrangement to work:
                    # σ = P means σ has 1s at exactly the B and C column positions.
                    #   This requires s = b+c = wp.
                    # σ = Q means σ has 1s at exactly the B and D column positions.
                    #   This requires s = b+d = wq.
                    # σ = 0^n means s = 0 (all rows use Q).
                    # σ = 1^n means s = n (all rows use P).

                    # comp(σ) = P means comp(σ) has 1s at B,C positions, so σ has 1s at A,D positions.
                    #   Requires s = a+d = n-wp.
                    # comp(σ) = Q means comp(σ) has 1s at B,D positions, so σ has 1s at A,C positions.
                    #   Requires s = a+c = n-wq.

                    # But we can only pick ONE σ! So we can satisfy at most one of:
                    # σ=P, σ=Q, σ=0, σ=1, comp(σ)=P, comp(σ)=Q, comp(σ)=0, comp(σ)=1
                    # Actually we might satisfy two simultaneously if e.g. σ=P and comp(σ)=Q
                    # which requires s=wp and n-s=wq, i.e., wp+wq=n.

                    # Let's enumerate all possible σ choices (characterized by which items match σ):
                    possible_sigma_matchings = []

                    # σ matches nothing special:
                    possible_sigma_matchings.append(set())

                    # σ = P:
                    if s == wp:
                        m = {'sigma_is_P'}
                        if n - s == wq:
                            m.add('comp_is_Q')
                        if n - s == 0:
                            m.add('comp_is_zero')
                        if n - s == n:
                            m.add('comp_is_one')
                        if n - s == wp:
                            m.add('comp_is_P')
                        possible_sigma_matchings.append(m)

                    # σ = Q:
                    if s == wq:
                        m = {'sigma_is_Q'}
                        if n - s == wp:
                            m.add('comp_is_P')
                        if n - s == 0:
                            m.add('comp_is_zero')
                        if n - s == n:
                            m.add('comp_is_one')
                        if n - s == wq:
                            m.add('comp_is_Q')
                        possible_sigma_matchings.append(m)

                    # σ = 0^n:
                    if s == 0:
                        m = {'sigma_is_zero'}
                        if n == wp:
                            m.add('comp_is_P')
                        if n == wq:
                            m.add('comp_is_Q')
                        m.add('comp_is_one')
                        possible_sigma_matchings.append(m)

                    # σ = 1^n:
                    if s == n:
                        m = {'sigma_is_one'}
                        if 0 == wp:
                            m.add('comp_is_P')
                        if 0 == wq:
                            m.add('comp_is_Q')
                        m.add('comp_is_zero')
                        possible_sigma_matchings.append(m)

                    # comp(σ) = P (σ at A,D positions):
                    if s == n - wp:
                        m = {'comp_is_P'}
                        if s == wp:
                            m.add('sigma_is_P')
                        if s == wq:
                            m.add('sigma_is_Q')
                        if s == 0:
                            m.add('sigma_is_zero')
                        if s == n:
                            m.add('sigma_is_one')
                        possible_sigma_matchings.append(m)

                    # comp(σ) = Q:
                    if s == n - wq:
                        m = {'comp_is_Q'}
                        if s == wp:
                            m.add('sigma_is_P')
                        if s == wq:
                            m.add('sigma_is_Q')
                        if s == 0:
                            m.add('sigma_is_zero')
                        if s == n:
                            m.add('sigma_is_one')
                        possible_sigma_matchings.append(m)

                    # For each possible σ matching, compute distinct patterns count
                    for matching in possible_sigma_matchings:
                        # Start with {P, Q}
                        distinct = {'P', 'Q'}

                        # Add zero if needed
                        if a > 0:
                            if 'P' in distinct and wp == 0:
                                pass  # zero = P
                            elif 'Q' in distinct and wq == 0:
                                pass  # zero = Q
                            else:
                                distinct.add('zero')

                        # Add one if needed
                        if b > 0:
                            if wp == n:
                                pass  # one = P
                            elif wq == n:
                                pass  # one = Q
                            else:
                                distinct.add('one')

                        # Add sigma if needed
                        if c > 0:
                            if 'sigma_is_P' in matching:
                                pass
                            elif 'sigma_is_Q' in matching:
                                pass
                            elif 'sigma_is_zero' in matching and 'zero' not in distinct and (wp == 0 or wq == 0):
                                pass
                            elif 'sigma_is_zero' in matching:
                                if a > 0:
                                    pass  # zero already in distinct or coincides
                                else:
                                    distinct.add('sigma')  # sigma = 0^n but 0^n not in set
                            elif 'sigma_is_one' in matching:
                                if b > 0:
                                    pass
                                else:
                                    distinct.add('sigma')
                            else:
                                distinct.add('sigma')

                        # Add comp_sigma if needed
                        if d > 0:
                            if 'comp_is_P' in matching:
                                pass
                            elif 'comp_is_Q' in matching:
                                pass
                            elif 'comp_is_zero' in matching:
                                if a > 0 or wp == 0 or wq == 0:
                                    pass  # 0^n is already represented
                                else:
                                    distinct.add('comp_sigma')
                            elif 'comp_is_one' in matching:
                                if b > 0 or wp == n or wq == n:
                                    pass
                                else:
                                    distinct.add('comp_sigma')
                            else:
                                # Check if comp_sigma = sigma (only if σ = comp(σ), i.e., impossible for binary)
                                distinct.add('comp_sigma')

                        best[k] = min(best[k], len(distinct))

    return best


# This is getting overly complicated. Let me just do a direct search.
# For n up to ~8, enumerate (P, Q, σ) triples.
# P, Q are represented by (weight_P, weight_Q, a, b, c, d).
# σ is represented by its weight s.
# But we need to track the actual patterns for coincidence checking.

# Simpler approach: for each n and k, try ALL (P, Q, σ) with
# P, Q ∈ {0,1}^n, σ ∈ {0,1}^n, and M[i][j] = P[j] if σ[i] else Q[j].
# Count k and complexity.

# For n=8: 256^3/2 = 8M triples. Should run in reasonable time.

import itertools

def compute_cnk_direct(n, max_patterns=2):
    """Find min complexity for each k using up to max_patterns row types."""
    best = {}

    all_vecs = list(itertools.product([0,1], repeat=n))

    if max_patterns == 2:
        for idx_p, P in enumerate(all_vecs):
            for Q in all_vecs[idx_p:]:  # P <= Q
                for sigma in all_vecs:
                    # Build matrix
                    k = 0
                    col_sums = {}
                    patterns = set()
                    patterns.add(P)
                    patterns.add(Q)

                    valid = True
                    for j in range(n):
                        col = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                        patterns.add(col)
                        k += sum(col)

                    comp = len(patterns)
                    if k not in best or comp < best[k]:
                        best[k] = comp

    return best


# For n=5, do direct computation
for n in [5, 6]:
    print(f"\nn={n}:")
    best = compute_cnk_direct(n, 2)
    total = 0
    for k in range(n*n+1):
        c = best.get(k, n+n)
        total += c
        if c > 3:
            print(f"  c({n},{k}) = {c}  <-- greater than 3!")
    print(f"  C({n}) upper bound (2 row types) = {total}")
