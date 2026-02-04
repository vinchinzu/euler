#!/usr/bin/env python3
"""
Project Euler 827 - Pythagorean Triple Occurrence

Q(n) = smallest number occurring in exactly n Pythagorean triples.
Find sum_{k=1}^{18} Q(10^k) mod 409120391.
"""

import math
from sympy import factorint

MOD = 409120391

def sieve(limit):
    is_p = bytearray(b'\x01') * (limit + 1)
    is_p[0] = is_p[1] = 0
    for i in range(2, int(limit**0.5) + 1):
        if is_p[i]:
            is_p[i*i::i] = bytearray(len(is_p[i*i::i]))
    return [i for i in range(2, limit + 1) if is_p[i]]

PRIMES = sieve(500)
PRIMES_1MOD4 = [p for p in PRIMES if p % 4 == 1]
PRIMES_3MOD4 = [p for p in PRIMES if p % 4 == 3]
LOG_P1 = [math.log(p) for p in PRIMES_1MOD4]
LOG_P3 = [math.log(p) for p in PRIMES_3MOD4]
LOG2 = math.log(2)

# Cache for factorizations and min_number results
_factor_cache = {}
_min_cache = {}

def fast_factor(n):
    """Fast factorization using sympy (Pollard rho etc), with cache."""
    if n in _factor_cache:
        return _factor_cache[n]
    result = factorint(n)
    _factor_cache[n] = result
    return result

def get_odd_divisors_of(n):
    """Get all odd divisors of n, sorted."""
    while n % 2 == 0:
        n //= 2
    if n == 1:
        return [1]
    fdict = fast_factor(n)
    divs = [1]
    for p, e in fdict.items():
        new_divs = []
        for dv in divs:
            pk = 1
            for _ in range(e + 1):
                new_divs.append(dv * pk)
                pk *= p
        divs = new_divs
    return sorted(divs)

def divisors_of(n):
    """Get all divisors of n >= 1, sorted."""
    if n == 1:
        return [1]
    fdict = fast_factor(n)
    divs = [1]
    for p, e in fdict.items():
        new_divs = []
        for dv in divs:
            pk = 1
            for _ in range(e + 1):
                new_divs.append(dv * pk)
                pk *= p
        divs = new_divs
    return sorted(divs)

def ordered_factorizations(n, min_val=3):
    """Generate all ways to write n as product of factors >= min_val (non-decreasing).
    n must be odd >= 1.
    """
    if n == 1:
        yield ()
        return
    if n < min_val:
        return

    # Get all divisors of n that are >= min_val and <= sqrt(n), plus n itself
    all_divs = divisors_of(n)

    for d in all_divs:
        if d < min_val:
            continue
        if d == n:
            yield (n,)
        elif n % d == 0 and d * d <= n:
            for rest in ordered_factorizations(n // d, d):
                yield (d,) + rest

def min_number_for_shape(n, log_primes, mod_primes, budget):
    """Find factorization of odd n into parts >= 3 minimizing
    prod(primes[i]^((f_i-1)/2)). Returns (log_val, mod_val) or None."""
    key = (n, id(log_primes), budget)
    if key in _min_cache:
        return _min_cache[key]

    if n == 1:
        return (0.0, 1)
    if n < 3 or n % 2 == 0:
        return None

    nprimes = len(log_primes)
    best_log = budget
    best_mod_val = 0
    found = False

    for factors in ordered_factorizations(n, 3):
        k = len(factors)
        if k > nprimes:
            continue
        exps = sorted([(f - 1) // 2 for f in factors], reverse=True)
        log_val = sum(exps[i] * log_primes[i] for i in range(k))
        if log_val < best_log:
            best_log = log_val
            mod_val = 1
            for i in range(k):
                mod_val = mod_val * pow(mod_primes[i], exps[i], MOD) % MOD
            best_mod_val = mod_val
            found = True

    if found:
        result = (best_log, best_mod_val)
        # Don't cache with budget since budget varies
        return result
    return None

def solve():
    N = 18
    total = 0

    for k in range(1, N + 1):
        T = 10 ** k
        target = 2 * T + 2

        best_log = float('inf')
        best_mod = 0

        odd_divs = get_odd_divisors_of(target)

        for A in odd_divs:
            D = target // A
            Dm1 = D - 1

            # Best representation of A using primes 1 mod 4
            if A == 1:
                logA, modA = 0.0, 1
            else:
                res_A = min_number_for_shape(A, LOG_P1, PRIMES_1MOD4, best_log)
                if res_A is None:
                    continue
                logA, modA = res_A

            if logA >= best_log:
                continue

            # Case 1: odd m (a0=0), B = Dm1
            if Dm1 == 1:
                if logA < best_log:
                    best_log = logA
                    best_mod = modA
            elif Dm1 % 2 == 1:
                remaining = best_log - logA
                res_B = min_number_for_shape(Dm1, LOG_P3, PRIMES_3MOD4, remaining)
                if res_B is not None:
                    logB, modB = res_B
                    tl = logA + logB
                    if tl < best_log:
                        best_log = tl
                        best_mod = (modA * modB) % MOD

            # Case 2: even m, Dm1 = C * B, C = 2*a0-1
            if Dm1 >= 1 and Dm1 % 2 == 1:
                c_divs = get_odd_divisors_of(Dm1)
                for C in c_divs:
                    B = Dm1 // C
                    a0 = (C + 1) // 2
                    log2_part = a0 * LOG2

                    if logA + log2_part >= best_log:
                        continue

                    remaining = best_log - logA - log2_part
                    mod2 = pow(2, a0, MOD)

                    if B == 1:
                        tl = logA + log2_part
                        if tl < best_log:
                            best_log = tl
                            best_mod = (modA * mod2) % MOD
                    elif B % 2 == 1:
                        res_B = min_number_for_shape(B, LOG_P3, PRIMES_3MOD4, remaining)
                        if res_B is not None:
                            logB, modB = res_B
                            tl = logA + log2_part + logB
                            if tl < best_log:
                                best_log = tl
                                best_mod = (modA * mod2 % MOD * modB) % MOD

        total = (total + best_mod) % MOD

    return total

if __name__ == "__main__":
    print(solve())
