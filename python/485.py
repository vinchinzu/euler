"""Project Euler Problem 485: Maximum number of divisors.

Let M(n, k) be the maximum number of divisors of a number from n to n+k-1
inclusive. Find Σ_{n=1}^{N-K+1} M(n, K).
"""

from __future__ import annotations

from typing import List, Tuple


def num_divisors(n: int) -> int:
    """Count divisors of n."""
    count = 0
    i = 1
    while i * i <= n:
        if n % i == 0:
            count += 2 if i * i != n else 1
        i += 1
    return count


def solve() -> int:
    """Solve Problem 485."""
    N = 10**8
    K = 10**5

    # Find maximum divisors in range [1, K]
    max_divisors = 0
    for n in range(1, K + 1):
        max_divisors = max(max_divisors, num_divisors(n))

    # Generate numbers with at least max_divisors divisors
    values: List[Tuple[int, int]] = []
    for n in range(1, N + 1):
        divs = num_divisors(n)
        if divs >= max_divisors:
            values.append((n, divs))

    values.sort(key=lambda x: x[1], reverse=True)

    # Compute sum using sliding window
    ans = 0
    used = [False] * (N - K + 2)

    for n, divs in values:
        # Find interval around n that hasn't been used
        start = max(1, n - K + 1)
        end = min(N - K + 1, n)

        for pos in range(start, end + 1):
            if not used[pos]:
                used[pos] = True
                ans += divs

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
