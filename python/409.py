"""Project Euler Problem 409: Nim Extreme.

Find the number of winning positions in a game of Nim with N non-empty
piles such that each pile has size less than 2^N and no two piles have the
same size.

The number of positions with k piles of size less than 2^N is
(2^N - 1)(2^N - 2) ... (2^N - k). Now let dp[k] be the number of losing
positions of k piles, e.g. the XOR of the k piles is zero. Then dp[k] is
roughly equal to the number of positions with k-1 piles, because given k-1
piles, the last pile needs to equal the XOR of the other piles and is
therefore fixed. However, we are over-counting the following two cases:

1. The XOR of the k-1 piles is zero, because the last pile cannot have zero
   size. There are dp[k-1] such configurations.
2. The XOR of the k-1 piles is the same size as one of the k-1 piles.
   Suppose it's the same size as the first pile; then the XOR of the other
   k-2 piles is zero, of which there are dp[k-2] configurations, and the
   first pile can be any of the remaining 2^k - (k-1) possible values. We
   multiply this subtotal by k-1 because the XOR can match any of the k-1
   piles.

The number of winning positions with N piles can be computed by subtracting
dp[N] from the total number of positions with N piles.
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 409."""
    N = 10**7
    M = 10**9 + 7

    pow2 = pow_mod(2, N, M)
    num_positions = [0] * (N + 1)
    num_positions[0] = 1
    for k in range(1, N + 1):
        num_positions[k] = (num_positions[k - 1] * (pow2 - k)) % M

    dp = [0] * (N + 1)
    dp[0] = 1
    for k in range(2, N + 1):
        dp[k] = (
            num_positions[k - 1]
            - dp[k - 1]
            - (dp[k - 2] * (k - 1) % M * (pow2 - (k - 1))) % M
        ) % M

    ans = (num_positions[N] - dp[N]) % M
    return (ans + M) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
