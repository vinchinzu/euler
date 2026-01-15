"""Project Euler Problem 652: Distinct values of logarithms.

Find the number of distinct values of log_m(n) for 2 ≤ m,n ≤ N.

There are (N - 1)² ways to choose m and n, but there are duplicates that we
need to subtract:
- log_m(n) = 1 if m = n
- If m and n are both k-th powers, then log_m(n) = log_{ᵏ√m}(ᵏ√n)
- If n is a k-th power of m ≥ 3, then log_m(n) is identical to the value
  when m = 2 and n = 2^k
- Pairs of powers with exponents that are relatively prime
"""

from __future__ import annotations

from math import gcd, isqrt

from sympy import mobius, primerange


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


def nth_root(n: int, k: int) -> int:
    """Compute k-th root of n (floor)."""
    if k == 1:
        return n
    if k == 2:
        return isqrt(n)
    low, high = 1, n
    while low < high:
        mid = (low + high + 1) // 2
        power = pow(mid, k)
        if power <= n:
            low = mid
        else:
            high = mid - 1
    return low


def nCr(n: int, k: int) -> int:
    """Binomial coefficient."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = result * (n - i) // (i + 1)
    return result


def pre_mobius(max_k: int) -> list[int]:
    """Precompute Möbius function."""
    mu = [1] * (max_k + 1)
    mu[0] = 0
    primes = list(primerange(2, max_k + 1))
    for p in primes:
        for j in range(p, max_k + 1, p):
            mu[j] = -mu[j]
        for j in range(p * p, max_k + 1, p * p):
            mu[j] = 0
    return mu


def solve() -> int:
    """Solve Problem 652."""
    N = 10**18
    M = 10**9

    max_k = 60  # log_3(10^18) ≈ 37
    mobius_vals = pre_mobius(max_k)

    ans = ((N - 1) ** 2 - (N - 2)) % M

    # Subtract k-th powers
    for k in range(2, max_k + 1):
        if pow(3, k) > N:
            break
        num_powers = nth_root(N, k) - 1
        ans = (ans + 2 * mobius_vals[k] * nCr(num_powers, 2)) % M

    # Subtract cases where n is a k-th power of m ≥ 3
    for k in range(2, max_k + 1):
        if pow(3, k) > N:
            break
        ans = (ans - 2 * (nth_root(N, k) - 2)) % M
        e = 2
        while pow(2, k * e) <= N:
            ans = (ans - 2 * mobius_vals[e] * (nth_root(N, k * e) - 1)) % M
            e += 1

    # Subtract pairs of powers with relatively prime exponents
    max_b = int(N ** (1 / 3)) + 1
    is_perfect_power = [False] * (max_b + 1)
    for b in range(2, max_b + 1):
        if pow(b, 6) > N:
            break
        e = 2
        while pow(b, 3 * e) <= N:
            power_val = pow(b, e)
            if power_val <= max_b:
                is_perfect_power[power_val] = True
            e += 1

    for b in range(3, max_b + 1):
        if pow(b, 3) > N:
            break
        if is_perfect_power[b]:
            continue
        largest_e = 1
        while pow(b, largest_e + 1) <= N:
            largest_e += 1
        for e1 in range(2, largest_e + 1):
            for e2 in range(2, largest_e + 1):
                if gcd(e1, e2) == 1:
                    ans = (ans - 1) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
