#!/usr/bin/env python3
# Project Euler 840: compute S(5*10^4) mod 999676999

MOD = 999_676_999
N   = 50_000

# ---------- smallest prime factor sieve ----------
spf = list(range(N+1))
for i in range(2, int(N**0.5)+1):
    if spf[i] == i:
        step = i
        start = i*i
        for j in range(start, N+1, step):
            if spf[j] == j:
                spf[j] = i

# ---------- D(n): arithmetic derivative with D(1)=1 ----------
D = [0]*(N+1)
D[1] = 1
for n in range(2, N+1):
    # factor n via spf and use D(p^e)=e*p^(e-1), D(ab)=D(a)b+aD(b)
    m = n
    deriv = 0
    while m > 1:
        p = spf[m]
        e = 0
        while m % p == 0:
            m //= p
            e += 1
        # D(n) = n * sum_{p|n} e/p  (mod MOD)
        deriv = (deriv + e * (n // p)) % MOD
    D[n] = deriv % MOD

# ---------- Build B_k = sum_{d|k} d * (D(d))^(k/d) ----------
B = [0]*(N+1)
for d in range(1, N+1):
    y = D[d] % MOD
    pow_y = y  # y^(1)
    k = d
    while k <= N:
        # contribution for k = d * r with r growing
        B[k] = (B[k] + d * pow_y) % MOD
        k += d
        pow_y = (pow_y * y) % MOD

# ---------- Compute G(n) via g[0]=1; g[n] = (1/n) * sum_{k=1..n} B[k]*g[n-k] ----------
g = [0]*(N+1)
g[0] = 1
# Precompute inverses of 1..N (Python 3.8+: pow(a, -1, MOD) uses EGCD)
inv = [0]*(N+1)
for n in range(1, N+1):
    inv[n] = pow(n, -1, MOD)  # should exist for all n under this MOD

for n in range(1, N+1):
    s = 0
    # tight inner loop; Python handles 50k^2/2 ~ 1.25e9 ops badly,
    # but this is only ~1.25e9 *light* integer ops? We'll keep it lean.
    # (If needed, switch to PyPy; it flies.)
    for k in range(1, n+1):
        s += B[k] * g[n-k]
    g[n] = (s % MOD) * inv[n] % MOD

# ---------- S(N) = sum_{n=1..N} G(n) ----------
S = (sum(g) - 1) % MOD  # subtract g[0]
print(S)

# ---- Quick sanity checks given in the statement ----
# G(10) = 164 and S(10) = 396
def check_small():
    from functools import lru_cache
    # brute partitions for n<=10, using D from above
    @lru_cache(None)
    def G_brut(n, maxp):
        if n == 0:
            return 1
        tot = 0
        for p in range(1, min(n, maxp)+1):
            tot = (tot + D[p]*G_brut(n-p, p)) % MOD
        return tot
    G10 = G_brut(10, 10) % MOD
    S10 = sum(G_brut(i, i) for i in range(1, 11)) % MOD
    assert G10 == 164, G10
    assert S10 == 396, S10

# Uncomment to run the check (it’s tiny):
# check_small()

