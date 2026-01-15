"""Project Euler Problem 621: Expressing an integer as the sum of three
triangular numbers.

Find the number of ways to write N as the sum of three triangular numbers
(permutations are considered distinct).

To compute the number of ways to write X = N - k(k+1)/2 as the sum of two
triangular numbers, we write:
a(a+1)/2 + b(b+1)/2 = X
=> (2a+1)² + (2b+1)² = 8X+2 = 8N - 4k(k+1) + 2,

and the number of ways to write a value as the sum of two squares can be
computed efficiently from its prime factorization.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def is_square_mod(n: int, p: int) -> bool:
    """Check if n is a quadratic residue modulo p."""
    return pow(n, (p - 1) // 2, p) == 1


def sqrt_mod(n: int, p: int) -> int:
    """Tonelli-Shanks algorithm for modular square root."""
    if p == 2:
        return n % 2
    if not is_square_mod(n, p):
        return 0
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    # For p % 4 == 1, use Tonelli-Shanks
    q = p - 1
    s = 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while is_square_mod(z, p):
        z += 1
    m = s
    c = pow(z, q, p)
    t = pow(n, q, p)
    r = pow(n, (q + 1) // 2, p)
    while t != 1:
        tt = t
        i = 1
        while i < m and pow(tt, 2**i, p) != 1:
            i += 1
        b = pow(c, 2 ** (m - i - 1), p)
        m = i
        c = (b * b) % p
        t = (t * c) % p
        r = (r * b) % p
    return r


def solve() -> int:
    """Solve Problem 621."""
    N = 17526 * 10**9
    L = int((isqrt(8 * N + 1) - 1) / 2)

    targets = [0] * (L + 1)
    num_sums = [1] * (L + 1)

    for k in range(L + 1):
        targets[k] = 8 * N - 4 * L * k * (k + 1) + 2
        while targets[k] % 2 == 0:
            targets[k] //= 2

    primes = list(primerange(3, 2 * L + 1))
    for p in primes:
        if not is_square_mod(8 * N + 3, p):
            continue
        sqrt_val = sqrt_mod(8 * N + 3, p)
        for signed_sqrt in [sqrt_val, p - sqrt_val]:
            k_start = ((p + signed_sqrt - 1) * ((p + 1) // 2)) % p
            k = k_start
            while k <= L:
                e = 0
                temp = targets[k]
                while temp % p == 0:
                    e += 1
                    temp //= p
                targets[k] = temp
                if p % 4 == 1:
                    num_sums[k] *= e + 1
                elif e % 2 == 1:
                    num_sums[k] = 0
                k += p

    for k in range(L + 1):
        if targets[k] % 4 == 3:
            num_sums[k] = 0
        elif targets[k] > 1:
            num_sums[k] *= 2

    return sum(num_sums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
