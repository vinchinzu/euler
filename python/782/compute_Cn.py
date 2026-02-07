#!/usr/bin/env python3
"""
Compute C(n) = 2n^2 + N3, where N3 = (n^2-1) - N2,
and N2 = number of k in {1,...,n^2-1} achievable with complexity 2.

From the analysis, k is achievable with complexity 2 iff we can find
(a, b, c, d, s) with a+b+c+d = n, c+d > 0, satisfying the constraints,
and k = s*(b+c) + (n-s)*(b+d).

Let me compute this efficiently.
"""

def get_comp2_k_values(n):
    """Find all k values achievable with complexity exactly 2 for an n×n matrix."""
    achievable = set()

    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue

                # Check zero/ones constraints
                zero_is_P = (b + c == 0)
                zero_is_Q = (b + d == 0)
                ones_is_P = (a + d == 0)
                ones_is_Q = (a + c == 0)

                if a > 0 and not (zero_is_P or zero_is_Q):
                    continue
                if b > 0 and not (ones_is_P or ones_is_Q):
                    continue

                # Find valid s values
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

def C(n):
    comp2 = get_comp2_k_values(n)
    N2 = len(comp2)
    N3 = (n * n - 1) - N2
    return 2 * n * n + N3

# Verify known values
print("Verification:")
for n in [1, 2, 3, 4, 5, 10, 20]:
    cn = C(n)
    print(f"C({n}) = {cn}")

# Expected: C(2)=8, C(5)=64, C(10)=274, C(20)=1150
