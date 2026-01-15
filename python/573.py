"""Project Euler Problem 573: Unfair Race.

N runners have speeds 1/N, 2/N, ... N/N. N random positions are generated on
a racetrack, and the one closest to the finish line is assigned to the first
runner, the second closest assigned to the second runner, etc. Let P_{N,k} be
the probability that the kth runner wins the race. Find E_N = Σ_{k=1}^N k*P_{N,k}.

After analysis, we find that:
E_N = Σ_{k=1}^N k N! ∫_0^{k/N} (1-p_k)^{N-k-1} (k-N*p_k) (p_k)^{k-1} /
      (k(N-k)!k!) dp_k
    = nCr(N,k) k^k (N-k)^{N-k} / N^N.
"""

from __future__ import annotations

import math


def log_factorials(n: int) -> list[float]:
    """Precompute log factorials up to n."""
    result = [0.0] * (n + 1)
    for i in range(1, n + 1):
        result[i] = result[i - 1] + math.log(i)
    return result


def solve() -> float:
    """Solve Problem 573."""
    N = 1000000

    log_facts = log_factorials(N)
    ans = 1.0  # k = N case: nCr(N,N) * N^N * 0^0 / N^N = 1

    for k in range(1, N):
        log_term = (
            log_facts[N]
            - log_facts[k]
            - log_facts[N - k]
            + k * math.log(k)
            + (N - k) * math.log(N - k)
            - N * math.log(N)
        )
        ans += math.exp(log_term)

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return result


if __name__ == "__main__":
    main()
