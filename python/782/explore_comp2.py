#!/usr/bin/env python3
"""
Characterize which k values have c(n,k) = 2.

From the analysis: a complexity-2 matrix has 2 patterns {P, Q} in union(rows, cols).
Every row and every column is P or Q.

We showed that with σ being the row indicator:
- Positions where P[j]=Q[j]=0: column j = all-zeros vector
- Positions where P[j]=Q[j]=1: column j = all-ones vector
- Positions where P[j]=1,Q[j]=0: column j = σ
- Positions where P[j]=0,Q[j]=1: column j = complement(σ)

All columns must ∈ {P, Q}, so:
(i)   If ∃j with P[j]=Q[j]=0: (0,...,0) ∈ {P,Q}
(ii)  If ∃j with P[j]=Q[j]=1: (1,...,1) ∈ {P,Q}
(iii) If ∃j with P[j]=1,Q[j]=0: σ ∈ {P,Q}
(iv)  If ∃j with P[j]=0,Q[j]=1: complement(σ) ∈ {P,Q}

P and Q are binary vectors of length n.
σ is binary vector of length n with some weight s (number of rows that are type P).

Let me enumerate cases based on which of types A,B,C,D are present.
a = #positions where P[j]=Q[j]=0 (≥0)
b = #positions where P[j]=Q[j]=1 (≥0)
c = #positions where P[j]=1,Q[j]=0 (≥0)
d = #positions where P[j]=0,Q[j]=1 (≥0)
a+b+c+d = n

Number of 1s:
Each row of type P contributes b+c ones. Each row of type Q contributes b+d ones.
k = s*(b+c) + (n-s)*(b+d) = s*b + s*c + n*b + n*d - s*b - s*d = n*b + s*c + (n-s)*d
Actually: k = s*(b+c) + (n-s)*(b+d) = n*b + s*c + (n-s)*d

Now constraints:
(i) a > 0 ⟹ (0,...,0) ∈ {P,Q}
(ii) b > 0 ⟹ (1,...,1) ∈ {P,Q}
(iii) c > 0 ⟹ σ ∈ {P,Q}
(iv) d > 0 ⟹ complement(σ) ∈ {P,Q}

Also P ≠ Q (else complexity 1), meaning (c,d) ≠ (0,0) (if c=d=0 then P=Q).

CASE 1: c > 0, d = 0 (P[j] ≥ Q[j] for all j)
Then from (iii): σ ∈ {P, Q}.
P has 1s at positions of type B and C, 0s at type A.
Q has 1s at positions of type B, 0s at type A and C.
σ has 1s where rows are type P.

σ = P means σ has 1s at type B and type C positions.
But σ is about rows, while types are about columns. Wait... σ is a length-n vector
where σ[i] = 1 if row i is P. P is also a length-n vector where P[j] depends on column j.

Row indices and column indices both run from 0 to n-1, but they're different!
So σ is about row indices, P is about column indices. They can still be equal
as abstract binary vectors of length n.

If σ = P: σ[i] = P[i] for all i. P[i] = 1 iff position i is type B or C.
So row i is type P iff column i is type B or C.
Weight of σ = b + c = s.

(i) a > 0 ⟹ (0,...,0) ∈ {P,Q}. We need all-zeros = P or Q.
    P = 0 at type A positions, 1 at B and C. P = all-zeros iff b=c=0, but c>0. So P ≠ all-zeros.
    Q = 0 at type A and C, 1 at B. Q = all-zeros iff b=0.
    So if a > 0 and b > 0: neither is all-zeros. Constraint violated.
    If a > 0 and b = 0: Q is all-zeros. OK, Q = (0,...,0).

(ii) b > 0 ⟹ (1,...,1) ∈ {P,Q}.
    P = all-ones iff a = 0 and all positions are B or C. i.e., a=d=0 and n=b+c.
    Q = all-ones iff a=c=0 and n=b (and d=0 since we're in case d=0).
    So Q = all-ones iff a=c=0, but c>0. So Q ≠ all-ones.
    P = all-ones iff a = 0 (since d=0 already).
    If b > 0 and a > 0: need P or Q = all-ones. But P is not all-ones (a>0) and Q is not all-ones (c>0). Violated.
    If b > 0 and a = 0: P = all-ones (positions are B and C only, P has 1s everywhere). OK.

Summary for Case 1 (c > 0, d = 0):
If a > 0 and b > 0: impossible (constraints (i) and (ii) both need satisfaction)
Actually, constraint (i) needs (0,...,0) ∈ {P,Q} and (ii) needs (1,...,1) ∈ {P,Q}.
Since {P,Q} has exactly 2 elements, we'd need {P,Q} = {(0,...,0), (1,...,1)}.
But P has 1s at B and C positions and 0s elsewhere, and we need P = all-zeros or all-ones.
If P = all-zeros: b=c=0, contradicts c>0.
If P = all-ones: a=0, contradicts a>0.
So a > 0 AND b > 0 is indeed impossible in Case 1.

Sub-case 1a: a > 0, b = 0, c > 0, d = 0. So n = a + c.
Q = all-zeros. P has c ones (at type C positions).
σ = P (from (iii)).
s = weight of σ = b + c = c.
k = n*b + s*c + (n-s)*d = 0 + c*c + 0 = c^2.
Also k = s*(b+c) + (n-s)*(b+d) = c*(0+c) + a*(0+0) = c^2.

Sub-case 1b: a = 0, b > 0, c > 0, d = 0. So n = b + c.
P = all-ones. P has n ones.
σ = P = all-ones means s = n (all rows are type P).
Then all rows are P = all-ones, so k = n*n = n^2.
But then the matrix is all-ones, complexity 1. P = Q is required which contradicts P ≠ Q.
Wait, if s = n then all rows are P, so we only see row pattern P.
Columns: each column j = (P[j])^n = all-P[j] = constant.
If P = all-ones, columns are all all-ones. So only pattern is all-ones. Complexity 1.
So this sub-case gives complexity 1, not 2.

Actually wait, can σ = Q instead of P? From (iii): σ ∈ {P, Q}.
If σ = Q: σ = Q. Q has 1s at type B positions, 0s at type A=0 and C.
σ[i] = Q[i] for all i. Weight of σ = b = s.
k = s*(b+c) + (n-s)*(b+d) = b*(b+c) + c*(b+0) = b^2 + 2bc = b(b+2c).
Hmm, let me just be more systematic and enumerate all valid configurations.

Let me write code to enumerate valid (a,b,c,d,σ_choice) configurations.
"""

def get_achievable_k_comp2(n):
    """Find all k values achievable with complexity exactly 2 for an n×n matrix."""
    achievable = set()

    # Enumerate all possible (a, b, c, d) with a+b+c+d = n, at least c+d > 0 (P ≠ Q)
    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue  # P = Q

                # Constraints:
                # (i) a > 0 => all-zeros ∈ {P, Q}
                # (ii) b > 0 => all-ones ∈ {P, Q}
                # (iii) c > 0 => σ ∈ {P, Q}
                # (iv) d > 0 => complement(σ) ∈ {P, Q}

                # P has 1s at type B and C positions (weight b+c)
                # Q has 1s at type B and D positions (weight b+d)
                # all-zeros has weight 0
                # all-ones has weight n
                # σ has weight s (unknown, between 0 and n)
                # complement(σ) has weight n-s

                # We need to find possible σ configurations.
                # σ is a binary vector of length n where σ[i] tells us if row i uses P.
                # But the constraint is on σ as an abstract vector matching P or Q.

                # Since σ is compared to P and Q position-wise, and P,Q depend on
                # the column type assignments, σ must match P or Q as binary vectors.

                # P and Q are specific vectors determined by (a,b,c,d) arrangement.
                # But σ is about ROW assignments. The constraint σ ∈ {P,Q} etc. treats
                # them as abstract n-length binary vectors.

                # Actually wait - the analysis above showed that column j depends on σ.
                # Let me re-derive:
                # P is a specific vector (the pattern of one row type)
                # σ is a specific vector (which rows use pattern P)
                # They live in the same vector space {0,1}^n.
                # Column j depends on σ and P[j], Q[j].

                # But the arrangement of types across columns is a choice.
                # We have a columns of type A, b of type B, etc.
                # σ has s ones in positions 0..n-1 (s = number of P-rows).

                # The key: σ is treated as a binary vector of length n, and it must
                # be in {P, Q} (if c > 0) and complement(σ) must be in {P, Q} (if d > 0).

                # But P and Q are specific vectors depending on which columns are
                # type A, B, C, D. By choosing the column arrangement, we can choose
                # which positions of P are 0 or 1. Essentially, we're choosing which
                # positions are in each type.

                # So the question becomes: can we arrange a positions of type A,
                # b of type B, c of type C, d of type D among the n column positions,
                # AND choose which s of the n row positions are type P (giving σ),
                # such that:
                # - If c > 0: σ = P or σ = Q (as binary vectors)
                # - If d > 0: complement(σ) = P or complement(σ) = Q

                # P[j] = 1 iff j is in B or C positions
                # Q[j] = 1 iff j is in B or D positions

                # σ = P means: σ[i] = 1 iff column i is type B or C.
                #   weight(σ) = b + c = s.
                # σ = Q means: σ[i] = 1 iff column i is type B or D.
                #   weight(σ) = b + d = s.
                # complement(σ) = P: σ[i] = 0 iff column i is type B or C.
                #   σ[i] = 1 iff column i is type A or D. weight(σ) = a + d = s.
                # complement(σ) = Q: σ[i] = 0 iff column i is type B or D.
                #   σ[i] = 1 iff column i is type A or C. weight(σ) = a + c = s.

                # So the possible σ-pattern requirements:
                constraints_sigma = []
                if c > 0:
                    # σ ∈ {P, Q}
                    constraints_sigma.append('P_or_Q')
                if d > 0:
                    # complement(σ) ∈ {P, Q}
                    constraints_sigma.append('comp_P_or_Q')

                # For the zero/ones constraints:
                zero_required = a > 0
                ones_required = b > 0

                # all-zeros vector: weight 0. This is P iff b+c=0, Q iff b+d=0.
                zero_is_P = (b + c == 0)
                zero_is_Q = (b + d == 0)
                # all-ones vector: weight n. This is P iff a+d=0, Q iff a+c=0.
                ones_is_P = (a + d == 0)
                ones_is_Q = (a + c == 0)

                if zero_required and not (zero_is_P or zero_is_Q):
                    continue
                if ones_required and not (ones_is_P or ones_is_Q):
                    continue

                # Now handle σ constraints.
                # σ can be P (weight b+c) or Q (weight b+d)
                # complement(σ) can be P (weight b+c, so σ weight = n-b-c=a+d) or Q (weight b+d, so σ weight = a+c)

                possible_s_sigma = []  # possible (s, σ_is) tuples

                if c > 0 and d > 0:
                    # σ ∈ {P,Q} AND complement(σ) ∈ {P,Q}
                    # Options:
                    for sigma_choice in ['P', 'Q']:
                        for comp_choice in ['P', 'Q']:
                            if sigma_choice == 'P':
                                s_from_sigma = b + c
                            else:
                                s_from_sigma = b + d
                            if comp_choice == 'P':
                                s_from_comp = a + d  # weight of σ when comp(σ)=P
                            else:
                                s_from_comp = a + c
                            if s_from_sigma == s_from_comp:
                                possible_s_sigma.append(s_from_sigma)
                elif c > 0 and d == 0:
                    # σ ∈ {P, Q}, no constraint from comp
                    possible_s_sigma.append(b + c)  # σ = P
                    possible_s_sigma.append(b + d)  # σ = Q
                elif c == 0 and d > 0:
                    # comp(σ) ∈ {P, Q}, no constraint from σ
                    possible_s_sigma.append(a + d)  # comp(σ) = P, so σ weight = a+d
                    possible_s_sigma.append(a + c)  # comp(σ) = Q, so σ weight = a+c
                else:
                    # c = 0, d = 0: already excluded above
                    pass

                # For each valid s, compute k
                for s in set(possible_s_sigma):
                    if s < 0 or s > n:
                        continue
                    # k = s*(b+c) + (n-s)*(b+d)
                    k = s * (b + c) + (n - s) * (b + d)
                    if 0 < k < n * n:  # c(n,k)=1 for k=0 or n^2
                        achievable.add(k)

                    # Also consider by symmetry: complement the entire matrix.
                    # Complementing swaps 0↔1, so k -> n^2 - k,
                    # and complexity is preserved.
                    k_comp = n * n - k
                    if 0 < k_comp < n * n:
                        achievable.add(k_comp)

    return achievable

# Verify against brute force
def complexity(matrix):
    n = len(matrix)
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    return len(patterns)

def brute_force_comp2_k(n):
    achievable = set()
    total = n * n
    for bits in range(2 ** total):
        k = bin(bits).count('1')
        if k == 0 or k == total:
            continue
        matrix = []
        for i in range(n):
            row = []
            for j in range(n):
                row.append((bits >> (i * n + j)) & 1)
            matrix.append(row)
        c = complexity(matrix)
        if c <= 2:
            achievable.add(k)
    return achievable

for n in range(2, 6):
    analytic = get_achievable_k_comp2(n)
    if n <= 4:
        brute = brute_force_comp2_k(n)
        match = analytic == brute
        print(f"n={n}: analytic={sorted(analytic)}, brute={sorted(brute)}, match={match}")
        if not match:
            print(f"  Missing from analytic: {sorted(brute - analytic)}")
            print(f"  Extra in analytic: {sorted(analytic - brute)}")
    else:
        print(f"n={n}: analytic={sorted(analytic)}")

    N2 = len(analytic)
    N3 = n*n - 1 - N2
    Cn = 2*n*n + N3
    print(f"  N2={N2}, N3={N3}, C({n}) = {Cn}")
