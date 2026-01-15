"""Project Euler Problem 472: Lightbulb placement.

People sit in a row of N seats, the first person choosing any seat and each
subsequent person choosing a seat furthest from anyone else already seated.
Find Σ_{k=1}^N f(k) where f(k) is the number of chairs where the first person
can sit such that the total number of seated people is maximized.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 472."""
    N = 10**12
    M = 10**8

    # Precompute f(k) for small k
    f_small = [0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]
    ans = sum(f_small[: min(N + 1, len(f_small))])

    if N < len(f_small):
        return ans % M

    # Pattern for larger N
    t = 4
    k = 16
    while k <= N:
        # Add 8 seats for 2^t + 2
        ans = (ans + 8) % M
        k += 1

        # Add ranges
        range_size = 1
        while range_size < (1 << (t - 2)) and k <= N:
            ans = (ans + 2 * range_size) % M  # Two ranges
            range_size *= 2
            k += range_size

        # Center range
        if k <= N:
            center_size = 1 << (t - 2)
            while center_size > 0 and k <= N:
                ans = (ans + center_size + 2) % M  # Range + 2 ends
                center_size //= 2
                k += center_size

        t += 1
        k = (1 << t) + 1

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
