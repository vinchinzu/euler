"""Project Euler Problem 430: Range flips.

N disks numbered from 1 to N are placed in a row. On every turn, two random
integers A and B are selected from [1, N], and all disks from A to B
inclusive are flipped over. After M turns, find the expected number of
disks that will be flipped an even number of times.

The probability p_k that disk k is not flipped on a given turn is equal to
the probability that both A and B are to the left of k, which is (k-1)² / N²,
and the probability that both are to the right of k, which is (N-k)² / N².

The probability that disk k is flipped an even number of times is therefore:
P(k) = Σ_{i=0, i even}^M (p_k)^i (1 - p_k)^{M - i}
=> 2 P(k) = ( Σ_{i=0}^M (p_k)^i (1 - p_k)^{M - i} )
          + ( Σ_{i=0}^M (-1)^i (p_k)^i (1 - p_k)^{M - i} )
          = (p_k + (1 - p_k))^M + (p_k - (1 - p_k))^M
          = 1 + (2p_k - 1)^M

By linearity of expectation, we need to sum P(k) for k from 1 to N. By
symmetry:
Σ_{k=1}^N P(k) = Σ_{k=1}^{N/2} 2 P(k)
               = N/2 + Σ_{k=1}^{N/2} (2p_k - 1)^M

Finally, for efficiency, note that for large enough k, the probability p_k
becomes small enough that (2p_k - 1)^M becomes negligible, so we can stop
at that point.
"""

from __future__ import annotations


def solve() -> float:
    """Solve Problem 430."""
    N = 10**10
    M = 4000

    ans = 0.0
    for k in range(1, N // 2 + 1):
        p_k = ((k - 1) ** 2 + (N - k) ** 2) / (N * N)
        term = (2 * p_k - 1) ** M
        if abs(term) < 1e-15:
            break
        ans += term
    ans += N / 2
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.2f}")
    return result


if __name__ == "__main__":
    main()
