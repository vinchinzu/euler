"""Project Euler Problem 192: Best Approximations."""

from typing import Dict, List
import math

LIMIT_N = 100_000
DENOMINATOR_BOUND = 1_000_000_000_000


def int_sqrt(n: int) -> int:
    """Integer square root via binary search."""
    if n <= 0:
        return 0
    x = int(math.sqrt(n))
    while x * x > n:
        x -= 1
    while (x + 1) * (x + 1) <= n:
        x += 1
    return x


def perfect_square(n: int) -> bool:
    """Check if n is a perfect square."""
    r = int_sqrt(n)
    return r * r == n


def continued_fraction_sqrt(n: int) -> Dict[str, int | List[int]]:
    """Return the continued fraction data for √n."""
    a0 = int_sqrt(n)
    if a0 * a0 == n:
        return {"a0": a0, "period": []}

    m = 0
    d = 1
    a = a0
    period: List[int] = []
    seen: Dict[tuple[int, int], bool] = {}

    while True:
        key = (m, d)
        if key in seen:
            break
        seen[key] = True

        m = d * a - m
        d = (n - m * m) // d
        a = (a0 + m) // d
        period.append(a)

    return {"a0": a0, "period": period}


def best_denominator(n: int, cf: Dict[str, int | List[int]], limit: int) -> int:
    """Compute the best denominator ≤ limit for √n."""
    a0 = cf["a0"]
    period = cf["period"]
    if isinstance(period, int):
        period = []
    period_cycle = period if period else [0]

    p_prev = 1
    q_prev = 0
    p_curr = a0
    q_curr = 1

    if q_curr > limit:
        return q_curr

    best = q_curr
    index = 0

    while True:
        a = period_cycle[index % len(period_cycle)]
        p_full = a * p_curr + p_prev
        q_full = a * q_curr + q_prev

        if q_full <= limit:
            p_prev = p_curr
            q_prev = q_curr
            p_curr = p_full
            q_curr = q_full
            best = q_curr
        else:
            # Semi-convergent: find max k=1 to a-1
            max_k = min(a - 1, (limit - q_prev) // q_curr)
            if max_k >= 1:
                best_q = max_k * q_curr + q_prev
                best = max(best, best_q)
            # After handling semi-convergent, break as q_full > limit
            break

        index += 1

    return best


def main() -> int:
    """Main function."""
    total = 0

    for n in range(2, LIMIT_N + 1):
        if perfect_square(n):
            continue
        cf = continued_fraction_sqrt(n)
        total += best_denominator(n, cf, DENOMINATOR_BOUND)

    return total


if __name__ == "__main__":
    print(main())
