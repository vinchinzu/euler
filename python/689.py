"""Project Euler Problem 689: Binary Series.

Let d_i(x) be the ith digit after the binary point of the binary representation
of x. Find the probability that (Σ_{i=1}^∞ d_i(x) / i²) > N if x is uniformly
distributed from 0 to 1.

As k goes to ∞, the distribution of Σ_{i=k]^∞ d_i(x) / i² becomes similar to a
normal distribution, with mean equal to half the sum of all terms, and variance
equal to half the variance of the terms. So we can brute force the sums of
subsets of small terms, and for each subset use the erf function to compute the
approximate probability that the value of the remaining normal distribution is
higher than the required threshold.
"""

from __future__ import annotations

import math


def fsq(n: float) -> float:
    """Square of n."""
    return n * n


def erf(z: float) -> float:
    """Error function approximation."""
    if abs(z) > 5:
        return 1.0 if z > 0 else -1.0
    a1 = 0.254829592
    a2 = -0.284496736
    a3 = 1.421413741
    a4 = -1.453152027
    a5 = 1.061405429
    p = 0.3275911
    t = 1 / (1 + p * abs(z))
    sign = 1.0 if z > 0 else -1.0
    return sign * (
        1
        - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1)
        * t
        * math.exp(-fsq(z))
    )


def solve() -> float:
    """Solve Problem 689."""
    N = 0.5
    L = 22

    mean = fsq(math.pi) / 6
    stddev = math.pow(math.pi, 4) / 90
    for i in range(1, L + 1):
        mean -= 1 / fsq(i)
        stddev -= 1 / math.pow(i, 4)
    mean /= 2
    stddev = math.sqrt(stddev / 2)

    ans = 0.0
    for subset in range(1 << L):
        sum_val = 0.0
        for i in range(L):
            if (subset & (1 << i)) > 0:
                sum_val += 1 / fsq(i + 1)
        ans += (1 - erf((N - mean - sum_val) / stddev)) / 2

    ans /= 1 << L
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return int(result * 10**8)


if __name__ == "__main__":
    main()
