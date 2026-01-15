"""Project Euler Problem 238: Infinite string tour.

Let w be the concatenation of numbers defined by S_0 and S_k = (S_{k-1})² (mod M).
Find sum_{k=1}^N p(k), where p(k) is the smallest index such that a substring
of w starting from that index has digit sum k.
"""

from __future__ import annotations

from typing import List


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def imod(n: int, mod: int) -> int:
    """Return n mod mod (always positive)."""
    return ((n % mod) + mod) % mod


def solve() -> int:
    """Solve Problem 238."""
    N = 2 * (10**15)
    S0 = 14025256
    M = 20300713
    L = 20000000
    B = 10

    # Generate sequence until it cycles
    w: List[int] = [0] * L
    index = 0
    s = S0

    while True:
        s_str = str(s)
        index += len(s_str)
        i = index
        temp = s
        while temp > 0:
            i -= 1
            w[i] = temp % B
            temp //= B
        s = sq(s) % M
        if s == S0:
            break

    # Compute digit sum D
    D = sum(w[:index])

    # Compute p(k) for k <= D
    p = [0] * (D + 1)
    pos = 0
    start_sum = 0
    num_found = 0

    while num_found + index // 2 < D:
        k = 0
        for i in range(pos, index):
            if p[k] == 0:
                p[k] = pos + 1
                num_found += 1
            k += w[i]
        w[index] = w[pos]
        index += 1
        start_sum += w[pos]
        pos += 1

    # Fill remaining p(k)
    for k in range(D):
        if p[k] == 0:
            i = pos
            new_k = k + start_sum
            while p[new_k % D] != 1:
                new_k += w[i - 1]
                i += 1
            p[k] = i + 1

    # Compute final answer
    sum1 = sum(p[k] for k in range(N % D + 1))
    sum2 = sum(p[k] for k in range(imod(N, D) + 1, D))
    ans = (sum1 + sum2) * (N // D) + sum1 - 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
