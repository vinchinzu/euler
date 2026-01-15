"""Project Euler Problem 815: Grouping Cards.

If one card is dealt at a time from a pack of cards with K copies of each
of N values, and the K copies of each value are removed as soon as they
are all dealt, find the expected value of the maximum number of distinct
card values present at a time during this process.

This can be solved using dynamic programming. At any point, we can
consider the counts of values of which there are 0 present, 1 present,
2 present, etc. up to K-1 present (the number of which there are K
present is fixed because they must add to N), as well as the current
maximum number of distinct card values so far. Then we consider the K
possible states of the next card: e.g. if the next card is a value of
which there are currently t present, then we decrement the number of
values where there are t present and increment the number where there
are t+1 present, and update the max as necessary.

By balls and bins, there are nCr(N+K, K) possible values of these
counts. For performance, instead of using a map to store the cached
values, we use an array where the index is determined by the
combinatorial number system.
"""

from __future__ import annotations

from typing import Dict, List, Tuple


def nCr(n: int, r: int) -> int:
    """Compute binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def compute_nCr_table(max_n: int, max_r: int) -> List[List[int]]:
    """Precompute binomial coefficients."""
    table: List[List[int]] = []
    for n in range(max_n + 1):
        row: List[int] = []
        for r in range(max_r + 1):
            row.append(nCr(n, r))
        table.append(row)
    return table


def index_from_counts(counts: List[int], nCr_table: List[List[int]]) -> int:
    """Convert counts to index using combinatorial number system."""
    total = -1
    idx = 0
    K = len(counts) - 1
    for i in range(K):
        total += counts[i] + 1
        if total >= i + 1:
            idx += nCr_table[total][i + 1]
    return idx


def solve() -> float:
    """Solve Problem 815."""
    N = 60
    K = 4

    nCr_table = compute_nCr_table(N + K, K)
    max_index = nCr_table[N + K][K]
    cache: Dict[Tuple[int, int], float] = {}

    def e(counts: List[int], max_val: int) -> float:
        """Expected maximum given counts and current max."""
        idx = index_from_counts(counts, nCr_table)
        key = (idx, max_val)
        if key in cache:
            return cache[key]

        remaining = 0
        for i in range(K):
            remaining += (K - i) * counts[i]
        if remaining == 0:
            cache[key] = float(max_val)
            return cache[key]

        result = 0.0
        for t in range(K):
            if counts[t] > 0:
                count = counts[t]
                counts[t] -= 1
                counts[t + 1] += 1
                new_max = max(max_val, N - counts[0] - counts[K])
                result += e(counts, new_max) * (K - t) * count
                counts[t] += 1
                counts[t + 1] -= 1

        cache[key] = result / remaining
        return cache[key]

    counts = [N] + [0] * K
    return e(counts, 0)


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()
