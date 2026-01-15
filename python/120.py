"""Project Euler Problem 120.

Let r be the remainder when (a - 1)^n + (a + 1)^n is divided by a^2.
For example, if a = 7 and n = 3, then r = 42: 6^3 + 8^3 = 728 ≡ 42 mod 49.
And as n varies, so too will r, but for a = 7 it turns out that r_max = 42.
For 3 ≤ a ≤ 1000, find ∑ r_max.
"""

from math import gcd


def max_r_for_a(a: int) -> int:
    """Compute the maximum remainder r_max for a given a using closed-form formula.
    
    For odd n, r ≡ 2 * n * a (mod a^2) = a * (2n mod a)
    The maximum occurs when (2n mod a) is maximized for odd n.
    The possible values of (2n mod a) for odd n form an arithmetic sequence
    starting at 2 with step 4 modulo a, covering all residues ≡ 2 (mod d)
    where d = gcd(4, a). The maximum such residue m < a leads to max r = a * m.
    m = 2 + d * floor((a - 3)/d)
    """
    if a < 3:
        raise ValueError("Invalid a: must be >= 3")

    d = gcd(4, a)
    # Largest m < a such that m ≡ 2 (mod d)
    m = 2 + d * ((a - 3) // d)
    # Since m < a, a * m < a^2, so no modulo needed
    return a * m


def main() -> int:
    """Main computation."""
    sum_r_max = 0
    for a in range(3, 1001):
        sum_r_max += max_r_for_a(a)
    return sum_r_max


if __name__ == "__main__":
    print(main())
