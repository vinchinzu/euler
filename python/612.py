"""Project Euler Problem 612: Friend numbers.

Find the number of pairs (p,q) with 1≤p<q<10^N such that p and q share at
least one common digit.

Let f(bitset) be the number of integers less than 10^N that have exactly the
given digits. To compute this, we start with the integers that have a subset
of the given digits, Σ_{d=1}^N n^d (where n is the number of distinct digits)
minus Σ_{d=1}^N n^{d-1} if the digits includes 0, to avoid counting numbers
starting with 0. Then we subtract the integers that have exactly any strict
subset of those digits.

To compute the answer, we take all pairs of digit bitsets that share at least
one digit, and add f(bitset1, bitset2) for each pair. This also counts numbers
p=q and p>q, so we need to subtract the 10^N - 1 pairs of p=q and then divide
by 2.
"""

from __future__ import annotations


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


def bit_count(n: int) -> int:
    """Count number of set bits."""
    return bin(n).count("1")


def solve() -> int:
    """Solve Problem 612."""
    N = 18
    M = 1000267129
    B = 10

    f = [0] * (1 << B)
    for subset in range(1, 1 << B):
        n = bit_count(subset)
        for d in range(1, N + 1):
            f[subset] = (f[subset] + pow_mod(n, d, M)) % M
            if subset % 2 == 1:  # includes digit 0
                f[subset] = (f[subset] - pow_mod(n, d - 1, M)) % M
        for strict_subset in range(1, subset):
            if (subset | strict_subset) == subset:
                f[subset] = (f[subset] - f[strict_subset]) % M
        f[subset] %= M

    ans = 0
    for subset1 in range(1, 1 << B):
        for subset2 in range(1, 1 << B):
            if (subset1 & subset2) > 0:
                ans = (ans + f[subset1] * f[subset2]) % M

    ans = (ans - (pow_mod(B, N, M) - 1)) % M
    ans = (ans * mod_inverse(2, M)) % M
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
