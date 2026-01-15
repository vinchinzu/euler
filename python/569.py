"""Project Euler Problem 569: Mountain Range Peaks.

A mountain range consists of slopes with lengths equal to the prime numbers in increasing
order, and alternatively sloping up 45º and down 45º. If P(k) is the number of peaks visible
when looking back from peak k, find Σ_{k=1}^N P(k).

Consider peak i. Let V(i) be the list of all peaks visible from i (from right to left). If
k and j are two consecutive peaks in V(i), then it must be the case that peak k and peak j
are visible from each other.

So we can generate V(i) by starting at peak j=i-1 (which is always visible), then finding
the closest peak visible from both peak j and peak i, then setting j to that closest peak
and repeating until no more peaks are visible. To efficiently find the closest peak visible
from both peak j and peak i, we note that any further peaks visible from j will still be
visible from i, so we can binary search on the peaks visible from j.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List


N = 2500000


@dataclass(frozen=True)
class LPoint:
    """Lattice point."""

    x: int
    y: int


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def get_primes(limit: int) -> List[int]:
    """Get primes up to limit."""
    return sieve_primes(limit)


def turn(p1: LPoint, p2: LPoint, p3: LPoint) -> float:
    """Compute turn direction (cross product)."""
    dx1 = p2.x - p1.x
    dy1 = p2.y - p1.y
    dx2 = p3.x - p2.x
    dy2 = p3.y - p2.y
    return dx1 * dy2 - dy1 * dx2


def solve() -> int:
    """Solve Problem 569."""
    primes_list = get_primes(2 * N)
    peaks: List[LPoint] = []
    all_visible_peaks: List[List[int]] = []
    x = 0
    y = 0
    ans = 0

    for i in range(N):
        x += primes_list[2 * i]
        y += primes_list[2 * i]
        peaks.append(LPoint(x, y))
        x += primes_list[2 * i + 1]
        y -= primes_list[2 * i + 1]

        visible_peaks: List[int] = []
        j = i - 1
        while j >= 0:
            visible_peaks.append(j)
            if j == 0:
                break
            prev_visible_peaks = all_visible_peaks[j]
            mid = peaks[j]
            base = peaks[i]

            # Binary search for next visible peak
            left = 0
            right = len(prev_visible_peaks)
            while left < right:
                mid_idx = (left + right) // 2
                peak_idx = prev_visible_peaks[mid_idx]
                turn_val = turn(peaks[peak_idx], mid, base)
                if turn_val < 0:
                    left = mid_idx + 1
                else:
                    right = mid_idx

            if left >= len(prev_visible_peaks):
                break
            j = prev_visible_peaks[left]

        all_visible_peaks.append(visible_peaks)
        ans += len(visible_peaks)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
