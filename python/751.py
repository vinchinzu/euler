"""Project Euler Problem 751: Concatenation Coincidence.

For a given θ, define a sequence b_1 = θ and b_n = ⌊b_{n-1}⌋ (b_{n-1} - ⌊b_{n-1}⌋ + 1)
for n ≥ 2. Then let a_n = ⌊b_n⌋. Find the unique value θ such that the number with
the decimal expansion (a_1) . (a_2) (a_3) (a_4) ... is equal to θ.

We binary search on θ. If the resulting number (a_1) . (a_2) (a_3) (a_4) is larger
than θ, then we need to search higher θ; otherwise we search lower θ.
"""

from __future__ import annotations

from decimal import Decimal, getcontext

getcontext().prec = 50


def solve() -> str:
    """Solve Problem 751."""
    N = 2
    K = 24
    low = Decimal(N)
    high = Decimal(N + 1)
    ans = ""

    while True:
        theta = (low + high) / 2
        digits = ""
        b = theta
        while len(digits) < K:
            fractional = b - int(b)
            b = int(b) * (fractional + 1)
            digits += str(int(b))

        tau = f"{int(theta)}.{digits[:K]}"
        tau_decimal = Decimal(tau)

        if tau_decimal > theta:
            low = theta
        else:
            high = theta

        if tau == ans:
            break
        ans = tau

    return ans


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
