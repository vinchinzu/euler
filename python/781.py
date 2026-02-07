#!/usr/bin/env python3
"""Project Euler 781 - Feynman Diagrams.

F(n) = number of non-isomorphic connected graphs with n degree-3 vertices
(each with incoming blue, outgoing blue, and one red edge) plus two degree-1
vertices (source and sink).

Key insight: F(n) = L(n)/n! where L(n) is the labeled connected count.
Using the exponential formula decomposition into main component + satellites:

T(n) = sum_{p} C(n,p) * L(p) * D(n-p) * (n-p-1)!!

where T(n) = B(n) * (n-1)!! is the total labeled count,
B(n) = (n+1)*D(n) + n*D(n-1) counts valid blue bijections,
D(n) = subfactorial/derangement count.

Setting m = n/2 and defining:
  t(m) = B(2m) / (2^m * m!)
  s(j) = D(2j) / (2^j * j!)
  f(m) = F(2m)

we get the recurrence: f(m) = t(m) - sum_{j=1}^{m} s(j) * f(m-j)
"""


def solve(n=50000):
    MOD = 10**9 + 7
    m = n // 2  # number of pairs

    # Precompute derangements D(k) for k=0..n, mod MOD
    D = [0] * (n + 1)
    D[0] = 1
    if n >= 1:
        D[1] = 0
    for k in range(2, n + 1):
        D[k] = (k * D[k - 1] + (1 if k % 2 == 0 else MOD - 1)) % MOD

    # Precompute factorials and inverse factorials mod MOD
    fact = [1] * (m + 1)
    for i in range(1, m + 1):
        fact[i] = fact[i - 1] * i % MOD
    inv_fact = [1] * (m + 1)
    inv_fact[m] = pow(fact[m], MOD - 2, MOD)
    for i in range(m - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD

    # Precompute powers of inv(2)
    inv2 = pow(2, MOD - 2, MOD)
    inv_pow2 = [1] * (m + 1)
    for i in range(1, m + 1):
        inv_pow2[i] = inv_pow2[i - 1] * inv2 % MOD

    # Compute B(2j) = (2j+1)*D(2j) + 2j*D(2j-1) for j=0..m
    # t(j) = B(2j) * inv(2^j) * inv(j!) mod MOD
    # s(j) = D(2j) * inv(2^j) * inv(j!) mod MOD
    t = [0] * (m + 1)
    s = [0] * (m + 1)
    for j in range(m + 1):
        idx = 2 * j
        if idx == 0:
            B_val = 1  # B(0) = 1*D(0) + 0 = 1
        else:
            B_val = ((idx + 1) * D[idx] + idx * D[idx - 1]) % MOD
        coeff = inv_pow2[j] * inv_fact[j] % MOD
        t[j] = B_val * coeff % MOD
        s[j] = D[idx] * coeff % MOD

    # Compute f(j) = t(j) - sum_{k=1}^{j} s(k) * f(j-k)
    f = [0] * (m + 1)
    for j in range(m + 1):
        val = t[j]
        for k in range(1, j + 1):
            val = (val - s[k] * f[j - k]) % MOD
        f[j] = val

    return f[m] % MOD


if __name__ == "__main__":
    print(solve())
