"""Project Euler Problem 604: Convex path in square.

Find the maximum number of lattice points in an axis-aligned NxN square that an
increasing strictly convex function can pass through.

The optimal order to add segments is to add pairs of segments with the smallest
possible sum of x-distance and y-distance: (1, 1), (1, 2), (2, 1), (1, 3),
(3, 1), (1, 4), (2, 3), (3, 2), (4, 1)... there are φ(k) segments with sum k,
which will increase the width of the square by k*φ(k)/2.

After adding these "complete" levels up to but excluding segments with sum k,
we add pairs of segments that sum to k. Finally, if there is a segment with
both x-distance and y-distance that remains in the boundary of the square, we
can add that single segment.
"""

from __future__ import annotations

from math import ceil, gcd, isqrt


def sieve_phi(limit: int) -> list[int]:
    """Euler totient function sieve."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def solve() -> int:
    """Solve Problem 604."""
    N = 10**18
    limit = int(3 * (N ** (1 / 3)))
    phi = sieve_phi(limit)

    ans = 1
    width = 0
    k = 2
    while width + k * phi[k] // 2 <= N:
        ans += phi[k]
        width += k * phi[k] // 2
        k += 1

    num_additions = (N - width) // k
    ans += 2 * num_additions
    width += k * num_additions

    # Find last segment
    found = False
    while not found and width + ceil(k / 2) <= N:
        for big in range(ceil(k / 2), N - width + 1):
            if gcd(k, big) == 1:
                ans += 1
                found = True
                break
        k += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
