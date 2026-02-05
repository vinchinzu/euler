"""Project Euler Problem 355 - maximal coprime subset sum.

Find the largest possible sum of a mutually coprime subset of {1, ..., 200000}.

This implementation mirrors the Java approach:
- Keep all primes > sqrt(N) plus 1 as a base set.
- For each small prime p <= sqrt(N), choose exactly one base element q and replace
  q by the largest q * p^k <= N.
- The gain matrix is solved by maximum-weight bipartite matching.

The only nontrivial component is assignment; we implement a rectangular Hungarian
algorithm directly (no third-party dependencies).
"""

from __future__ import annotations


def _sieve_primes(limit: int) -> list[int]:
    """Return all primes <= limit."""
    is_prime = bytearray(b"\x01") * (limit + 1)
    is_prime[0:2] = b"\x00\x00"

    p = 2
    while p * p <= limit:
        if is_prime[p]:
            start = p * p
            step = p
            is_prime[start : limit + 1 : step] = b"\x00" * (
                ((limit - start) // step) + 1
            )
        p += 1

    return [i for i in range(2, limit + 1) if is_prime[i]]


def _maximum_weight_assignment(weights: list[list[int]]) -> int:
    """Return max total weight selecting one distinct column per row.

    Uses Hungarian algorithm for a rectangular matrix with row_count <= col_count.
    Time complexity: O(row_count^2 * col_count).
    """
    if not weights:
        return 0

    row_count = len(weights)
    col_count = len(weights[0])
    if row_count > col_count:
        raise ValueError("assignment requires rows <= columns")

    # Hungarian for minimization; convert max(w) to min(-w).
    u = [0] * (row_count + 1)
    v = [0] * (col_count + 1)
    p = [0] * (col_count + 1)
    way = [0] * (col_count + 1)
    inf = 10**30

    for i in range(1, row_count + 1):
        p[0] = i
        j0 = 0
        minv = [inf] * (col_count + 1)
        used = [False] * (col_count + 1)

        while True:
            used[j0] = True
            i0 = p[j0]
            row = weights[i0 - 1]
            delta = inf
            j1 = 0

            for j in range(1, col_count + 1):
                if used[j]:
                    continue
                cur = -row[j - 1] - u[i0] - v[j]
                if cur < minv[j]:
                    minv[j] = cur
                    way[j] = j0
                if minv[j] < delta:
                    delta = minv[j]
                    j1 = j

            for j in range(col_count + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta

            j0 = j1
            if p[j0] == 0:
                break

        while True:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
            if j0 == 0:
                break

    # Recover selected columns and sum original weights.
    assignment = [0] * (row_count + 1)
    for j in range(1, col_count + 1):
        if p[j] != 0:
            assignment[p[j]] = j

    total = 0
    for i in range(1, row_count + 1):
        total += weights[i - 1][assignment[i] - 1]
    return total


def solve() -> int:
    n = 200000
    sqrt_n = int(n**0.5)

    primes = _sieve_primes(n)
    small_primes = [p for p in primes if p <= sqrt_n]
    base_elements = [1] + [p for p in primes if p > sqrt_n]

    base_sum = sum(base_elements)

    gains: list[list[int]] = []
    for p in small_primes:
        row: list[int] = []
        for q in base_elements:
            combined = q
            while combined * p <= n:
                combined *= p
            row.append(combined - (0 if q == 1 else q))
        gains.append(row)

    return base_sum + _maximum_weight_assignment(gains)


if __name__ == "__main__":
    print(solve())
