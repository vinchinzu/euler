"""Project Euler Problem 359: Hilbert's New Hotel

An infinite number of people (numbered 1, 2, 3, etc.) are lined up to get a room
at Hilbert's newest infinite hotel. The hotel contains an infinite number of floors
(numbered 1, 2, 3, etc.), and each floor contains an infinite number of rooms
(numbered 1, 2, 3, etc.).

Initially the hotel is empty. Person n gets the first vacant room in the lowest
numbered floor satisfying either of the following:
- the floor is empty
- the floor is not empty, and if the latest person taking a room in that floor
  is person m, then m + n is a perfect square

Define P(f, r) to be n if person n occupies room r in floor f, and 0 if no person
occupies the room.

Find the sum of all P(f, r) for all positive f and r such that f * r = 71328803586048
and give the last 8 digits as your answer.

Solution approach:
1. Simulate the hotel assignment to find patterns
2. Derive closed-form formulas for P(f, r) based on floor and room parity
3. Apply formulas to all divisor pairs (f, r) of N and sum mod 10^8

The key insight is that P(f, r) follows different quadratic formulas depending on
whether f and r are odd or even:
- Floor 1: P(1, r) = r(r+1)/2 (triangular numbers)
- Even floor, odd room:  P(f, r) = ((f+r)^2 - 2f - r) / 2
- Even floor, even room: P(f, r) = ((f+r)^2 - r) / 2
- Odd floor (f>=3), odd room:  P(f, r) = ((f+r)^2 - 2f - 3r + 1) / 2
- Odd floor (f>=3), even room: P(f, r) = ((f+r)^2 - 4f - 3r + 3) / 2
"""

from __future__ import annotations


def P(f: int, r: int) -> int:
    """Compute P(f, r) - the person number in room r of floor f.

    Based on the parity of f and r, uses the appropriate closed-form formula.
    """
    if f == 1:
        # Floor 1: triangular numbers
        return r * (r + 1) // 2

    if f % 2 == 0:  # even floor
        if r % 2 == 1:  # odd room
            return ((f + r) ** 2 - 2 * f - r) // 2
        else:  # even room
            return ((f + r) ** 2 - r) // 2
    else:  # odd floor (f >= 3)
        if r % 2 == 1:  # odd room
            return ((f + r) ** 2 - 2 * f - 3 * r + 1) // 2
        else:  # even room
            return ((f + r) ** 2 - 4 * f - 3 * r + 3) // 2


def get_divisors(n: int) -> list[int]:
    """Get all divisors of n."""
    divisors = []
    d = 1
    while d * d <= n:
        if n % d == 0:
            divisors.append(d)
            if d * d != n:
                divisors.append(n // d)
        d += 1
    return divisors


def solve(n: int = 71328803586048, mod: int = 10**8) -> int:
    """Solve PE 359: sum of P(f, r) for all (f, r) where f * r = n, mod 10^8."""
    divisors = get_divisors(n)

    total = 0
    for f in divisors:
        r = n // f
        total += P(f, r)

    return total % mod


if __name__ == "__main__":
    print(solve())
