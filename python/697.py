"""Project Euler Problem 697: Randomly Decaying Sequence.

Find log(c) such that c multiplied by N uniform random variables between 0 and 1
has a 25% chance of being less than 1.

The distribution of the product of N uniform random variables between 0 and 1 is
d(z) = (-ln z)^(n-1) / (n-1)!, so we need to solve the equation

(1 / (n-1)!) ∫_0^{1/c} (-ln z)^{n-1} dz = 25%.

The indefinite integral is Γ(n, -ln z), which tends to 0 as z → 0. Using the
closed formula for Γ:

(1 / (n-1)!) [ Γ(n, -ln z) ]_0^{1/c} = 25%
=> (1 / (n-1)!) [ (n-1)! e^{ln z} Σ_{k=0}^{n-1} ( (-ln z)^k / k! ) ]_0^{1/c} = 25%
=> e^{-ln c} Σ_{k=0}^{n-1} ( (ln c)^k / k! ) = 25%
=> ( ln Σ_{k=0}^{n-1} ( (ln c)^k / k! ) ) - (ln c) = ln 25%.

We can then use binary search to compute (ln c), and from that (log c).
"""

from __future__ import annotations

import math


def log_factorials(n: int) -> list[float]:
    """Precompute log factorials."""
    result = [0.0] * (n + 1)
    for i in range(1, n + 1):
        result[i] = result[i - 1] + math.log(i)
    return result


def log_sum(log_a: float, log_b: float) -> float:
    """Compute log(a + b) given log(a) and log(b)."""
    if log_a == -float('inf'):
        return log_b
    if log_b == -float('inf'):
        return log_a
    if log_a > log_b:
        return log_a + math.log(1 + math.exp(log_b - log_a))
    return log_b + math.log(1 + math.exp(log_a - log_b))


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> float:
    """Solve Problem 697."""
    N = 10**7
    R = 0.25

    log_facts = log_factorials(N)
    low = 0.0
    high = 2.0 * N

    while low + 1e-3 < high:
        log_c = (low + high) / 2
        log_prob = -float('inf')
        for k in range(N - 1, -1, -1):
            new_log_prob = log_sum(
                log_prob, k * math.log(log_c) - log_facts[k]
            )
            if feq(log_prob, new_log_prob):
                break
            log_prob = new_log_prob

        if log_prob > math.log(R) + log_c:
            low = log_c
        else:
            high = log_c

    ans = low * math.log10(math.e)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.2f}")
    return int(result * 100)


if __name__ == "__main__":
    main()
