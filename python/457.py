"""Project Euler Problem 457: Quadratic residues modulo p².

Let f(n) = n² - 3n - 1, and let R(p) be the smallest positive integer n such
that f(n) ≡ 0 (mod p²) (or 0 if no integer exists). Find Σ R(p) for all primes
p ≤ N.
"""

from __future__ import annotations

from math import isqrt


def sieve_primes(limit: int) -> list[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def sq(n: int) -> int:
    """Square."""
    return n * n


def is_sq(a: int, p: int) -> bool:
    """Check if a is a quadratic residue modulo p."""
    return pow(a, (p - 1) // 2, p) == 1


def sqrt_mod(a: int, p: int) -> int:
    """Tonelli-Shanks algorithm for modular square root."""
    if p == 2:
        return a % 2
    if pow(a, (p - 1) // 2, p) != 1:
        raise ValueError("Not a quadratic residue")

    # Find Q and S such that p-1 = Q * 2^S
    Q = p - 1
    S = 0
    while Q % 2 == 0:
        Q //= 2
        S += 1

    # Find a quadratic non-residue z
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1

    M = S
    c = pow(z, Q, p)
    t = pow(a, Q, p)
    R = pow(a, (Q + 1) // 2, p)

    while t != 1:
        tt = t
        i = 0
        while i < M and tt != 1:
            tt = (tt * tt) % p
            i += 1
        b = pow(c, 1 << (M - i - 1), p)
        M = i
        c = (b * b) % p
        t = (t * c) % p
        R = (R * b) % p

    return R


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def f(n: int) -> int:
    """Function f(n) = n² - 3n - 1."""
    return sq(n) - 3 * n - 1


def R(p: int) -> int:
    """Compute R(p)."""
    if p <= 13:
        for n in range(1, sq(p) + 1):
            if f(n) % sq(p) == 0:
                return n
        return 0

    if not is_sq(13, p):
        return 0

    sqrt_val = sqrt_mod(13, p)
    R_min = float("inf")

    for sign in [-1, 1]:
        n_val = ((3 + sign * sqrt_val) * (p + 1) // 2) % p
        k = (f(n_val) // p * mod_inv(3 - 2 * n_val, p)) % p
        candidate = k * p + n_val
        if candidate < R_min:
            R_min = candidate

    return int(R_min) if R_min != float("inf") else 0


def solve() -> int:
    """Solve Problem 457."""
    N = 10**7
    primes = sieve_primes(N)
    ans = 0

    for p in primes:
        if p >= 2:
            ans += R(p)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
