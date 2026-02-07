#!/usr/bin/env python3
"""
Direct computation of c(n,k) using 2 row types with ANY σ.

For 2 row types (P, Q) and row assignment σ:
M[i][j] = P[j] if σ[i]=1 else Q[j]

k = sum(σ) * weight(P) + (n - sum(σ)) * weight(Q)
  = s * wp + (n-s) * wq

Complexity = |{row patterns used} ∪ {column patterns}|

Row patterns: {P} if σ=all-1, {Q} if σ=all-0, {P,Q} otherwise.
Col patterns: ⊆ {(P[j])^n, (Q[j])^n, σ, comp(σ)} depending on (P[j],Q[j]).

More precisely, col j has the pattern:
- (P[j])^n = P[j] repeated n times, if P[j]=Q[j] (all entries same).
  This is 0^n or 1^n.
- σ if P[j]=1, Q[j]=0
- comp(σ) if P[j]=0, Q[j]=1

The column patterns that appear:
col_pats = set()
if any j with P[j]=Q[j]=0: col_pats.add(0^n)     [present iff a > 0]
if any j with P[j]=Q[j]=1: col_pats.add(1^n)     [present iff b > 0]
if any j with P[j]=1,Q[j]=0: col_pats.add(σ)     [present iff c > 0]
if any j with P[j]=0,Q[j]=1: col_pats.add(comp(σ)) [present iff d > 0]

Row patterns: {P, Q} (assuming P ≠ Q, i.e., c+d > 0).
If σ = all-0: only Q used. If σ = all-1: only P used.

Complexity = |row_pats ∪ col_pats|.

Key: the abstract elements in the set are actual binary vectors.
0^n, 1^n, P, Q, σ, comp(σ) are all specific binary vectors.
Two of them are "the same" if they're equal as vectors.

For fixed (a, b, c, d) [column types]:
P and Q are specific vectors (determined by column type arrangement).
But we're free to choose the arrangement.

For fixed s (weight of σ), k is determined:
k = s*wp + (n-s)*wq = s*(b+c) + (n-s)*(b+d).

But the complexity depends on which coincidences exist among:
{P, Q, 0^n, 1^n, σ, comp(σ)}.

Two of these coincide iff they have the same weight AND we can arrange
column types and choose σ to make them identical as vectors.

P has weight wp = b+c. Q has weight wq = b+d.
0^n has weight 0. 1^n has weight n.
σ has weight s. comp(σ) has weight n-s.

For P = 0^n: wp = 0, so b=c=0. Then c+d > 0 requires d > 0.
For Q = 0^n: wq = 0, so b=d=0. Then c+d > 0 requires c > 0.
For P = 1^n: wp = n, so a=d=0.
For Q = 1^n: wq = n, so a=c=0.

For σ = P: need s = wp = b+c AND we can arrange σ to match P.
Since P has 1s at B and C column positions, σ = P means σ[i] = 1 iff
column position i is type B or C. We can choose which n positions are
types A,B,C,D freely, so we can set σ[i] = 1 for any s = b+c positions.
This is always achievable.

For σ = Q: need s = wq = b+d. Same logic, always achievable.
For σ = 0^n: s = 0.
For σ = 1^n: s = n.
For comp(σ) = P: need n-s = wp, i.e., s = n-wp = a+d. Achievable.
For comp(σ) = Q: need n-s = wq, i.e., s = n-wq = a+c. Achievable.

Additional: σ = comp(σ) is impossible (each bit = its complement).
P = Q is impossible when c+d > 0 (they differ at c+d positions).
P = comp(Q): P[j] = 1-Q[j] for all j. Since P has 1s at B,C and Q has 1s at B,D:
P[j] = 1-Q[j] means: at type A: 0 = 1-0 = 1, false. So P ≠ comp(Q) if a > 0.
If a = 0: at B: 1 = 1-1 = 0, false if b > 0.
If a = b = 0: P has 1s at C, Q has 1s at D. comp(Q) has 1s at C. P = comp(Q). ✓
So P = comp(Q) iff a = b = 0.

Now, for complexity computation:
Start with {P, Q} (2 elements).
Add 0^n if a > 0 and 0^n ∉ {P, Q}.
Add 1^n if b > 0 and 1^n ∉ {P, Q}.
Add σ if c > 0 and σ ∉ {P, Q, 0^n [if already in set], 1^n [if already in set]}.
Add comp(σ) if d > 0 and comp(σ) ∉ {existing elements}.

But σ is determined by s and the arrangement. For a given s:
σ can potentially coincide with P (if s = wp), Q (if s = wq), 0^n (if s = 0), 1^n (if s = n).
comp(σ) can coincide with P (if s = n-wp), Q (if s = n-wq), 0^n (if s = n), 1^n (if s = 0).

For MINIMUM complexity, we want to choose s to maximize coincidences.

Let me enumerate all (a,b,c,d,s) and compute exact complexity for each.
"""

def compute_min_complexity_all_k(n):
    """For each k, find minimum complexity using 2 row types with ANY σ weight."""
    best = [999] * (n*n + 1)
    best[0] = 1
    best[n*n] = 1

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c

                wp = b + c  # weight of P
                wq = b + d  # weight of Q

                for s in range(n+1):
                    k = s * wp + (n - s) * wq
                    if k < 0 or k > n*n:
                        continue

                    # Compute complexity
                    # Start with counting distinct elements in the set
                    # {P, Q, [0^n if needed], [1^n if needed], [σ if needed], [comp(σ) if needed]}

                    # Use weight-based uniqueness (two elements are same iff same weight
                    # AND we can arrange to make them identical)

                    # Possible elements and their weights:
                    # P: wp
                    # Q: wq
                    # 0^n: 0
                    # 1^n: n
                    # σ: s
                    # comp(σ): n-s

                    # Elements needed:
                    elements = set()
                    if c + d > 0:
                        elements.add(('P', wp))
                        elements.add(('Q', wq))
                    else:
                        elements.add(('P', wp))  # P = Q

                    if a > 0:
                        elements.add(('zero', 0))
                    if b > 0:
                        elements.add(('one', n))
                    if c > 0:
                        elements.add(('sigma', s))
                    if d > 0:
                        elements.add(('comp', n-s))

                    # Now merge elements with the same weight (they can be made identical
                    # by arrangement choice)
                    # Wait, this isn't quite right. Two elements with the same weight
                    # CAN be made identical only if the arrangement allows it.

                    # Actually, elements are:
                    # P, Q: determined by column type arrangement (which positions are A,B,C,D)
                    # 0^n, 1^n: fixed vectors
                    # σ: determined by row assignment (which rows use P)
                    # comp(σ): complement of σ

                    # We can choose BOTH the column arrangement AND the row assignment.
                    # Given (a,b,c,d) and s:
                    # - P and Q are determined up to permutation of columns.
                    #   By choosing column arrangement, we can make P = any vector
                    #   with weight wp and having specific structure (wp = b+c ones,
                    #   with b of them at positions where Q also has 1).
                    # - σ is any vector with weight s.
                    #   We choose σ to maximize coincidences.

                    # For σ to equal P: need s = wp AND σ has 1s at the same positions as P.
                    #   Since we choose both the column arrangement (determining P's positions)
                    #   and σ, we can make σ = P by setting the column arrangement such that
                    #   P's 1-positions are exactly σ's 1-positions. Since both have the same
                    #   weight (s = wp), this is possible.

                    # For σ to equal Q: need s = wq. By similar argument, possible.
                    # For σ = 0^n: s = 0.
                    # For σ = 1^n: s = n.
                    # For comp(σ) = P: s = n - wp. We arrange so comp(σ) has 1s at P's positions.
                    # For comp(σ) = Q: s = n - wq.

                    # Key: we can only satisfy ONE of these at a time (for σ) and ONE for comp(σ).
                    # But some might be simultaneously satisfiable.

                    # σ = P and comp(σ) = Q: need s = wp and n-s = wq, i.e., wp+wq = n.
                    # σ = P and comp(σ) = 1^n: need s = wp and n-s = n, i.e., s = 0. So wp = 0.
                    # etc.

                    # Also: σ = 0^n: s=0, comp(σ) = 1^n. Both determined.
                    # σ = 1^n: s=n, comp(σ) = 0^n.

                    # Let me enumerate the possible coincidences for σ and comp(σ):
                    # For each possible "assignment" of σ to one of {P,Q,0^n,1^n,<new>}
                    # and comp(σ) to one of {P,Q,0^n,1^n,<new>}:

                    # The "identities" we can assign:
                    sigma_options = set()  # what σ can be equal to
                    comp_options = set()   # what comp(σ) can be equal to

                    if s == wp:
                        sigma_options.add('P')
                    if s == wq:
                        sigma_options.add('Q')
                    if s == 0:
                        sigma_options.add('zero')
                    if s == n:
                        sigma_options.add('one')

                    ns = n - s
                    if ns == wp:
                        comp_options.add('P')
                    if ns == wq:
                        comp_options.add('Q')
                    if ns == 0:
                        comp_options.add('zero')
                    if ns == n:
                        comp_options.add('one')

                    # Also σ can be a new pattern (not equal to any existing):
                    sigma_options.add('new_s')
                    comp_options.add('new_c')

                    # Try all combinations
                    min_comp = 999
                    for so in sigma_options:
                        for co in comp_options:
                            # Check compatibility: can σ and comp(σ) have these identities simultaneously?
                            # If so = 'P' and co = 'Q': need wp + wq = n (so comp(P) = Q).
                            # If so = 'P' and co = 'P': need wp = n-s and s = wp, so n-s = wp, 2*wp = n.
                            #   Also means comp(σ) = P and σ = P, so comp(P) = P => P = comp(P) => all entries 0.5? No.
                            #   Actually P is a binary vector. comp(P) = P means every bit equals its complement. Impossible.
                            #   So so='P' and co='P' requires σ = P and comp(σ) = P, impossible unless P = comp(P).
                            #   P = comp(P) only if n = 0. Not possible.

                            # Let me just check weight compatibility.
                            # σ has weight s, comp(σ) has weight n-s.
                            # If so identifies σ with a known element of weight w_s,
                            # and co identifies comp(σ) with a known element of weight w_c,
                            # we need w_s + w_c = n.

                            w_s = {'P': wp, 'Q': wq, 'zero': 0, 'one': n, 'new_s': s}
                            w_c = {'P': wp, 'Q': wq, 'zero': 0, 'one': n, 'new_c': n-s}

                            if w_s[so] + w_c[co] != n:
                                continue  # Weights don't add up

                            # Also σ ≠ comp(σ) (always true for n > 0 since no binary vector = its complement).
                            # So so ≠ co unless they refer to different underlying vectors.
                            # If so = 'P' and co = 'P': σ = P and comp(σ) = P means σ = comp(σ) = P. But σ ≠ comp(σ). Contradiction.
                            if so == co and so in ('P', 'Q', 'zero', 'one'):
                                continue  # σ = comp(σ) impossible

                            # Check arrangement compatibility:
                            # Can we arrange column types and σ so that:
                            # σ matches 'so' and comp(σ) matches 'co'?

                            # For σ = P: σ has 1s at B,C positions (weight b+c = wp).
                            # For comp(σ) = Q: comp(σ) has 1s at B,D positions (weight b+d = wq).
                            #   So σ has 0s at B,D positions, 1s at A,C positions.
                            #   σ has 1s at B,C AND 1s at A,C? No, σ is determined by the row assignment.

                            # Hmm, the issue is that σ = P requires σ to have 1s at specific positions
                            # (B and C column positions), but comp(σ) = Q requires σ to have specific
                            # 0s at B,D positions.

                            # σ = P: σ[i] = 1 iff position i is type B or C.
                            # comp(σ) = Q: comp(σ)[i] = Q[i]. Q[i] = 1 iff type B or D.
                            # comp(σ)[i] = 1 - σ[i].
                            # So 1 - σ[i] = Q[i], σ[i] = 1 - Q[i].
                            # σ[i] = 1 iff Q[i] = 0 iff position i is type A or C.

                            # But σ = P says σ[i] = 1 iff type B or C.
                            # And comp(σ) = Q says σ[i] = 1 iff type A or C.
                            # Both: σ[i] = 1 iff (type B or C) AND (type A or C) = type C.
                            # So σ[i] = 1 iff position i is type C. Weight = c.
                            # But σ = P has weight wp = b+c. So c = b+c, meaning b = 0.
                            # And comp(σ) = Q has weight n-(b+c)= a+d... actually wq = b+d.
                            # Weight of comp(σ) = n - c = a + b + d = a + d (since b=0).
                            # Need this = wq = b + d = d. So a + d = d, a = 0.
                            # So we need a = b = 0. Then n = c + d.

                            # This is consistent with P = comp(Q) iff a = b = 0 (derived earlier).

                            # So the weight check (wp + wq = n) is necessary but we also need
                            # to check the arrangement consistency.

                            # For the general case, arrangement compatibility is subtle.
                            # Let me just check: for so ∈ {P,Q,zero,one} and co ∈ {P,Q,zero,one}:
                            # The arrangement must satisfy BOTH σ-identity and comp(σ)-identity.

                            # I'll skip the detailed compatibility check for now and just
                            # record the OPTIMISTIC complexity (minimum possible).
                            # This gives a LOWER BOUND on achievable complexity.

                            # Count distinct patterns
                            pattern_set = set()
                            # Always have P and Q (if c+d > 0)
                            if c + d > 0:
                                pattern_set.add('P')
                                pattern_set.add('Q')
                            else:
                                pattern_set.add('P')

                            # Add 0^n if needed
                            if a > 0:
                                pattern_set.add('zero')
                            # Add 1^n if needed
                            if b > 0:
                                pattern_set.add('one')
                            # Add σ if needed
                            if c > 0:
                                pattern_set.add(so if so != 'new_s' else 'sigma')
                            # Add comp(σ) if needed
                            if d > 0:
                                pattern_set.add(co if co != 'new_c' else 'comp_sigma')

                            comp = len(pattern_set)
                            if comp < min_comp:
                                min_comp = comp

                    if min_comp < best[k]:
                        best[k] = min_comp

    return best


# Test for small n first
for n in [3, 4, 5]:
    best = compute_min_complexity_all_k(n)
    total = sum(best[k] for k in range(n*n+1))
    print(f"C({n}) = {total} (expected: n=3:22, n=4:38, n=5:64)")
    for k in range(n*n+1):
        if best[k] > 3:
            print(f"  c({n},{k}) = {best[k]}")

# Test for n=10
n = 10
best = compute_min_complexity_all_k(n)
total = sum(best[k] for k in range(n*n+1))
print(f"C({n}) = {total} (expected: 274)")

# Test for n=20
n = 20
best = compute_min_complexity_all_k(n)
total = sum(best[k] for k in range(n*n+1))
print(f"C({n}) = {total} (expected: 1150)")

# Count distribution
from collections import Counter
dist = Counter(best[k] for k in range(n*n+1))
print(f"Distribution: {dict(sorted(dist.items()))}")
