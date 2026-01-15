"""Project Euler Problem 758: Buckets of Water.

Let a≤b and GCD(a,b)=1. We have three buckets S,M,L with sizes a,b,a+b
liters respectively. S and M are initially full of water, and L is
initially empty. The only operation we can do is pour the water from one
bucket to another, only stopping when either the source is empty or the
destination is full. If P(a,b) is the minimal number of pourings until
one bucket has 1 liter, then find Σ P(2^{p^5} - 1, 2^{q^5} - 1) for all
primes p < q < N.

For the first step, we can pour either S into L or M into L. After that,
there is always only one pouring at any time that doesn't return to one
of the "base" states where both S and M are empty or full.

Consider the amount of empty space in both S and M combined. In the
beginning, there is 0. If we start by pouring S into L, we will
repeatedly pour from S -> L, M -> S, and L -> M. Note that each even
numbered pouring will be from M -> S (with no change in empty space), and
each odd numbered pouring is either S -> L or L -> M, which means we
either add a empty space or remove b empty space, which are both
equivalent to adding a (mod a+b). 

Since we need b-1 empty space in M to finish, the number of steps if we
start pouring from S -> L is 2k, where k*a ≡ b-1 (mod a+b). If we started
with M -> L, the value is similar: 2k, where k*b ≡ a-1 (mod a+b).
P(a,b) is the minimum of the two values.

We now need to compute this for large inputs P(2^e - 1, 2^f - 1). Note
that finding k satisfying k*b ≡ a-1 (mod a+b) is equivalent to finding
(k,x) such that k*b + (a+b)x = a-1, or a(1-x) + b(-k-x) = 1, so we can
solve ax'+by'=1 and find k = x'-y'-1. (Similarly, for k*a ≡ b-1
(mod a+b), we have k = y'-x'-1.)

To solve ax'+by'=1 for near powers of 2 (2^e - 1, 2^f - 1), we note
that when using the extended Euclidean algorithm, we need to compute
⌊a/b⌋, and here we have ⌊(2^e-1)/(2^f-1)⌋ = (2^e - 2^{e%f}) / (2^f - 1).
Also, since we perform operations (mod M), we need to keep track of
whether x'-y'-1 or y'-x'-1 is larger; we can see that it swaps on each
step of the Euclidean algorithm. This lets us compute the smaller k, and
summing over all P(2^{p^5} - 1, 2^{q^5} - 1) gives the answer.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd, isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


@dataclass
class Result:
    """Result of extended Euclidean algorithm."""

    x: int
    y: int
    sign: int


def lin_comb_near_pow2s(e: int, f: int, M: int) -> Result:
    """Extended Euclidean algorithm for near powers of 2."""
    if f == 0:
        return Result(1, 0, 1)

    prev = lin_comb_near_pow2s(f, e % f, M)

    # Compute ⌊(2^e-1)/(2^f-1)⌋ = (2^e - 2^{e%f}) / (2^f - 1)
    numerator = (pow_mod(2, e, M) - pow_mod(2, e % f, M)) % M
    denominator = (pow_mod(2, f, M) - 1) % M
    quotient = (numerator * mod_inverse(denominator, M)) % M

    new_y = (prev.x - quotient * prev.y) % M
    return Result(prev.y, new_y, -prev.sign)


def solve() -> int:
    """Solve Problem 758."""
    N = 1000
    M = 10**9 + 7

    primes_list = sieve_primes(N)
    ans = 0

    for i, p in enumerate(primes_list):
        for q in primes_list[i + 1 :]:
            e = p**5
            f = q**5
            result = lin_comb_near_pow2s(e, f, M)
            k = (result.sign * (result.x - result.y) - 1) % M
            ans = (ans + 2 * k) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
