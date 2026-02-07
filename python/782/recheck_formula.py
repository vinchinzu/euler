#!/usr/bin/env python3
"""
Let me reconsider the problem. I need to check: can c(n,k) ever be > 3?

For n ≤ 10, my formula (assuming c ≤ 3) gives the correct C(n).
But for n=20 it fails. Let me see if the issue is in the number of comp-2 k values.

Let me actually enumerate the comp-2 k values more carefully.

Actually, wait. Let me re-examine my "direct" method for n=8.
In my optimized version, I compute k = sum_j [s*P[j] + (n-s)*Q[j]],
but this assumes σ is a vector with sum s, and I'm computing k based on weight only.

But the actual k depends on the specific σ, not just its weight!

Wait, no. k = sum_{i,j} M[i][j] = sum_j sum_i M[i][j].
M[i][j] = P[j] if σ[i]=1 else Q[j].
sum_i M[i][j] = (sum of σ[i]) * P[j] + (n - sum σ[i]) * Q[j] = s*P[j] + (n-s)*Q[j].
So k = sum_j [s*P[j] + (n-s)*Q[j]] = s*|P| + (n-s)*|Q| where |X| = weight of X.

So k depends only on s, |P|, |Q|, and n. Not on the specific arrangement.
This means my formula should be right.

But wait, the constraint is σ ∈ {P, Q} (or complement). And for the arrangement
to work, we need specific structure on P, Q.

Hmm but I verify against direct enumeration for n ≤ 10 and get the right answer.
Let me just check: is C(20) really 1150?
"""

# Let me check the problem statement values more carefully.
# The problem says C(2)=8, C(5)=64, C(10)=274, C(20)=1150.

# My computation:
# C(2)=8 ✓
# C(5)=64 ✓
# C(10)=274 ✓
# C(20)=1144 ✗ (expected 1150)

# So somewhere between n=10 and n=20, my formula breaks.
# Let me compute the difference C(n) - 2n^2 for each n and look for patterns.

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

print("n | N2 | N3=n^2-1-N2 | C=2n^2+N3 | comp2 k-values")
for n in range(1, 25):
    comp2 = get_comp2_k_values(n)
    N2 = len(comp2)
    N3 = (n * n - 1) - N2
    Cn = 2 * n * n + N3
    print(f"{n:2d} | {N2:3d} | {N3:4d} | {Cn:5d} | {sorted(comp2)[:20]}{'...' if len(comp2) > 20 else ''}")

# Let me also look at the comp2 k values more carefully.
# For the block-matrix case: k = a*b with 0 ≤ a,b ≤ n gives complexity ≤ 3.
# For complexity 2 specifically with the constraints, the achievable k values
# seem to follow a specific pattern.

print("\n\n=== Comp2 k-values analysis ===")
for n in range(2, 15):
    comp2 = sorted(get_comp2_k_values(n))
    print(f"n={n}: {comp2}")
    # Note symmetry: k and n^2-k should both appear or both not
    sym = all((n*n - k) in comp2 for k in comp2)
    print(f"  Symmetric: {sym}")
