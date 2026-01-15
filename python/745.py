"""Project Euler Problem 745: Sum of Squares.

Find Σ_{n=1}^N g(n), where g(n) is the largest perfect square dividing n.

The number of times that k² appears in the sum is equal to the number of
square-free integers up to ⌊N/k²⌋, f(⌊N/k²⌋). We can compute f(i) for all
small i, and compute f(i) for large i using the identity
n = f(i) + f(i/4) + f(i/9) + ... repeatedly. By computing sums of entire
ranges with the same f(i), we can compute all f(i) in O(N^{3/7}) time.
"""

from __future__ import annotations

from math import cbrt, isqrt, pow as math_pow


def sieve_square_free(limit: int) -> list[bool]:
    """Sieve to mark square-free numbers."""
    is_square_free = [True] * (limit + 1)
    for i in range(2, isqrt(limit) + 1):
        i_sq = i * i
        for j in range(i_sq, limit + 1, i_sq):
            is_square_free[j] = False
    return is_square_free


def solve() -> int:
    """Solve Problem 745."""
    n = 10**14
    m = 10**9 + 7

    l = int(math_pow(n, 2.0 / 7))
    l2 = int(n // (l * l))

    is_square_free = sieve_square_free(l2)

    small = [0] * (l2 + 1)
    for i in range(1, l2 + 1):
        small[i] = small[i - 1] + (1 if is_square_free[i] else 0)

    big = [0] * (l + 1)
    for i in range(l, 0, -1):
        n_val = n // (i * i)
        big[i] = n_val
        lim = int(cbrt(n_val))
        for k in range(2, isqrt(n_val // lim) + 1):
            k_sq = k * k
            if i * k < l:
                big[i] -= big[i * k]
            else:
                big[i] -= small[n_val // k_sq]
        for t in range(1, lim):
            big[i] -= (isqrt(n_val // t) - isqrt(n_val // (t + 1))) * small[t]

    ans = 0
    k = 1
    while k * k <= n:
        k_sq = k * k
        if k < l:
            ans = (ans + k_sq * big[k]) % m
        else:
            ans = (ans + k_sq * small[n // k_sq]) % m
        k += 1

    return ans % m


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
