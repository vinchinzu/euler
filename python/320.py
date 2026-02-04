"""Project Euler Problem 320 - Factorials divisible by large power.

Find sum_{i=10}^{1000000} N(i) mod 10^18, where N(i) is the smallest n
such that n! is divisible by (i!)^1234567890.
"""
from math import isqrt

def solve():
    MOD = 10**18
    K = 1234567890
    MIN_I = 10
    MAX_I = 1000000

    # Sieve smallest prime factor
    spf = list(range(MAX_I + 1))
    for i in range(2, isqrt(MAX_I) + 1):
        if spf[i] == i:
            for j in range(i * i, MAX_I + 1, i):
                if spf[j] == j:
                    spf[j] = i

    # Sieve to get exponent of each prime in i! as i increases
    # primes up to MAX_I
    primes = []
    for i in range(2, MAX_I + 1):
        if spf[i] == i:
            primes.append(i)

    # For each prime, track cumulative exponent in i!
    # Use a flat array indexed by prime's position
    np = len(primes)
    pidx = [0] * (MAX_I + 1)  # pidx[p] = index of p in primes array
    for i, p in enumerate(primes):
        pidx[p] = i

    exp_f = [0] * np  # exp_f[idx] = exponent of primes[idx] in current i!

    # Pre-accumulate for 2..MIN_I-1
    for j in range(2, MIN_I):
        n = j
        while n > 1:
            p = spf[n]
            idx = pidx[p]
            while n % p == 0:
                n //= p
                exp_f[idx] += 1

    max_n = 0
    ans = 0

    for i in range(MIN_I, MAX_I + 1):
        # Factorize i, update exp_f
        n = i
        changed = []
        while n > 1:
            p = spf[n]
            idx = pidx[p]
            while n % p == 0:
                n //= p
                exp_f[idx] += 1
            changed.append((p, idx))

        for p, idx in changed:
            num_needed = K * exp_f[idx]
            n_p = (p - 1) * num_needed
            n_p -= n_p % p
            # Compute legendre(n_p, p)
            cur = 0
            pk = p
            while pk <= n_p:
                cur += n_p // pk
                pk *= p
            # Increment until we have enough
            while cur < num_needed:
                n_p += p
                k = n_p
                while k % p == 0:
                    cur += 1
                    k //= p
            if n_p > max_n:
                max_n = n_p

        ans = (ans + max_n) % MOD

    return ans

if __name__ == "__main__":
    print(solve())
