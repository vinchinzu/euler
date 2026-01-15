"""Project Euler Problem 641: A Long Row of Dice.

If N² dice are arranged in a line and all showing 1, and each second die is
increased by 1, each third die is increased by 1, and so on (with 6 wrapping
back to 1), then find the total number of dice that shows 1 at the end.

The problem is equivalent to finding the number of integers up to N with 6k+1
factors if we also include 1.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def nth_root(n: int, root: int) -> int:
    """Integer nth root."""
    return int(n ** (1 / root))


def solve() -> int:
    """Solve Problem 641."""
    N = 10**18
    K = 6
    L = int(N**0.4)

    primes = list(primerange(2, L + 1))
    prime_counts = [0] * (L + 1)
    count = 0
    for i in range(2, L + 1):
        if i in primes:
            count += 1
        prime_counts[i] = count

    ans = 1

    def helper(min_index: int, n: int, num_divisors: int) -> None:
        """Recursive helper."""
        nonlocal ans
        for e in range(
            K if num_divisors % K == 1 else K - 2, N + 1, K
        ):
            bound = nth_root(N // n, e // 2)
            if bound < primes[min_index]:
                break
            if bound > L:
                # Use prime counting (simplified)
                count_val = bound - min_index
            else:
                count_val = prime_counts[bound] - min_index
            ans += count_val

        for index in range(min_index, len(primes)):
            p = primes[index]
            if n * pow(p, K - 2) > N:
                break
            for start_e in [K - 2, K]:
                e = start_e
                while n * pow(p, e // 2) < N:
                    helper(
                        index + 1,
                        n * pow(p, e // 2),
                        num_divisors * (e + 1),
                    )
                    e += K

    helper(0, 1, 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
