"""Project Euler Problem 743: Window into a Matrix.

Find the number of 2xN matrices of 0s and 1s such that the sum of the
entries in every 2xK sub-matrix is K.

Let i be the number of columns of the first K columns where both entries are
zero. Then i of those columns most have both entries equal to one as well.
The number of 2xN matrices for a given i is nCr(K,i) ways to choose the zero
columns, times nCr(K-i,i) ways to choose the one columns, times
2^{(N/K)(K-2i)} ways to choose for each remaining column whether the 1 is on
the top or the bottom. So the answer is

Σ_{i=0}^⌊K/2⌋ nCr(K,i) nCr(K-i,i) * 2^{(N/K)(K-2i)}.

For performance, we note that the term for i=0 is 2^N, and we can
iteratively compute each term from the previous one.
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


def mod_invs(n: int, mod: int) -> list[int]:
    """Generate modular inverses for 1..n modulo mod."""
    result = [0] * (n + 1)
    for i in range(1, n + 1):
        result[i] = pow(i, mod - 2, mod)
    return result


def sq(n: int, mod: int) -> int:
    """Square of n modulo mod."""
    return (n * n) % mod


def solve() -> int:
    """Solve Problem 743."""
    n = 10**16
    k = 10**8
    m = 10**9 + 7

    mod_invs_list = mod_invs(k // 2 + 1, m)
    base = pow_mod(pow_mod(2, 2 * n // k, m), m - 2, m)  # mod inverse
    res = pow_mod(2, n, m)
    ans = 0

    for i in range(0, k // 2 + 1):
        ans = (ans + res) % m
        if 2 * i < k:
            res = (
                res
                * sq(mod_invs_list[i + 1], m)
                % m
                * (k - 2 * i)
                % m
                * (k - 2 * i - 1)
                % m
                * base
                % m
            )

    return ans % m


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
