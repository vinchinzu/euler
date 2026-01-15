"""Project Euler Problem 541: Divisibility of Harmonic Numbers.

Find the largest n such that the nth harmonic number H_n has a denominator not
divisible by P when reduced to lowest terms.

This is equivalent to finding the largest n such that H_n is a p-adic integer.
Let n = a*P + b for 0 ≤ b < P. We have

H_n = Σ_{k=1}^{a*P+b} 1/k
    = Σ_{q=1}^a 1/(qP) + Σ_{r=1}^{P-1} Σ_{q=1}^L 1/(qP-r)        (L=a if b+r<P, otherwise L=a+1)
    = (1/P) Σ_q 1/i    - Σ_r Σ_q (1/r + qP/r² + (qP)²/r³ + ...)  (expanding (qP-r)⁻¹)
    ≡ (1/P) H_q        - Σ_r Σ_q 1/r Σ_{k=0}^{e-1} (qP/r)^k (mod P^e)
    ≡ (1/P) H_q        - Σ_r Σ_k 1/r (P/r)^k Σ_q q^k.

To compute H_n (mod P^e), we first need to compute H_q (mod P^{e+1}). If it is not divisible
by P, then we know H_n is not a p-adic integer.

Then we perform the summation in the second term. The only summation with large bounds is the
inner one over q, but we can compute it directly using the sumPowers formula.

To compute the answer, we note that if H_n is not a p-adic integer, then neither is H_{nP+b}
for 0 ≤ b < P. So we start with H_0 and for any p-adic integer H_n, check all H_{nP+b}.
The largest n we find is the answer.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from functools import lru_cache
from typing import Dict, Tuple


P = 137


@dataclass(frozen=True)
class Key:
    """Key for memoization cache."""

    n: int
    e: int


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)
    if base == 0:
        return 0
    base %= mod
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(a: int, m: int) -> int:
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
        msg = "Modular inverse does not exist"
        raise ValueError(msg)
    if t < 0:
        t += m
    return t


def mod(a: int, m: int) -> int:
    """Modular reduction."""
    return ((a % m) + m) % m


def pow_int(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def factorial(n: int) -> int:
    """Factorial."""
    if n < 0:
        msg = "Factorial undefined for negative numbers"
        raise ValueError(msg)
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


def sum_powers(limit: int, exp: int, mod_val: int) -> int:
    """Sum of powers: Σ_{j=1}^{limit} j^exp mod mod_val.

    Uses formula from generating functions.
    """
    if limit == 0:
        return 0
    if mod_val < 2**31:  # Use integer arithmetic for small mod
        sum_pows = 0
        result = 0
        for j in range(1, exp + 3):
            sum_pows = (sum_pows + pow_mod(j, exp, mod_val)) % mod_val
            res = sum_pows
            if (exp + j) % 2 != 0:
                res = (-res) % mod_val
            # Compute binomial coefficients modulo mod_val
            # Use iterative computation to avoid overflow
            denom = 1
            for k in range(1, j):
                denom = (denom * k) % mod_val
            for k in range(1, exp + 3 - j):
                denom = (denom * k) % mod_val
            inv_denom = mod_inv(denom, mod_val)
            res = (res * inv_denom) % mod_val
            for k in range(1, exp + 3):
                if k != j:
                    res = (res * ((limit - k) % mod_val)) % mod_val
            result = (result + res) % mod_val
        return result
    else:  # Use BigInteger arithmetic for large mod
        return bsum_powers(limit, exp, mod_val)


def bsum_powers(limit: int, exp: int, mod_val: int) -> int:
    """Sum of powers using BigInteger arithmetic."""
    sum_pows = 0
    result = 0
    for j in range(1, exp + 3):
        sum_pows = sum_pows + pow_int(j, exp)
        res = sum_pows
        if (exp + j) % 2 != 0:
            res = -res
        # Compute denominator iteratively
        denom = 1
        for k in range(1, j):
            denom = (denom * k) % mod_val
        for k in range(1, exp + 3 - j):
            denom = (denom * k) % mod_val
        inv_denom = mod_inv(denom, mod_val)
        res = (res % mod_val) * inv_denom % mod_val
        for k in range(1, exp + 3):
            if k != j:
                res = (res * ((limit - k) % mod_val)) % mod_val
        result = (result + res) % mod_val
    return result


cache: Dict[Key, int] = {}


def H(n: int, e: int) -> int:
    """Compute H_n mod P^e.

    Returns -1 if H_n is not a p-adic integer.
    """
    if n == 0:
        return 0

    key = Key(n, e)
    if key in cache:
        return cache[key]

    H_val = H(n // P, e + 1)
    if H_val == -1 or H_val % P != 0:
        cache[key] = -1
        return -1

    H_val //= P
    pe = pow_int(P, e)

    for r in range(1, P):
        L = n // P if (n % P + r < P) else n // P + 1
        if pe < 2**31:
            r_inv = mod_inv(r, pe)
            for k in range(e):
                term = (
                    r_inv
                    * pow_mod(P * r_inv, k, pe) % pe
                    * sum_powers(L, k, pe) % pe
                )
                H_val = (H_val - term) % pe
        else:
            r_inv_big = mod_inv(r, pe)
            for k in range(e):
                term_big = (
                    r_inv_big
                    * pow_mod(P * r_inv_big, k, pe)
                    * bsum_powers(L, k, pe)
                ) % pe
                H_val = (H_val - term_big) % pe

    result = mod(H_val, pe)
    cache[key] = result
    return result


def solve() -> int:
    """Solve Problem 541."""
    nums = [0]
    ans = 0
    while nums:
        n = nums.pop(0)
        if H(n * P, 0) == 0:
            start = 1 if n == 0 else 0
            for i in range(start, P):
                nums.append(n * P + i)
        ans = n
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
