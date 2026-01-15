"""Project Euler Problem 175: Fractions involving the number of different ways a number can be expressed as a sum of powers of 2."""

from typing import List
import math

P_TARGET = 123_456_789
Q_TARGET = 987_654_321

POWERS = [1 << k for k in range(61)]


def continued_fraction(num: int, den: int) -> List[int]:
    """Simple continued fraction of num/den."""
    cf: List[int] = []
    while den != 0:
        a = num // den
        cf.append(a)
        num, den = den, num - a * den
    return cf


def ensure_parity(cf: List[int], desired_parity: int) -> List[int]:
    """Adjust the continued fraction so its length parity matches desired."""
    cf = cf.copy()
    while len(cf) % 2 != desired_parity:
        if cf[-1] > 1:
            cf[-1] -= 1
            cf.append(1)
        elif len(cf) == 1:
            cf[0] += 1
        else:
            cf[-2] += 1
            cf.pop()
    return cf


def runs_from_fraction(p: int, q: int) -> List[int]:
    """Convert fraction to run lengths for the shortened binary expansion."""
    if p <= 0 or q <= 0:
        raise ValueError("p and q must be positive")

    g = math.gcd(p, q)
    p //= g
    q //= g
    if p == q:
        return [1]

    if p < q:
        cf = continued_fraction(q, p)
        cf = ensure_parity(cf, 1)  # expect odd number of runs when ratio < 1
    else:
        cf = continued_fraction(p, q)
        cf = ensure_parity(cf, 0)  # expect even number of runs when ratio >= 1

    cf.reverse()
    return cf


def main() -> str:
    """Main function."""
    runs = runs_from_fraction(P_TARGET, Q_TARGET)
    return ",".join(str(r) for r in runs)


if __name__ == "__main__":
    print(main())
