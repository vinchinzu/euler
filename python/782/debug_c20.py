#!/usr/bin/env python3
"""
Debug C(20): Expected 1150 but got 1144.
Difference = 6, meaning 6 k values are incorrectly classified.

Let me think about what my analysis might be missing.

My analysis assumed a very specific structure:
- The matrix has rows that are either P or Q.
- The column types are determined by P, Q, and the row arrangement σ.

But I might be missing that P and Q can overlap with columns in more complex ways.
Let me reconsider.

Actually, let me reconsider the complexity-2 analysis from scratch.

A matrix has complexity 2 means |rows ∪ cols| = 2.
So there are exactly 2 distinct binary strings of length n, say P and Q,
such that every row is P or Q and every column is P or Q.

Let me denote:
- σ[i] = 1 if row i is P, 0 if row i is Q. So σ ∈ {0,1}^n.
- τ[j] = 1 if column j is P, 0 if column j is Q. So τ ∈ {0,1}^n.

Now, M[i][j] is the j-th entry of row i's pattern.
If row i is P (σ[i]=1): M[i][j] = P[j]
If row i is Q (σ[i]=0): M[i][j] = Q[j]

Also, M[i][j] is the i-th entry of column j's pattern.
If column j is P (τ[j]=1): M[i][j] = P[i]
If column j is Q (τ[j]=0): M[i][j] = Q[i]

Combining both:
If σ[i]=1 (row i = P): M[i][j] = P[j]
  If τ[j]=1: P[j] = P[i] (since column j is P, its i-th entry is P[i])
  If τ[j]=0: P[j] = Q[i]

If σ[i]=0 (row i = Q): M[i][j] = Q[j]
  If τ[j]=1: Q[j] = P[i]
  If τ[j]=0: Q[j] = Q[i]

So:
For σ[i]=1, τ[j]=1: P[j] = P[i]           ... (A)
For σ[i]=1, τ[j]=0: P[j] = Q[i]           ... (B)
For σ[i]=0, τ[j]=1: Q[j] = P[i]           ... (C)
For σ[i]=0, τ[j]=0: Q[j] = Q[i]           ... (D)

From (A): For any i with σ[i]=1 and any j with τ[j]=1: P[i] = P[j].
This means P is constant on the set {i: σ[i]=1} ∪ {j: τ[j]=1}.
Let S = {i: σ[i]=1} and T = {j: τ[j]=1}.
P is constant on S ∪ T. Let p = P[i] for i ∈ S ∪ T.

From (B): For i ∈ S and j ∉ T: P[j] = Q[i].
P[j] for j ∉ T, and Q[i] for i ∈ S.
Let me denote:
- P restricted to T^c (complement of T): P[j] for j ∉ T.
- Q restricted to S: Q[i] for i ∈ S.
(B) says: for all i ∈ S and j ∉ T, P[j] = Q[i].
This means both P on T^c and Q on S are constant. Let q' = P[j] for j ∉ T = Q[i] for i ∈ S.

From (C): For i ∉ S and j ∈ T: Q[j] = P[i].
Q on T and P on S^c. Both constant. Let p' = Q[j] for j ∈ T = P[i] for i ∉ S.

From (D): For i ∉ S and j ∉ T: Q[j] = Q[i].
Q on T^c and Q on S^c. Both constant and equal. Let q = Q[j] for j ∉ T = Q[i] for i ∉ S.

Summary:
P[i] = p for i ∈ S ∪ T
P[i] = p' for i ∉ S (so P[i] for i ∈ S: p, for i ∉ S: p')
Wait, let me reconcile:
- P[i] = p for i ∈ S ∪ T (from (A))
- P[i] = p' for i ∉ S (from (C))
These must be consistent. For i ∈ T \ S: i ∈ S ∪ T so P[i] = p; also i ∉ S so P[i] = p'.
So p = p' if T \ S ≠ ∅.

Similarly:
- Q[i] = q' for i ∈ S (from (B))
- Q[i] = q for i ∉ S (from (D))
- Q[j] = p' for j ∈ T (from (C))
- Q[j] = q for j ∉ T (from (D))
For j ∈ T ∩ S: Q[j] = q' (from i ∈ S) and Q[j] = p' (from j ∈ T). So q' = p' if S ∩ T ≠ ∅.
For j ∈ T \ S: Q[j] = q (from j ∉ S) and Q[j] = p' (from j ∈ T). So q = p' if T \ S ≠ ∅.

And P on T^c: P[j] = q' for j ∉ T (from (B)).
P[j] for j ∉ T and j ∈ S: P[j] = p (from S ∪ T, j ∈ S). And P[j] = q' (from (B), j ∉ T).
So p = q' if S \ T ≠ ∅.

Let me handle this case by case based on S ∩ T, S \ T, T \ S, (S ∪ T)^c.
Let α = |S ∩ T|, β = |S \ T|, γ = |T \ S|, δ = |(S ∪ T)^c| = n - α - β - γ.
|S| = α + β = s, |T| = α + γ = t.

P:
- On S ∩ T: P = p (from A, i ∈ S, j ∈ T)
- On S \ T: P = p (from A, i ∈ S) AND P = q' (from B, j ∉ T). So p = q' if β > 0.
- On T \ S: P = p (from A, j ∈ T) AND P = p' (from C, i ∉ S). So p = p' if γ > 0.
- On complement: P = p' (from C) AND P = q' (from B).

Q:
- On S ∩ T: Q = q' (from B, i ∈ S) AND Q = p' (from C, j ∈ T). So q' = p' if α > 0.
- On S \ T: Q = q' (from B, i ∈ S) AND Q = q (from D, j ∉ T).
- On T \ S: Q = p' (from C, j ∈ T) AND Q = q (from D, i ∉ S). So p' = q if γ > 0.
- On complement: Q = q (from D).

This is quite involved. Let me enumerate the 16 cases based on which of α, β, γ, δ are > 0.

Actually, let me simplify by considering the 4 values p, q, p', q' and the constraints.
"""

import itertools

def get_all_comp2_k(n):
    """
    Exhaustive computation of complexity-2 achievable k values.

    The matrix M has complexity 2 with patterns {P, Q}.
    σ[i] = 1 iff row i = P, τ[j] = 1 iff col j = P.
    Let S = support(σ), T = support(τ).
    s = |S|, t = |T|.
    α = |S ∩ T|, β = |S \ T|, γ = |T \ S|, δ = n - α - β - γ.

    P values: p on S∪T, p' on (S∪T)^c and also on T\S, q' on T^c (and S)
    Q values: q' on S, p' on T, q on complement

    With constraints depending on which of α,β,γ,δ > 0.

    Let me just enumerate (s, t, p, q, p', q') and check all constraints.
    """
    achievable = set()

    for s in range(n+1):
        for t in range(n+1):
            # α = |S ∩ T| ranges from max(0, s+t-n) to min(s, t)
            for alpha in range(max(0, s+t-n), min(s, t)+1):
                beta = s - alpha
                gamma = t - alpha
                delta = n - alpha - beta - gamma

                if delta < 0:
                    continue

                for p in range(2):
                    for q in range(2):
                        for pp in range(2):  # p'
                            for qp in range(2):  # q'
                                # Check constraints
                                # P on S ∩ T: p
                                # P on S \ T: p and q' => if β > 0: p = q'
                                if beta > 0 and p != qp:
                                    continue
                                # P on T \ S: p and p' => if γ > 0: p = p'
                                if gamma > 0 and p != pp:
                                    continue
                                # P on complement: p' and q'
                                if delta > 0 and pp != qp:
                                    continue

                                # Q on S ∩ T: q' and p' => if α > 0: q' = p'
                                if alpha > 0 and qp != pp:
                                    continue
                                # Q on S \ T: q' and q
                                if beta > 0 and qp != q:
                                    continue
                                # Q on T \ S: p' and q => if γ > 0: p' = q
                                if gamma > 0 and pp != q:
                                    continue
                                # Q on complement: q
                                # (no extra constraint)

                                # Now construct P and Q and check they're different
                                # P = (p on S∪T, ? on complement)
                                # But we need to be precise:
                                # P[i] for i in S∪T = p
                                # P[i] for i in complement = p' (from C constraint) = q' (from B)
                                # Q[i] for i in S = q'
                                # Q[i] for i not in S = q
                                # Q[j] for j in T = p'
                                # Q[j] for j not in T = q

                                # P is a vector: p repeated (α+β+γ) times, then p'/q' repeated δ times
                                # (where p'=q' by the delta>0 constraint)
                                # Actually P[i] depends on whether i ∈ S ∪ T:
                                # P[i] = p if i ∈ S ∪ T (size α+β+γ)
                                # P[i] = p' (or q', they're equal if δ>0) if i ∉ S ∪ T (size δ)
                                # But if δ = 0, what is P on complement? Not constrained. Let's set it to p by default.

                                # For P and Q to be different patterns:
                                # We need P ≠ Q as length-n binary vectors.

                                # Build P and Q explicitly (as counts of 0s and 1s is enough):
                                # P: p for (α+β+γ) positions, and for δ positions:
                                #   if δ > 0: pp (=qp by constraint)
                                #   if δ = 0: doesn't matter
                                p_complement_val = pp if delta > 0 else p  # default to p

                                # Q is trickier because S and T partition differently for Q:
                                # Q[i] for i ∈ S: qp
                                # Q[i] for i ∉ S: q
                                # This splits positions differently from how P is defined.

                                # Let me compute P and Q on the 4 regions:
                                # Region S∩T (size α): P=p, Q=qp (and also p')
                                # Region S\T (size β): P=p, Q=qp (and also q)
                                # Region T\S (size γ): P=p, Q=q (and also p')
                                # Region compl (size δ): P=pp (=qp), Q=q

                                P_vals = [p]*alpha + [p]*beta + [p]*gamma + [p_complement_val]*delta
                                Q_vals = [qp]*alpha + [qp]*beta + [q]*gamma + [q]*delta

                                if P_vals == Q_vals:
                                    continue  # P = Q, complexity 1

                                # Compute k:
                                # k = sum over all positions of M[i][j]
                                # M[i][j] = P[j] if σ[i]=1 else Q[j]
                                # Number of 1s:
                                # For positions j in S∩T: P[j]=p, Q[j]=qp. Contribution: s*p + (n-s)*qp from this set of α columns
                                # For positions j in S\T: P[j]=p, Q[j]=qp. Same: s*p + (n-s)*qp from β columns
                                # For positions j in T\S: P[j]=p, Q[j]=q. Contribution: s*p + (n-s)*q from γ columns
                                # For positions j in complement: P[j]=pp, Q[j]=q. Contribution: s*pp + (n-s)*q from δ columns

                                # Wait: M[i][j] = P[j] if σ[i]=1 else Q[j].
                                # For each column j, the contribution to k is: s*P[j] + (n-s)*Q[j]
                                # (since s rows have σ[i]=1 and n-s have σ[i]=0)

                                k = 0
                                # S∩T columns (α of them)
                                k += alpha * (s * p + (n - s) * qp)
                                # S\T columns (β of them)
                                k += beta * (s * p + (n - s) * qp)
                                # T\S columns (γ of them)
                                k += gamma * (s * p + (n - s) * q)
                                # Complement columns (δ of them)
                                k += delta * (s * p_complement_val + (n - s) * q)

                                # Hmm wait, I'm conflating: Q values depend on column position j,
                                # not on the region of j.
                                # Q[j] for j ∈ S∩T: Q[j] = ? We said Q[j] for j ∈ T = p', and for j ∈ S = q'.
                                # For j ∈ S∩T: Q[j] must be both p' and q'. So p' = q' needed.
                                # Already constrained by α > 0 => q' = p'.

                                # Actually let me recompute Q[j] per region:
                                # Q[j]: j ∈ T => Q[j] = p'; j ∉ T => Q[j] = q (from D)
                                # Also Q[i]: i ∈ S => Q[i] = q' (from B); i ∉ S => Q[i] = q (from D)
                                # These are about Q as a function of position.
                                # Since j is a column index but Q is a pattern (vector indexed by row positions):

                                # WAIT. I think I'm confusing something fundamental.
                                # P and Q are binary vectors of length n. P[j] is the j-th entry.
                                # Row i being P means the entire row vector is P, so M[i][j] = P[j].
                                # Column j being P means the entire column vector is P, so M[i][j] = P[i].

                                # So P[j] is used when accessing column index j of a P-row.
                                # P[i] is used when accessing row index i of a P-column.
                                # The same vector P is used for both!

                                # Now Q[j] for column j's value in a Q-row: M[i][j] = Q[j] if row i = Q.
                                # And for row i's value in a P-column: M[i][j] = P[i] if col j = P.

                                # So the actual Q values by region:
                                # For each column position j:
                                #   P[j] is determined by which region j is in.
                                #   Q[j] is also determined by which region j is in.

                                # From the analysis:
                                # P[j]:
                                #   j ∈ S ∪ T => P[j] = p
                                #   j ∉ S ∪ T => P[j] = pp (if δ > 0)

                                # Q[j]: We derived Q on different regions.
                                # Q on S ∩ T = qp (and pp, which equals qp if α > 0)
                                # Q on S \ T = qp (from i ∈ S) and q (from j ∉ T) => qp = q if β > 0
                                # Q on T \ S = pp (from j ∈ T) and q (from i ∉ S) => pp = q if γ > 0
                                # Q on complement = q

                                # So recompute Q[j] for each region:
                                Q_region = {}
                                Q_region['ST'] = qp  # S ∩ T
                                Q_region['S_only'] = qp  # S \ T (qp = q if β > 0)
                                Q_region['T_only'] = pp  # T \ S (pp = q if γ > 0)
                                Q_region['comp'] = q   # complement

                                # And P[j]:
                                P_region = {}
                                P_region['ST'] = p
                                P_region['S_only'] = p
                                P_region['T_only'] = p
                                P_region['comp'] = pp if delta > 0 else 0  # arbitrary if δ = 0

                                # k = sum over j of [s * P[j] + (n-s) * Q[j]]
                                k = 0
                                k += alpha * (s * P_region['ST'] + (n-s) * Q_region['ST'])
                                k += beta * (s * P_region['S_only'] + (n-s) * Q_region['S_only'])
                                k += gamma * (s * P_region['T_only'] + (n-s) * Q_region['T_only'])
                                if delta > 0:
                                    k += delta * (s * P_region['comp'] + (n-s) * Q_region['comp'])

                                if 0 < k < n*n:
                                    achievable.add(k)

    return achievable


def get_comp2_k_values_v1(n):
    """Original version for comparison."""
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


# Compare for small n
for n in range(2, 8):
    v1 = get_comp2_k_values_v1(n)
    v2 = get_all_comp2_k(n)
    print(f"n={n}: v1={sorted(v1)}")
    print(f"      v2={sorted(v2)}")
    if v1 != v2:
        print(f"  DIFF: in v2 not v1: {sorted(v2 - v1)}, in v1 not v2: {sorted(v1 - v2)}")
    print()
