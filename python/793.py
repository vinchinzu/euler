"""Project Euler Problem 793: Median of Products.

Find the median of the products S_i * S_j for 0 <= i < j < N.

We use binary search. For any X, we can iterate over each S_i, and for each
one find the number of S_j such that S_i * S_j < X. We then remove (S_i)^2 if
necessary. Since we are counting both S_i S_j and S_j S_i, we run binary
search on there are more than tr(N-1) products less than X, instead of
tr(N-1)/2.
"""

from __future__ import annotations


def blum_blum_shub(seed: int, n: int) -> list[int]:
    """Generate Blum Blum Shub sequence starting with S_0 = seed."""
    result = [seed]  # S_0 = seed
    x = seed
    for _ in range(n - 1):
        x = (x * x) % 50515093
        result.append(x)
    return result


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 793."""
    N = 1000003
    S = blum_blum_shub(290797, N)
    S.sort()

    low = 0
    high = sq(S[-1])

    while low + 1 < high:
        mid = (low + high) // 2
        rank = 0
        row_count = N - 1

        for s in S:
            while row_count >= 0 and s * S[row_count] >= mid:
                row_count -= 1
            # rowCount + 1 is the count of j where s * S[j] < mid
            # Subtract 1 if sq(s) < mid (to exclude the diagonal i=j case)
            rank += row_count + (0 if sq(s) < mid else 1)

        if rank > tr(N - 1):
            high = mid
        else:
            low = mid

    return low


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
