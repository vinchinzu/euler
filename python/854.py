"""
Project Euler Problem 854: Pisano Periods 2
"""

import sys
from math import gcd, isqrt

sys.setrecursionlimit(2000)

MOD = 1234567891
LIMIT = 1000000

def get_primes(n):
    sieve = [True] * (n + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, isqrt(n) + 1):
        if sieve[i]:
            for j in range(i*i, n + 1, i):
                sieve[j] = False
    return [i for i, is_prime in enumerate(sieve) if is_prime]

def get_spf(n):
    spf = list(range(n + 1))
    for i in range(2, isqrt(n) + 1):
        if spf[i] == i:
            for j in range(i*i, n + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf

def get_factors(n, spf):
    factors = {}
    while n > 1:
        p = spf[n]
        count = 0
        while n % p == 0:
            n //= p
            count += 1
        factors[p] = count
    return factors

def get_divisors_from_factors(factors):
    divs = [1]
    for p, e in factors.items():
        new_divs = []
        for d in divs:
            cur = 1
            for _ in range(e + 1):
                new_divs.append(d * cur)
                cur *= p
        divs = new_divs
    return sorted(divs)

def mat_mul(A, B, m):
    return (
        (A[0]*B[0] + A[1]*B[2]) % m,
        (A[0]*B[1] + A[1]*B[3]) % m,
        (A[2]*B[0] + A[3]*B[2]) % m,
        (A[2]*B[1] + A[3]*B[3]) % m
    )

def mat_pow(A, p, m):
    res = (1, 0, 0, 1)
    while p > 0:
        if p % 2 == 1:
            res = mat_mul(res, A, m)
        A = mat_mul(A, A, m)
        p //= 2
    return res

def get_fib_n_mod_m(n, m):
    if n == 0: return 0
    if n == 1: return 1 % m
    M = (1, 1, 1, 0)
    res = mat_pow(M, n - 1, m)
    return res[0]

def get_z_rank(p, spf_limit, spf):
    if p == 2: return 3
    if p == 5: return 5
    
    rem = p % 5
    if rem == 1 or rem == 4:
        D = p - 1
    else:
        D = 2 * (p + 1)
    
    factors = {}
    temp = D
    while temp > 1 and temp <= spf_limit:
        pr = spf[temp]
        cnt = 0
        while temp % pr == 0:
            temp //= pr
            cnt += 1
        factors[pr] = factors.get(pr, 0) + cnt
    
    divs = get_divisors_from_factors(factors)
    
    for d in divs:
        if get_fib_n_mod_m(d, p) == 0:
            return d
    return D

def get_ratio(m):
    if m == 3: return 1
    if m % 2 != 0: return 4
    if m % 4 == 2: return 1
    if m % 4 == 0: return 2
    return 0

def mod_inv(a, m):
    return pow(a, m - 2, m)

def get_v2(n):
    c = 0
    while n > 0 and n % 2 == 0:
        c += 1
        n //= 2
    return c

def get_vp(n, p):
    c = 0
    while n > 0 and n % p == 0:
        c += 1
        n //= p
    return c

def get_v2_F(m):
    if m % 3 != 0: return 0
    if m % 6 != 0: return 1
    return get_v2(m) + 2

def get_vp_F(m, z, p):
    return 1 + get_vp(m // z, p)

def solve():
    SPF_LIMIT = 2000100
    print(f"Generating SPF up to {SPF_LIMIT}...", file=sys.stderr)
    spf = get_spf(SPF_LIMIT)
    primes = [i for i, p in enumerate(spf) if p == i and i >= 2]
    
    small_primes = [p for p in primes if p <= LIMIT]
    print(f"Found {len(small_primes)} small primes.", file=sys.stderr)
    
    print("Computing z(r)...", file=sys.stderr)
    z_vals = {}
    for p in small_primes:
        z_vals[p] = get_z_rank(p, SPF_LIMIT, spf)
        
    updates_by_period = {}
    
    print("Generating prime power updates...", file=sys.stderr)
    for p in small_primes:
        if p == 2:
            # Special handling for p=2
            # pi(2^k) = 3 * 2^(k-1)
            # Powers: 2 (per 3), 4 (per 6), 8 (per 12)...
            period = 3
            while period <= LIMIT:
                if period not in updates_by_period:
                    updates_by_period[period] = 1
                updates_by_period[period] = (updates_by_period[period] * 2) % MOD
                period *= 2
        else:
            z = z_vals[p]
            curr_p = p
            curr_z = z
            while True:
                ratio = get_ratio(curr_z)
                period = curr_z * ratio
                if period > LIMIT:
                    break
                if period not in updates_by_period:
                    updates_by_period[period] = 1
                updates_by_period[period] = (updates_by_period[period] * p) % MOD
                
                curr_p *= p
                curr_z *= p
                if curr_z > LIMIT and period * p > LIMIT:
                    break
    
    print("Computing G_m...", file=sys.stderr)
    G = [0] * (LIMIT + 1)
    a, b = 0, 1
    for i in range(2, LIMIT + 1):
        a, b = b, (a + b) % MOD
        G[i] = b
    G[1] = 1
    
    print("  Sieving G_m...", file=sys.stderr)
    for i in range(1, LIMIT + 1):
        if G[i] <= 1: continue
        g_val = G[i]
        inv_g = mod_inv(g_val, MOD)
        for j in range(2 * i, LIMIT + 1, i):
            G[j] = (G[j] * inv_g) % MOD
            
    print("  Removing small primes from G_m...", file=sys.stderr)
    for p in small_primes:
        z = z_vals[p]
        max_k = LIMIT // z
        if max_k == 0: continue
        
        counts = [0] * (max_k + 1)
        
        if p == 2:
            for k in range(1, max_k + 1):
                counts[k] = get_v2_F(k * z)
        else:
            for k in range(1, max_k + 1):
                counts[k] = get_vp_F(k * z, z, p)
                
        for k in range(1, max_k + 1):
            if counts[k] == 0: continue
            c = counts[k]
            for m in range(2 * k, max_k + 1, k):
                counts[m] -= c
        
        inv_p = mod_inv(p, MOD)
        for k in range(1, max_k + 1):
            c = counts[k]
            if c > 0:
                m = k * z
                if G[m] > 0:
                    term = pow(inv_p, c, MOD)
                    G[m] = (G[m] * term) % MOD

    print("  Adding large G_m updates...", file=sys.stderr)
    for m in range(1, LIMIT + 1):
        if G[m] > 1:
            ratio = get_ratio(m)
            period = m * ratio
            if period <= LIMIT:
                if period not in updates_by_period:
                    updates_by_period[period] = 1
                updates_by_period[period] = (updates_by_period[period] * G[m]) % MOD

    print("Executing updates...", file=sys.stderr)
    M = [1] * (LIMIT + 1)
    L = [1] * (LIMIT + 1)
    
    sorted_periods = sorted(updates_by_period.keys())
    for w in sorted_periods:
        val = updates_by_period[w]
        if val == 1: continue
        for p in range(w, LIMIT + 1, w):
            M[p] = (M[p] * val) % MOD
            x = L[p]
            if x % w == 0: continue
            if w % x == 0:
                L[p] = w
            else:
                L[p] = (x * w) // gcd(x, w)
                
    print("Calculating result...", file=sys.stderr)
    total_prod = 1
    for p in range(1, LIMIT + 1):
        if L[p] == p:
            total_prod = (total_prod * M[p]) % MOD
            
    print(total_prod)

if __name__ == "__main__":
    solve()
