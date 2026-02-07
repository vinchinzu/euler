#!/usr/bin/env python3
"""
Try to achieve comp ≤ 3 for all k using 2-row-type non-block arrangements.

For 2 row types R1 = (1^w1, 0^{n-w1}) and R2 = (1^w2, 0^{n-w2}) with w1 > w2:
The column at positions w2..w1-1 is σ (the row assignment vector).
The column at positions 0..w2-1 is 1^n.
The column at positions w1..n-1 is 0^n.

For comp ≤ 3: we need |{R1, R2, 1^n, 0^n, σ}| ≤ 3.
(Some might not be present if w2=0 or w1=n.)

For s ∉ {0, w2, w1, n}: σ is a new pattern, giving comp ≥ 4.

KEY IDEA: Instead of using R1 and R2 as left-aligned blocks of 1s,
use ARBITRARY binary patterns for row types!

R1 can be ANY binary vector of weight w1.
R2 can be ANY binary vector of weight w2.

This gives much more freedom for σ to coincide with row patterns.

Actually, the fundamental setup is:
P and Q are two binary vectors (not necessarily left-aligned).
σ is the row assignment.

Column j depends on P[j], Q[j], and σ:
- If P[j] = Q[j]: col j is constant.
- If P[j] ≠ Q[j]: col j is σ or comp(σ).

For comp ≤ 3 with arbitrary P, Q, σ:
Union = {P, Q} ∪ {constant cols} ∪ {σ or comp(σ) if applicable}

For the constant columns:
- 0^n if any j with P[j]=Q[j]=0 (type A positions)
- 1^n if any j with P[j]=Q[j]=1 (type B positions)

For variable columns:
- σ if any j with P[j]=1,Q[j]=0 (type C)
- comp(σ) if any j with P[j]=0,Q[j]=1 (type D)

For comp ≤ 3, |{P,Q,0^n[if a>0],1^n[if b>0],σ[if c>0],comp(σ)[if d>0]}| ≤ 3.

Since |{P,Q}| = 2, at most 1 other distinct element.

Possible configurations for comp = 3:
One new element among {0^n, 1^n, σ, comp(σ)}, all others coincide with P or Q.

This means:
(i) All needed elements except one are in {P,Q}. The remaining one is the new element.

Let me enumerate for EACH possible "new" element:

Case 1: new element is 0^n.
Required: a > 0, 0^n ∉ {P,Q}.
And: 1^n ∈ {P,Q} if b > 0.
And: σ ∈ {P,Q} if c > 0.
And: comp(σ) ∈ {P,Q} if d > 0.

Sub-case 1a: b=0 (no 1^n needed). Then only σ and comp(σ) need to be in {P,Q}.
If c > 0 and d > 0: need σ ∈ {P,Q} and comp(σ) ∈ {P,Q} and Q = comp(P) (as before).
If c > 0 and d = 0: need σ ∈ {P,Q}. s ∈ {wp, wq}.
If c = 0 and d > 0: need comp(σ) ∈ {P,Q}. n-s ∈ {wp, wq}.

Sub-case 1b: b > 0, 1^n ∈ {P,Q}.
1^n = P: wp = n, so a=d=0. But a > 0 (needed for 0^n). Contradiction.
1^n = Q: wq = n, so a=c=0. But a > 0. Contradiction (c=0 is OK though).
Wait, a=0 contradicts a > 0. So sub-case 1b is impossible if a > 0 and b > 0
(need both 0^n and 1^n, but 1^n ∈ {P,Q} requires a=0 or a=0 depending).

Actually 1^n = P requires a + d = 0, so a = d = 0. But a > 0 is required. Contradiction.
1^n = Q requires a + c = 0, so a = c = 0. But a > 0 required. Contradiction.
So Case 1 with b > 0 is impossible.

Case 1, b=0, c > 0, d = 0:
0^n ∉ {P,Q}: wp = b+c = c > 0 and wq = b+d = 0. But wq = 0 means Q = 0^n.
0^n = Q. But we said 0^n ∉ {P,Q}. Contradiction!

Wait, wq = b+d = 0+0 = 0, so Q = 0^n. Then 0^n = Q ∈ {P,Q}. So 0^n IS in {P,Q}!
This contradicts our assumption that 0^n is the "new" element.

Let me reconsider. With b = 0 and d = 0: wq = 0, so Q = 0^n. Then 0^n ∈ {P,Q}.
So the "new" element cannot be 0^n when d = 0 and b = 0.

Case 1 requires a > 0 and 0^n ∉ {P,Q}. 0^n ∉ {P,Q} means wp > 0 and wq > 0,
i.e., b+c > 0 and b+d > 0.

Since b = 0 in sub-case 1a: c > 0 and d > 0. But then we need σ ∈ {P,Q} and comp(σ) ∈ {P,Q}
AND Q = comp(P). With a = b = 0: n = c+d.

wp = c, wq = d. P = (1^c, 0^d), Q = (0^c, 1^d).
comp(P) = (0^c, 1^d) = Q. ✓

σ = P: s = c. comp(σ) = Q. Both in {P,Q}. ✓
k = c*c + d*d = c^2 + (n-c)^2.

But 0^n ∉ {P,Q} since wp = c > 0 and wq = d > 0. ✓
And a > 0? We said a = b = 0. But a > 0 is required! Contradiction!

OK so actually a = b = 0 means a = 0, which contradicts a > 0.

So sub-case 1a (b=0, a>0, c>0, d>0) requires a > 0 with 0^n ∉ {P,Q} (wp > 0, wq > 0).
And Q = comp(P) (for σ,comp(σ) ∈ {P,Q}).
Q = comp(P) requires a = b (shown before). With b = 0: a = 0. Contradiction.

So case 1 seems impossible in all sub-cases!

Hmm. Let me reconsider.

Actually, I think the issue is that c > 0 and d > 0 doesn't require Q = comp(P)
when we're allowing comp 3. We need σ ∈ {P,Q} OR σ is the new element.
But in Case 1, the new element is 0^n, not σ.

So σ must ∈ {P,Q} even though there's a new element 0^n.
Similarly comp(σ) must ∈ {P,Q}.

OK so with c>0 and d>0 and σ ∈ {P,Q} and comp(σ) ∈ {P,Q}: Q = comp(P).

But b = 0, a > 0, Q = comp(P): requires a = b = 0 (a = 0). Contradiction.

What if only c > 0 (d = 0)?
Then comp(σ) is not needed.
σ ∈ {P,Q}. s ∈ {wp, wq}.

a > 0, b = 0, c > 0, d = 0.
wp = c, wq = 0. Q = 0^n. But 0^n = Q ∈ {P,Q}. So 0^n is not new! Contradiction.

What if c = 0, d > 0?
comp(σ) ∈ {P,Q}. n-s ∈ {wp, wq}.
a > 0, b = 0, c = 0, d > 0.
wp = 0. P = 0^n. 0^n = P ∈ {P,Q}. Not new! Contradiction.

So CASE 1 is truly impossible for b = 0!

And we showed Case 1 impossible for b > 0.

So comp = 3 with "new = 0^n" is impossible.

By symmetry, comp = 3 with "new = 1^n" is also impossible.

Case 3: new element is σ.
Required: c > 0, σ ∉ {P,Q}.
And: 0^n ∈ {P,Q} if a > 0.
And: 1^n ∈ {P,Q} if b > 0.
And: comp(σ) ∈ {P,Q} if d > 0.

Case 4: new element is comp(σ).
Required: d > 0, comp(σ) ∉ {P,Q}.
Similar to Case 3 by symmetry (swap P↔Q, swap c↔d).

So the only way to get comp = 3 is if the new element is σ or comp(σ).

Let me work out Case 3 in detail.

c > 0, σ ∉ {P,Q} (so s ≠ wp and s ≠ wq).
a > 0 => 0^n ∈ {P,Q}: wp = 0 or wq = 0.
b > 0 => 1^n ∈ {P,Q}: wp = n or wq = n.
d > 0 => comp(σ) ∈ {P,Q}: n-s = wp or n-s = wq.

Sub-case 3a: d = 0. No comp(σ) constraint.
a > 0 => 0^n ∈ {P,Q}. b > 0 => 1^n ∈ {P,Q}.

With d = 0: wq = b. wp = b+c.
0^n = P iff wp = 0 iff b = c = 0. But c > 0. So 0^n ≠ P.
0^n = Q iff wq = 0 iff b = 0.
1^n = P iff wp = n iff a = 0 (since d = 0, a = n - b - c).
1^n = Q iff wq = n iff a + c = 0 iff a = c = 0. But c > 0.

So:
If a > 0 and b > 0: need 0^n ∈ {P,Q} and 1^n ∈ {P,Q}.
  0^n ∈ {P,Q}: only if b = 0 (Q = 0^n). But b > 0. Contradiction.

If a > 0 and b = 0: need 0^n ∈ {P,Q}. wq = 0, so Q = 0^n. ✓
  wp = c. n = a + c.
  σ ∉ {P,Q}: s ≠ c and s ≠ 0.
  k = s*c + (n-s)*0 = s*c. With s ∈ {1,...,n}\{c} (s ≠ 0 and s ≠ c).
  k = s*c where 1 ≤ s ≤ n, s ≠ c.
  Union: {P=(1^c,0^a), Q=0^n, 0^n=Q, σ} = {P, Q, σ} = comp 3.

  k values: s*c for s = 1,...,c-1,c+1,...,n.
  But also s ≤ n and k ≤ n^2-1.
  s*c ranges: c, 2c, ..., (c-1)*c, (c+1)*c, ..., n*c.
  = multiples of c in range [c, nc] except c^2.

  Wait, s=c gives k=c^2 which is comp 2. So comp 3 gives k = s*c for s≠c, s≠0, s≤n.

If a = 0 and b > 0: need 1^n ∈ {P,Q}. wp = n (since a=0, d=0, n=b+c).
  P = 1^n. ✓
  wq = b.
  σ ∉ {P,Q}: s ≠ n and s ≠ b.
  k = s*n + (n-s)*b = sn + nb - sb = n(s+b) - sb = nb + s(n-b) = nb + sc.
  With s ∈ {0,...,n}\{n,b}.
  k = nb + sc for s ∈ {0,1,...,n}\{n,b}.
  = multiples: k = nb + sc.
  For s=0: k = nb. For s=1: k = nb+c. ... For s=n: k = nb+nc = n(b+c) = n^2.

Sub-case 3b: d > 0. comp(σ) ∈ {P,Q}: n-s = wp or n-s = wq.
  n-s = wp: s = n-wp = a+d.
  n-s = wq: s = n-wq = a+c.

Also a > 0 => 0^n ∈ {P,Q}, b > 0 => 1^n ∈ {P,Q}.

This is getting very detailed. Let me just compute it programmatically.
"""

def compute_comp3_k_values(n):
    """Find all k values achievable with comp ≤ 3 using 2 row types."""
    achievable = set()

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c

                wp = b + c
                wq = b + d

                if c == 0 and d == 0:
                    # P = Q. Single row type.
                    k = n * b
                    if 0 <= k <= n*n:
                        achievable.add(k)
                    continue

                # 2 row types, P ≠ Q.
                for s in range(n+1):
                    k = s * wp + (n - s) * wq
                    if k <= 0 or k >= n*n:
                        continue

                    # Compute exact complexity for this (a,b,c,d,s).
                    # Union: start with {P (wt wp), Q (wt wq)}.

                    # needed additions:
                    additions = set()
                    if a > 0:
                        additions.add(('zero', 0))
                    if b > 0:
                        additions.add(('one', n))
                    if c > 0:
                        additions.add(('sigma', s))
                    if d > 0:
                        additions.add(('comp', n-s))

                    # Merge with {P, Q}: check weight matches.
                    # An addition (name, wt) merges with P if wt = wp,
                    # or with Q if wt = wq.
                    # BUT: two additions can merge with each other if same weight
                    # AND compatible arrangement.

                    # Actually: merging means "can be made identical as vectors."
                    # For sigma (wt s) and P (wt wp): s = wp allows σ = P by arrangement.
                    # For sigma and zero (wt 0): s = 0 makes σ = 0^n.
                    # For sigma and comp: impossible (σ ≠ comp(σ)).
                    # For comp (wt n-s) and P: n-s = wp allows comp(σ) = P by arrangement.
                    # For comp and zero: n-s = 0, s = n.
                    # For comp and one: n-s = n, s = 0.
                    # For zero and one: weight 0 ≠ n (for n > 0).
                    # For zero and P: wp = 0.
                    # For zero and Q: wq = 0.
                    # For one and P: wp = n.
                    # For one and Q: wq = n.

                    # Build a graph of possible coincidences:
                    nodes = {('P', wp), ('Q', wq)}
                    for add in additions:
                        nodes.add(add)

                    # Two nodes can merge if they have the same weight AND
                    # the arrangement allows it.

                    # For arrangement: σ can match at most one of {P, Q, 0^n, 1^n}.
                    # comp(σ) can match at most one of {P, Q, 0^n, 1^n}.
                    # And σ ≠ comp(σ).

                    # For simplicity, I'll just count the number of distinct weights
                    # needed, considering the merge constraints.

                    # Merge rules:
                    # - Any two nodes with the same weight CAN be merged
                    #   UNLESS they are σ and comp(σ) (always different).
                    # - σ and comp(σ) cannot merge (they're always different vectors).

                    # Actually, two nodes with the same weight CAN be merged only if
                    # the arrangement allows them to be the same vector.
                    # This is generally possible (we have freedom in arrangement)
                    # UNLESS both are σ and comp(σ).

                    # Also: σ has weight s, and 0^n has weight 0.
                    # σ = 0^n iff s = 0. Then comp(σ) = 1^n (weight n).
                    # σ has weight s. P has weight wp.
                    # σ = P requires s = wp AND arrangement where σ has 1s at B,C positions.
                    # comp(σ) then has 1s at A,D positions, weight a+d.
                    # comp(σ) = Q requires Q has 1s at A,D positions... but Q has 1s at B,D.
                    # For comp(σ) = Q: need A positions = B positions. Since A,B disjoint
                    # (A has P[j]=Q[j]=0, B has P[j]=Q[j]=1), this requires a=0 and b=0.

                    # This is getting complex. Let me just enumerate the possible merging
                    # of σ with one target and comp(σ) with another target.

                    sigma_targets = ['new']
                    if s == wp: sigma_targets.append('P')
                    if s == wq: sigma_targets.append('Q')
                    if s == 0: sigma_targets.append('zero')
                    if s == n: sigma_targets.append('one')

                    comp_targets = ['new']
                    if n-s == wp: comp_targets.append('P')
                    if n-s == wq: comp_targets.append('Q')
                    if n-s == 0: comp_targets.append('zero')
                    if n-s == n: comp_targets.append('one')

                    # Check if both σ->P and comp->Q is compatible:
                    # σ = P and comp(σ) = Q requires P = comp(Q), which needs a=b=0.
                    # σ = P and comp(σ) = P: impossible (σ ≠ comp(σ)).
                    # σ = Q and comp(σ) = P: requires Q = comp(P), needs a=b=0.
                    # σ = Q and comp(σ) = Q: impossible.

                    for st in sigma_targets:
                        for ct in comp_targets:
                            # Check compatibility
                            if st == ct and st in ('P', 'Q', 'zero', 'one'):
                                continue  # σ = comp(σ) impossible

                            # For σ->P and comp->Q: need a=b=0
                            if st == 'P' and ct == 'Q':
                                if a != 0 or b != 0:
                                    continue
                            if st == 'Q' and ct == 'P':
                                if a != 0 or b != 0:
                                    continue

                            # Count distinct elements in the union
                            union = {'P', 'Q'}  # always present (since P≠Q and both used if s∈(0,n))
                            if s == 0:
                                union = {'Q'}  # only Q rows
                            elif s == n:
                                union = {'P'}  # only P rows
                            else:
                                union = {'P', 'Q'}

                            # Add needed elements
                            if a > 0:
                                union.add('zero')
                            if b > 0:
                                union.add('one')

                            if c > 0:
                                if st == 'P':
                                    pass  # σ = P, already in union
                                elif st == 'Q':
                                    pass  # σ = Q
                                elif st == 'zero':
                                    union.add('zero')  # might already be there
                                elif st == 'one':
                                    union.add('one')
                                else:
                                    union.add('sigma')

                            if d > 0:
                                if ct == 'P':
                                    pass
                                elif ct == 'Q':
                                    pass
                                elif ct == 'zero':
                                    union.add('zero')
                                elif ct == 'one':
                                    union.add('one')
                                else:
                                    union.add('comp')

                            # Also: P might equal zero or one
                            if wp == 0:
                                if 'P' in union and 'zero' in union:
                                    union.discard('zero')
                            if wp == n:
                                if 'P' in union and 'one' in union:
                                    union.discard('one')
                            if wq == 0:
                                if 'Q' in union and 'zero' in union:
                                    union.discard('zero')
                            if wq == n:
                                if 'Q' in union and 'one' in union:
                                    union.discard('one')

                            comp = len(union)
                            if comp <= 3:
                                achievable.add(k)
                                # Also add complement
                                k_comp = n*n - k
                                if 0 < k_comp < n*n:
                                    achievable.add(k_comp)
                                break
                        else:
                            continue
                        break

    return achievable


# Test
for nn in [3, 4, 5, 10, 20]:
    comp2_set = set()
    for cc in range(1, nn):
        comp2_set.add(cc*cc)
        comp2_set.add(nn*nn - cc*cc)
    for x in range(1, nn):
        y = nn - x
        comp2_set.add(x*x + y*y)
        comp2_set.add(2*x*y)
    comp2_set.discard(0)
    comp2_set.discard(nn*nn)

    comp3_set = compute_comp3_k_values(nn)
    N2 = len(comp2_set)
    N3_plus = len(comp3_set) - N2  # k values with comp exactly 3
    not_achieved = nn*nn - 1 - len(comp3_set)

    C_val = 2*nn*nn + N3_plus + 2*not_achieved
    print(f"C({nn}) = {C_val} (N2={N2}, N3={N3_plus}, N4={not_achieved}, total={N2+N3_plus+not_achieved})")
