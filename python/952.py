"""
Project Euler Problem 952: Order Modulo Factorial

Given a prime p and a positive integer n < p, let R(p, n) be
the multiplicative order of p modulo n!.
In other words, R(p, n) is the minimal positive integer r such that
p^r = 1 (mod n!)

For example, R(7, 4) = 2 and R(10^9 + 7, 12) = 17280.

Find R(10^9 + 7, 10^7). Give your answer modulo 10^9 + 7.
"""

import sys
from pathlib import Path

# Increase recursion depth just in case
sys.setrecursionlimit(2000)


def get_spf(n):
    """Computes smallest prime factor for each number up to n."""
    spf = list(range(n + 1))
    spf[0] = spf[1] = 0
    for i in range(2, int(n**0.5) + 1):
        if spf[i] == i:
            for j in range(i * i, n + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def get_unique_prime_factors(n, spf):
    """Returns a list of unique prime factors of n."""
    factors = []
    while n > 1:
        f = spf[n]
        factors.append(f)
        while n % f == 0:
            n //= f
    return factors


def power(a, b, m):
    """Computes (a^b) % m."""
    res = 1
    a %= m
    while b > 0:
        if b % 2 == 1:
            res = (res * a) % m
        a = (a * a) % m
        b //= 2
    return res


def get_ord(p, q, spf):
    """Computes multiplicative order of p modulo q.
    q must be prime. spf is used to factorize q-1."""
    if q == 2:
        return 1

    # Order divides q-1
    t = q - 1
    # Factors of q-1
    factors = get_unique_prime_factors(t, spf)

    curr_ord = t
    for f in factors:
        # Check if curr_ord / f is valid
        while curr_ord % f == 0 and power(p, curr_ord // f, q) == 1:
            curr_ord //= f
    return curr_ord


def get_legendre_valuation(n, p):
    """Computes the exponent of prime p in n!."""
    count = 0
    while n > 0:
        count += n // p
        n //= p
    return count


def solve(n, p, mod_ans):
    """
    Computes R(p, n) mod mod_ans.
    n: factorial limit (10^7)
    p: base (10^9 + 7)
    mod_ans: modulus for the answer (10^9 + 7)
    """

    # 1. Sieve
    spf = get_spf(n)
    primes = [i for i in range(2, n + 1) if spf[i] == i]

    # Array to store max exponent for each prime in the LCM
    # exponents[ell] will store v_ell(L)
    exponents = [0] * (n + 1)

    # 2. Handle q = 2 separately
    # Formula derived: v_2(L) = K_2 - 3 for K_2 >= 4
    K2 = get_legendre_valuation(n, 2)

    if K2 >= 4:
        exponents[2] = K2 - 3
    elif K2 >= 2:
        exponents[2] = 1

    # 3. Iterate odd primes q
    for q in primes:
        if q == 2:
            continue

        # a. Find d_q = ord_q(p)
        d_q = get_ord(p, q, spf)

        # b. Update B_ell for all prime factors of d_q
        # We need to factorize d_q.
        # d_q < q <= n, so we can use spf.
        temp = d_q
        while temp > 1:
            ell = spf[temp]
            count = 0
            while temp % ell == 0:
                count += 1
                temp //= ell
            if exponents[ell] < count:
                exponents[ell] = count

        # c. Find v_q = v_q(p^{d_q} - 1)
        # We know p^{d_q} = 1 mod q.
        # We check p^{d_q} mod q^2, mod q^3, etc.
        # We only care up to K_q because A_q = max(0, K_q - v_q).
        # If v_q >= K_q, then A_q = 0.

        K_q = get_legendre_valuation(n, q)
        if K_q == 0:  # Should not happen for q <= n
            continue

        v_q = 1
        curr_mod = q * q
        # Efficiently check valuation
        # Base check: p^{d_q} mod q^2
        rem = power(p, d_q, curr_mod)
        while rem == 1:
            v_q += 1
            if v_q >= K_q:
                break
            curr_mod *= q
            rem = power(p, d_q, curr_mod)

        A_q = max(0, K_q - v_q)
        if exponents[q] < A_q:
            exponents[q] = A_q

    # 4. Calculate final answer
    ans = 1
    for ell in range(2, n + 1):
        if exponents[ell] > 0:
            term = power(ell, exponents[ell], mod_ans)
            ans = (ans * term) % mod_ans

    return ans


if __name__ == "__main__":
    # Real problem
    # n = 10^7
    # p = 10^9 + 7
    # mod = 10^9 + 7
    res = solve(10**7, 10**9 + 7, 10**9 + 7)
    print(res)

    # Also write to answer.txt in the same directory as the script
    script_dir = Path(__file__).parent
    with open(script_dir / "answer.txt", "w") as f:
        f.write(str(res))
