"""Project Euler Problem 711: Binary Blackboard.

A blackboard starts with a single integer n in binary, and Oscar and Eric take
turns writing positive integers in binary, ensuring the sum never exceeds 2n.
Find the number of starting n for which Eric can guarantee that the number of
1s is even once the sum reaches 2n.

It can be shown that 2^{2k-1}-1, 2^{2k} and 2^{2k}-1 are always valid n.
Furthermore, if n is valid and not of the form 2^{2k-1}-1, then 2^{2k}+n is
also valid. This means that we can maintain the set of valid n (not
2^{2k-1}-1) up to 2^{2k} for increasing k. When incrementing k, we add
2^{2k} and 2^{2k}-1, as well as all the valid 2^{2k}+n. At the end, we tack
on the values of the form 2^{2k-1}-1, as well as the final 2^N and 2^N-1.
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


def solve() -> int:
    """Solve Problem 711."""
    n = 12345678
    m = 10**9 + 7

    # Precompute powers of 2
    pow2s = [pow_mod(2, i, m) for i in range(n + 1)]

    ans = 0
    for i in range(2, n, 2):
        ans = (ans + (pow2s[i // 2] - 2) * pow2s[i] + ans) % m
        ans = (ans + pow2s[i] - 1) % m
        ans = (ans + pow2s[i]) % m

    for i in range(1, n, 2):
        ans = (ans + pow2s[i] - 1) % m

    ans = (ans + pow2s[n] - 1) % m
    ans = (ans + pow2s[n]) % m
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
