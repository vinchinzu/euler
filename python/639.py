#!/usr/bin/env python3
"""Project Euler Problem 639: Summing a multiplicative function.

Find sum_{k=1}^{K} sum_{i=1}^{N} f_k(i), where f_k is multiplicative with f_k(p) = p^k.
Uses powerful number iteration with Lagrange interpolation for power sums.
"""

import math


def solve():
    N = 10**12
    K = 50
    M = 10**9 + 7
    L = int(math.isqrt(N))

    # Sieve primes up to L
    is_prime = bytearray(L + 1)
    is_prime[2] = 1
    for i in range(3, L + 1, 2):
        is_prime[i] = 1
    for i in range(3, int(math.isqrt(L)) + 1, 2):
        if is_prime[i]:
            for j in range(i * i, L + 1, 2 * i):
                is_prime[j] = 0
    primes = [i for i in range(2, L + 1) if is_prime[i]]

    # Precompute factorial inverses for Lagrange interpolation
    max_deg = K + 2  # need K+2 points for degree K+1 polynomial
    fact = [1] * (max_deg + 1)
    for i in range(1, max_deg + 1):
        fact[i] = fact[i - 1] * i % M
    inv_fact = [1] * (max_deg + 1)
    inv_fact[max_deg] = pow(fact[max_deg], M - 2, M)
    for i in range(max_deg - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    # Arrays updated incrementally across k iterations
    nth_pows = [1] * (L + 1)  # nth_pows[i] = i^k mod M after k-th iteration
    sum_powers = [0] * (L + 1)  # sum_powers[i] = sum_{j=1}^{i} j^k mod M
    sum_coeffs = [0] * (L + 1)  # sum_coeffs[i] = sum over primes p<=i of p^k*(1-p^k)

    def sum_kth_powers(n, k):
        """Compute sum_{i=1}^{n} i^k mod M using Lagrange interpolation."""
        if n <= L:
            return sum_powers[n]
        pts = k + 2  # number of sample points (polynomial degree = k+1)
        # Use points x=0,1,...,k+1 with y=sum_powers[0..k+1]
        prefix = [1] * (pts + 1)
        for j in range(pts):
            prefix[j + 1] = prefix[j] * ((n - j) % M) % M
        suffix = [1] * (pts + 1)
        for j in range(pts - 1, -1, -1):
            suffix[j] = suffix[j + 1] * ((n - j) % M) % M
        result = 0
        m = k + 1  # degree
        for i in range(pts):
            numer = prefix[i] * suffix[i + 1] % M * sum_powers[i] % M
            denom = inv_fact[i] * inv_fact[m - i] % M
            if (m - i) % 2 == 1:
                denom = M - denom
            result = (result + numer * denom) % M
        return result

    ans = 0

    for k in range(1, K + 1):
        sum_powers[0] = 0
        sum_coeffs[0] = 0
        for i in range(1, L + 1):
            nth_pows[i] = nth_pows[i] * i % M
            sum_powers[i] = (sum_powers[i - 1] + nth_pows[i]) % M
            if is_prime[i]:
                coeff = nth_pows[i] * (1 - nth_pows[i]) % M
            else:
                coeff = 0
            sum_coeffs[i] = (sum_coeffs[i - 1] + coeff) % M

        # Stack-based iteration over powerful numbers
        # State: (min_index, d, mult, prev_e)
        stack = [(0, 1, 1, 0)]

        while stack:
            min_idx, d, mult, prev_e = stack.pop()
            n = N // d

            # Part 1: contribution from n itself (if not already counted)
            if prev_e != 2:
                sp = sum_kth_powers(n, k)
                ans = (ans + sp * mult) % M

            lim = int(round(n ** (1.0 / 3)))
            while lim * lim * lim > n:
                lim -= 1
            while (lim + 1) ** 3 <= n:
                lim += 1

            # Part 2: sum over individual primes p (exponent 2 contribution)
            for i in range(min_idx, len(primes)):
                p = primes[i]
                if p * p > n // max(lim, 1):
                    break
                q = n // (p * p)
                sp_q = sum_kth_powers(q, k)
                coeff = nth_pows[p] * (1 - nth_pows[p]) % M
                ans = (ans + sp_q * mult % M * coeff) % M

            # Part 3: sum over ranges of primes using sum_coeffs
            p_min = primes[min_idx] if min_idx < len(primes) else L + 1
            for q in range(1, lim):
                high = int(math.isqrt(n // q))
                low = max(int(math.isqrt(n // (q + 1))), p_min - 1)
                if high > L:
                    high = L
                if high >= low:
                    coeff_sum = (sum_coeffs[high] - sum_coeffs[low]) % M
                    ans = (ans + sum_powers[q] * mult % M * coeff_sum) % M
                else:
                    break

            # Part 4: recurse with higher prime powers (exponent >= 3)
            for idx in range(min_idx, len(primes)):
                p = primes[idx]
                if d * p * p * p > N:
                    break
                new_d = d * p
                new_mult = mult * nth_pows[p] % M * (1 - nth_pows[p]) % M
                e = 1
                while new_d * p <= N:
                    new_d *= p
                    e += 1
                    stack.append((idx + 1, new_d, new_mult, e))

    return ans % M


if __name__ == "__main__":
    print(solve())
