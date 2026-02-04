"""Project Euler Problem 243: Resilience.

Find the smallest number d such that the proportion of proper fractions with
denominator d that are reduced is smaller than R.
"""

from math import isqrt
import sys

sys.setrecursionlimit(100000)


def sieve(limit):
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve():
    """Solve Problem 243.

    Find smallest d where phi(d)/(d-1) < 15499/94744.

    Strategy: d should be of the form primorial * k where k is a product of
    small primes. We search by building d from prime factors, using the primorial
    as a starting point and trying multiples.
    """
    R_num = 15499
    R_den = 94744

    primes_list = sieve(100)

    # First, find the smallest primorial P# such that phi(P#)/(P#-1) < R
    # phi(P#) = product of (p-1) for primes p dividing P#
    # For primorial, phi(P#)/P# = product((p-1)/p)
    prod = 1
    phi = 1
    base_index = 0
    for i, p in enumerate(primes_list):
        prod *= p
        phi *= (p - 1)
        # Check if phi/(prod-1) < R_num/R_den
        # i.e., phi * R_den < R_num * (prod - 1)
        if phi * R_den < R_num * (prod - 1):
            base_index = i
            break

    # Now prod is the smallest primorial satisfying the condition.
    # But we can potentially find a smaller d by using the previous primorial
    # multiplied by a small prime power.
    # Go back one step: use primorial of first (base_index) primes
    # and multiply by powers of those primes.

    ans = prod  # Current best: the primorial itself

    # Try: primorial of first base_index primes * k, for various k
    # The previous primorial (without the last prime) doesn't satisfy,
    # but multiplied by some factor it might.
    prev_prod = prod // primes_list[base_index]
    prev_phi = phi // (primes_list[base_index] - 1)

    # Try multiplying prev_prod by powers of primes <= primes_list[base_index]
    for i in range(base_index):
        p = primes_list[i]
        test_prod = prev_prod * p
        test_phi = prev_phi * p  # phi(n*p) when p already divides n = phi(n)*p
        if test_phi * R_den < R_num * (test_prod - 1):
            ans = min(ans, test_prod)

    # More thorough: try all multiples of prev_prod
    # d = prev_prod * m where m is composed of primes <= primes_list[base_index]
    # phi(d) = prev_phi * phi_factor(m)
    # We need prev_phi * phi_factor(m) / (prev_prod * m - 1) < R_num / R_den

    def search(idx, m, phi_m):
        """Search for d = prev_prod * m with phi(d) = prev_phi * phi_m."""
        nonlocal ans
        d = prev_prod * m
        if d >= ans:
            return
        phi_d = prev_phi * phi_m
        # Check: phi_d * R_den < R_num * (d - 1)
        if phi_d * R_den < R_num * (d - 1):
            ans = d
            return
        # Try multiplying by primes[0..base_index]
        for i in range(idx, base_index + 1):
            if i >= len(primes_list):
                break
            p = primes_list[i]
            new_m = m * p
            if prev_prod * new_m >= ans:
                break
            # phi contribution: if p already divides m, phi(m*p) = phi(m)*p
            # if p doesn't divide m, phi(m*p) = phi(m)*(p-1)
            # Since we're building up by multiplying, and prev_prod already contains p,
            # the phi factor for the additional p is just p (since p | prev_prod*m already)
            new_phi_m = phi_m * p
            search(i, new_m, new_phi_m)

    search(0, 1, 1)

    return ans


if __name__ == "__main__":
    print(solve())
