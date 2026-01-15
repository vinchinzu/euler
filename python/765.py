"""Project Euler Problem 765: Gambling Probability.

In a game where you start with 1 gold and in each round you can bet any gold
and double the bet with probability P and lose the bet otherwise, find the
probability you can get K gold after N rounds playing optimally.

In the optimal strategy, divide the region from [0, K] into 2^N identical
segments. In the first segment, the probability of success is 0. The consecutive
differences of probabilities consists of nCr(N,0) occurrences of P^N, then
nCr(N,1) occurrences of P^{N-1} (1-P), and so on with nCr(N,k) occurrences
of P^{N-k} (1-P)^k. We want the probability in segment 2^N / K. So we repeatedly
subtract nCr(N,k) until we can no longer do so; then for the final nCr(N,k) we
add the correct number of P^{N-k} (1-P)^k. Since these are large numbers,
this is all done in log space.
"""

from __future__ import annotations

import math
from typing import List


def log_factorials(n: int) -> List[float]:
    """Precompute log factorials."""
    log_fact = [0.0] * (n + 1)
    for i in range(1, n + 1):
        log_fact[i] = log_fact[i - 1] + math.log(i)
    return log_fact


def log_sum(log_x: float, log_y: float) -> float:
    """Compute log(x+y) from log(x) and log(y)."""
    if log_x == float("-inf"):
        return log_y
    if log_y == float("-inf"):
        return log_x
    if log_x > log_y:
        return log_x + math.log(1 + math.exp(log_y - log_x))
    else:
        return log_y + math.log(1 + math.exp(log_x - log_y))


def log_diff(log_x: float, log_y: float) -> float:
    """Compute log(x-y) from log(x) and log(y), assuming x > y."""
    return log_x + math.log(1 - math.exp(log_y - log_x))


def solve() -> float:
    """Solve Problem 765."""
    N = 1000
    K = 10**12
    P = 0.6

    log_factorials_arr = log_factorials(N)
    max_count = N * math.log(2) - math.log(K)
    ans = float("-inf")

    k = 0
    while True:
        count = (
            log_factorials_arr[N]
            - log_factorials_arr[k]
            - log_factorials_arr[N - k]
        )
        ans = log_sum(
            ans,
            min(count, max_count)
            + (N - k) * math.log(P)
            + k * math.log(1 - P),
        )
        if count > max_count:
            ans = math.exp(ans)
            break
        max_count = log_diff(max_count, count)
        k += 1

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
