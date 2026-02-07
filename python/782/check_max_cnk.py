#!/usr/bin/env python3
"""
Check maximum c(n,k) for small n. Is it always ≤ 3?
"""

def complexity(matrix):
    n = len(matrix)
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    return len(patterns)

def brute_force_cnk(n):
    cnk = {}
    total = n * n
    for bits in range(2 ** total):
        k = bin(bits).count('1')
        matrix = []
        for i in range(n):
            row = []
            for j in range(n):
                row.append((bits >> (i * n + j)) & 1)
            matrix.append(row)
        c = complexity(matrix)
        if k not in cnk or c < cnk[k]:
            cnk[k] = c
    return cnk

# Check n=1,2,3,4
for n in range(1, 5):
    cnk = brute_force_cnk(n)
    max_c = max(cnk.values())
    print(f"n={n}: max c(n,k) = {max_c}")
    print(f"  c(n,k) = {[cnk[k] for k in range(n*n+1)]}")

# For n=5, we can't enumerate all 2^25 matrices easily.
# But let me check: is there any k for n=5 where c(5,k) > 3?
# I already know:
# C(5) = 64 (given)
# My formula gives C(5) = 64.
# 2*25 + N3 = 50 + 14 = 64. Checks out.
# But for n=20, 2*400 + N3 should be 1150.
# So N3 for n=20 should be 1150 - 800 = 350.
# My formula gives N3 = 399 - N2, and I'm getting N2 = 55 (or whatever makes C=1144).

# Let me compute N2 for n=20.
print("\n=== N2 computation for n=20 ===")

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

comp2_20 = get_comp2_k_values(20)
N2_20 = len(comp2_20)
N3_20 = 399 - N2_20
C20 = 800 + N3_20
print(f"N2 = {N2_20}")
print(f"N3 = {N3_20}")
print(f"C(20) = {C20}")
print(f"Expected C(20) = 1150")
print(f"Difference = {1150 - C20}")
print(f"So we need {1150 - C20} more k values with c=3 than we compute")
print(f"Equivalently, {1150 - C20} k values that we think are complexity 2 are actually complexity 3")
print(f"OR there are k values with c(n,k) > 3")

# Wait actually if c(n,k) can be > 3, the formula changes.
# C(n) = sum c(n,k) = #(c=1)*1 + #(c=2)*2 + #(c=3)*3 + #(c=4)*4 + ...
# = 2 + 2*N2 + 3*N3 + 4*N4 + ...
# = 2 + 2*(n^2-1-N3-N4-...) + 3*N3 + 4*N4 + ...
# = 2 + 2n^2 - 2 + N3 + 2*N4 + ...
# = 2n^2 + N3 + 2*N4 + ...

# So if some k values have c(n,k) = 4, C is larger.
# C(20) = 1150 > 1144 = my formula.
# So 1150 - 1144 = 6.
# If there are 3 k values with c=4 instead of c=3, that adds 3.
# If there are 6 k values with c=4 instead of c=3, that adds 6. Yes!
# Or 3 k values with c=4 instead of c=2, that adds 3*2=6.
# Or combinations.

# So SOME k values for n=20 have c(n,k) > 3!
# But for n ≤ 4 (brute forced), max c was 3.
# Let me check if c can be > 3 for any n by thinking about it.

# Actually wait - let me reconsider. My formula assumed c(n,k) ∈ {1,2,3}.
# But maybe for larger n, some k values require c(n,k) = 4 or more?
# That would explain C(20) being higher.

# Let me think: for which k values is even c(n,k) = 3 not achievable?
# A matrix with complexity 3 has 3 distinct patterns in rows∪cols.
# That's a much wider class. The question is whether ALL k from 1 to n^2-1
# can be achieved with complexity ≤ 3.

# Actually, I bet ALL k can be achieved with complexity ≤ 3 for sufficiently
# large n, but there might be some k values for small n that need complexity > 3.

# Hmm wait, for n=4 max was 3. For n=3 max was 3. For n=2 max was 2.
# So maybe max c(n,k) = 3 for all n ≥ 3 and all k.
# Then my formula C(n) = 2n^2 + N3 should be exact.
# But it gives C(20) = 1144, not 1150.
# So either my N2 computation is wrong (too large), or c(n,k) can exceed 3.

# Let me verify: compute C(n) for small n and compare with known.
for n in range(1, 21):
    comp2 = get_comp2_k_values(n)
    N2 = len(comp2)
    N3 = (n * n - 1) - N2
    Cn = 2 * n * n + N3
    print(f"C({n}) = {Cn}")
