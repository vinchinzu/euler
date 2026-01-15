"""Project Euler Problem 407: Idempotents.

Let M(n) be the largest a < n such that a² ≡ a (mod n). Find
sum_{n=1}^N M(n).

If a² ≡ a (mod n), then there must exist some relatively prime r,s such
that a ≡ 1 (mod r), a ≡ 0 (mod s), and r*s = n. By the Chinese Remainder
Theorem, this gives a ≡ (s⁻¹ (mod r)) * s (mod n). We can therefore
generate all relatively prime (r,s) and determine if that yields a larger
value of M(n).

We generate all relatively prime (r,s) using the double ternary-tree
method. This yields all relatively prime r ≥ s, so we need to test both
a = (s⁻¹ (mod r)) * s and a = (r⁻¹ (mod s)) * r.

Finally, for efficiency we compute these modular inverses based on the
modular inverses of the parent node in the tree. For example, given modular
inverses r⁻¹ and s⁻¹, we have r*(r⁻¹) + s(s⁻¹) = 1 => (2r-s)(-s⁻¹) +
r(r⁻¹+2s⁻¹) = 1, which gives us the modular inverses for (2r-s, r). The
logic for the other branches of the ternary tree is similar.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass


@dataclass
class RelativelyPrimePair:
    """Represents a relatively prime pair (r, s) with r inverse."""

    r: int
    s: int
    r_inv: int


def mod_inverse(a: int, m: int) -> int:
    """Compute modular inverse of a mod m."""
    if m == 1:
        return 0
    g, x, _ = extended_gcd(a, m)
    if g != 1:
        return 0
    return (x % m + m) % m


def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = extended_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def solve() -> int:
    """Solve Problem 407."""
    N = 10**7

    M = [1] * (N + 1)
    for n in range(2, N + 1):
        M[n] = 1

    pairs = deque(
        [
            RelativelyPrimePair(2, 1, 0),
            RelativelyPrimePair(3, 1, 0),
        ]
    )

    while pairs:
        pair = pairs.popleft()
        r, s, r_inv = pair.r, pair.s, pair.r_inv
        if r * s <= N:
            s_inv = (1 - r * r_inv) // s
            a1 = r * (mod_inverse(r_inv, s) % s)
            a2 = s * (mod_inverse(s_inv, r) % r)
            a = max(a1, a2)
            if a > M[r * s]:
                M[r * s] = a
            pairs.append(RelativelyPrimePair(2 * r - s, r, -s_inv))
            pairs.append(RelativelyPrimePair(2 * r + s, r, s_inv))
            pairs.append(RelativelyPrimePair(r + 2 * s, s, r_inv))

    return sum(M[1:])


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
