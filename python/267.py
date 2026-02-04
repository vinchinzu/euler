"""Project Euler Problem 267: Billionaire."""
from __future__ import annotations
import math


def solve() -> str:
    N = 1000
    C = 1e9

    # Find the minimum number of wins W0 such that for w >= W0,
    # there exists f in (0,1) with (1-f)^(N-w) * (1+2f)^w >= C.
    # Optimal f for given w: f = (3w/N - 1) / 2
    # We need f in (0,1) => w in (N/3, N)
    # Use log to avoid overflow: (N-w)*log(1-f) + w*log(1+2f) >= log(C)

    log_C = math.log(C)
    ans = 0.0

    # Use log-space nCr to avoid overflow issues
    # log(nCr(N, w)) = sum of log terms
    log_ncr = [0.0] * (N + 1)
    # log(nCr(N, 0)) = 0
    for i in range(1, N + 1):
        log_ncr[i] = log_ncr[i - 1] + math.log(N - i + 1) - math.log(i)

    log_2N = N * math.log(2)

    # Find cutoff: smallest w such that E(f_opt, w) >= C
    # Then probability = sum_{w=W0}^{N} nCr(N,w) / 2^N
    for w in range(N, -1, -1):
        f = (3.0 * w / N - 1) / 2
        if f <= 0 or f >= 1:
            continue
        log_e = (N - w) * math.log(1 - f) + w * math.log(1 + 2 * f)
        if log_e < log_C:
            break
        # This w is valid
        # Add nCr(N, w) / 2^N = exp(log_ncr[w] - N*log(2))
        ans += math.exp(log_ncr[w] - log_2N)

    return f"{ans:.12f}"


def main() -> None:
    print(solve())


if __name__ == "__main__":
    main()
