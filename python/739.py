"""Project Euler Problem 739: Summation of Summations.

If we start with the first N terms of the Lucas sequence, and repeatedly
remove the first term and generate a new sequence of the cumulative sums of
the remaining terms, then find the value of the last remaining term.

By writing each sequence of cumulative sums below the previous sequence, we
can find that the number of times the k'th Lucas term is included in the
final term is the number of lattice paths from (0, 0) to (N-2, N-k) moving
only upwards or rightwards, and never crossing over the line y=x. The total
number of paths is nCr(2N-2-k, N-k), but we need to subtract the number of
paths that cross y=x, of which there are nCr(2N-2-k, N-k-1), because each
of those paths can be reflected across y=x+1 to hit the point (N-k-1,N-1):

Σ_{k=2}^N nCr(2N-2-k, N-k) - nCr(2N-2-k, N-k-1)

For performance, we hard code the term for the special case of k=N, and can
iteratively compute the remaining combinations by starting with 1 at k = N-1
and repeatedly multiplying by the appropriate fraction to get the next
combination.
"""

from __future__ import annotations


def mod_invs(n: int, mod: int) -> list[int]:
    """Generate modular inverses for 1..n modulo mod."""
    result = [0] * (n + 1)
    for i in range(1, n + 1):
        result[i] = pow(i, mod - 2, mod)
    return result


def solve() -> int:
    """Solve Problem 739."""
    n = 10**8
    m = 10**9 + 7

    mod_invs_list = mod_invs(n, m)

    # Lucas sequence
    lucas = [0] * (n + 1)
    lucas[1] = 1
    lucas[2] = 3
    for i in range(3, n + 1):
        lucas[i] = (lucas[i - 2] + lucas[i - 1]) % m

    ncr1 = 1
    ncr2 = 1
    ans = lucas[n]

    for k in range(n - 1, 1, -1):
        ncr1 = ncr1 * (2 * n - 2 - k) % m * mod_invs_list[n - k] % m
        ans = (ans + lucas[k] * (ncr1 - ncr2)) % m
        ncr2 = (ncr2 + ncr1) % m

    return ans % m


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
