"""Project Euler Problem 388 - Distinct Lines through lattice points.

D(N) = sum_{d=1}^N mu(d) * (floor(N/d) + 1)^3 - M(N)
where M(N) is the Mertens function.

Uses O(N^{2/3}) Mertens function via Lucy DP, then quotient grouping for the main sum.
"""

from math import isqrt


def solve(N: int) -> int:
    # Sieve limit: N^{2/3}
    cbrt = int(round(N ** (1.0 / 3.0)))
    while (cbrt + 1) ** 3 <= N:
        cbrt += 1
    while cbrt ** 3 > N:
        cbrt -= 1
    sieve_limit = max(cbrt * cbrt, isqrt(N) + 1)
    # Ensure sieve_limit is at least sqrt(N) for correctness
    sqrtN = isqrt(N)
    if sieve_limit < sqrtN + 1:
        sieve_limit = sqrtN + 1

    # Linear sieve for mu
    mu = [0] * (sieve_limit + 1)
    is_composite = bytearray(sieve_limit + 1)
    primes = []
    mu[1] = 1
    for i in range(2, sieve_limit + 1):
        if not is_composite[i]:
            primes.append(i)
            mu[i] = -1
        for p in primes:
            v = p * i
            if v > sieve_limit:
                break
            is_composite[v] = 1
            if i % p == 0:
                mu[v] = 0
                break
            mu[v] = -mu[i]

    # Prefix sums of mu
    mu_prefix = [0] * (sieve_limit + 1)
    for i in range(1, sieve_limit + 1):
        mu_prefix[i] = mu_prefix[i - 1] + mu[i]

    # Compute Mertens function M(n) for all quotient values of N
    # Quotient values: N//1, N//2, ..., N//sqrtN, and 1, 2, ..., sqrtN
    # For small values (<= sieve_limit), M(n) = mu_prefix[n]
    # For large values, use: M(n) = 1 - sum_{d=2}^n M(floor(n/d))

    # We need M(v) for all v = N//k. Store in dict.
    mertens_cache = {}

    def mertens(n):
        if n <= sieve_limit:
            return mu_prefix[n]
        if n in mertens_cache:
            return mertens_cache[n]

        # M(n) = 1 - sum_{d=2}^{n} M(floor(n/d))
        s = 0
        d = 2
        while d <= n:
            q = n // d
            # Find the range of d values giving the same quotient
            d_max = n // q
            s += (d_max - d + 1) * mertens(q)
            d = d_max + 1

        result = 1 - s
        mertens_cache[n] = result
        return result

    # Compute M(N) first (this fills the cache for all needed quotient values)
    M_N = mertens(N)

    # Main sum: sum_{d=1}^N mu(d) * (floor(N/d) + 1)^3
    # Group by quotient q = floor(N/d)
    main_sum = 0
    d = 1
    while d <= N:
        q = N // d
        d_max = N // q
        # sum of mu[d] for d in [d, d_max]
        if d_max <= sieve_limit:
            mu_range_sum = mu_prefix[d_max] - mu_prefix[d - 1]
        elif d <= sieve_limit:
            # Split: [d, sieve_limit] from prefix sums, [sieve_limit+1, d_max] from Mertens
            mu_range_sum = mu_prefix[sieve_limit] - mu_prefix[d - 1]
            # M(d_max) - M(sieve_limit) = sum of mu for [sieve_limit+1, d_max]
            mu_range_sum += mertens(d_max) - mu_prefix[sieve_limit]
        else:
            # Both endpoints > sieve_limit
            # sum mu[d..d_max] = M(d_max) - M(d-1)
            mu_range_sum = mertens(d_max) - mertens(d - 1)

        main_sum += mu_range_sum * (q + 1) ** 3
        d = d_max + 1

    return main_sum - M_N


if __name__ == "__main__":
    result = solve(10**10)
    s = str(result)
    print(s[:9] + s[-9:])
