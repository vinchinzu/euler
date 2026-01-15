"""Project Euler Problem 725: Digit Sum Numbers.

Find the sum of all DS-numbers, numbers where one digit is the sum of all
other digits, with N digits or less.

Given a digit d at place value k, by balls and bins there are nCr(d+N-2,d)
ways to choose the remaining N-1 digits to add up to d. This means that the
sum of all digits at place value k for numbers with largest digit d is
2d*nCr(d+N-2,d).

However, we've double counted numbers that only contain two non-zero digits
which are the same. For each place value k and largest digit d, there are
N-1 of these.

So the total is, in base B,
S(N) = (B^N-1)/(B-1) (n-1)(2nCr(B+N-2,B-2) - nCr(B,2)).
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
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
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def ncr(n: int, r: int, mod: int) -> int:
    """Binomial coefficient C(n, r) modulo mod."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = (result * (n - i) * mod_inv(i + 1, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 725."""
    n = 2020
    m = 10**16
    b = 10

    term1 = (pow_mod(b, n, m) - 1) * mod_inv(b - 1, m) % m
    term2 = (n - 1) % m
    term3 = (2 * ncr(b + n - 2, b - 2, m) - ncr(b, 2, m)) % m

    ans = term1 * term2 % m * term3 % m
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
