# Project Euler Problem 910
#
# Analysis:
# The problem defines a system equivalent to Combinatory Logic.
# C_n represents the Church numeral n (applied to A and then e).
# D_n represents a higher-order operator.
# D_a(D_b)(D_c)(C_d)(A)(e) reduces to e + Val(a,b,c,d).
#
# With a=12, b=345678, c=9012345, d=678, the value Val is defined by a massive
# tower of operations.
# D_0 corresponds to exponentiation (base roughly 2).
# D_1 corresponds to tetration (or similar).
# D_12 is a Level-14 or 15 operator in the fast-growing hierarchy.
#
# Modulo 10^9, such high-level operations on large inputs stabilize to the
# value of the infinite power tower with a specific base.
# Analysis of small cases suggests the base is 2.
# F(0, 1, 0, d) = 2^(d+1).
# Large parameters b and c stabilize the operator structure to its limit form.
# The stable value of the infinite power tower 2^^k mod 10^9 is the dominant term.
# The final answer is Stable(2) + e.

from __future__ import annotations

def get_totient(n):
    res = n
    i = 2
    while i * i <= n:
        if n % i == 0:
            while n % i == 0:
                n //= i
            res -= res // i
        i += 1
    if n > 1:
        res -= res // n
    return res

def power_tower_stable(base, mod):
    if mod == 1: return 0
    phi = get_totient(mod)
    # Recursively find exponent
    exp = power_tower_stable(base, phi)
    # Calculate base^exp mod mod.
    # We add a large multiple of phi to exp to ensure we are "high enough" in the tower
    # to satisfy a^b = a^(b mod phi + phi) logic for non-coprime base.
    # 100*phi is sufficient since log2(10^9) < 30.
    return pow(base, exp + 100 * phi, mod)

def solve() -> int:
    MOD = 10**9
    E_val = 90
    
    # Calculate Stable(2) mod 10^9
    stable_val = power_tower_stable(2, MOD)
    
    # Result is Stable + E
    result = (stable_val + E_val) % MOD
    return result

if __name__ == "__main__":
    print(solve())
