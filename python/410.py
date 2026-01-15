"""Project Euler Problem 410: Circle and tangent line.

Find F(R, X) be the number of integer quadruplets (r, a, b, c) such that
0 < r ≤ R, 0 < a ≤ X, and the line passing through (a, b) and (-a, c) is
tangent to the circle x² + y² = r². Find F(A, B) + F(B, A).

The line through (a, b) and (-a, c) has slope (c-b)/(2a), so the tangent
point is on the line y = (2a)/(b-c) x. This means the tangent point is on
these two lines and the circle, so satisfies the equations:
y-b = (c-b)/(2a) (x+a)
y = (2a)/(b-c) x
x² + y² = r²

Solving these equations and simplifying gives a² (b+c)² = r² ((2a)² + (b-c)²).
Note that this equation is symmetric in a and r, so F(A, B) = F(B, A), and
we need only compute F(B, A).

First suppose a is positive. There are some solutions where b = c;
geometrically, we can see that these correspond to horizontal lines, and
satisfy the conditions as long as b = c = r and a is any positive integer.
There are A*B such solutions.

We need (2a)² + (b-c)² to be a perfect square k², because all other terms
in the equation are perfect squares. This means (2a) and (b-c) are legs of
a Pythagorean triplet. Let the primitive triplet be (2j, k, l), where j|a.

There are 2^{ω(n)} primitive Pythagorean triplets with leg n, where ω(n)
is the number of distinct prime factors of n. If j is odd, then there are
2^{ω(j)} primitive Pythagorean triplets. Furthermore, since j and l are
relatively prime, we must have j|r. This means that for each primitive
Pythagorean triplet, a can be any of ⌊A/j⌋ valid multiples of j, and r can
be any of ⌊B/j⌋ valid multiples of j, for a total of ⌊A/j⌋ ⌊B/j⌋ solutions.

If j is even, then the logic is very similar, but we cannot have a be an
even multiple of j and r be an odd multiple of j (or vice versa), because
that would make (b+c) and (b-c) have different parity. Therefore, they must
either both be even multiples (⌊A/j/2⌋ ⌊B/j/2⌋ solutions) or both be odd
multiples (⌊(A/j + 1) / 2⌋ ⌊(B/j + 1) / 2⌋ solutions).

Finally, we double the solutions to include quadruplets where a is negative,
and double again to also count F(A, B) in addition to F(B, A).
"""

from __future__ import annotations


def omegas(n: int) -> list[int]:
    """Count distinct prime factors for each number up to n."""
    result = [0] * (n + 1)
    for i in range(2, n + 1):
        if result[i] == 0:  # i is prime
            for j in range(i, n + 1, i):
                result[j] += 1
    return result


def solve() -> int:
    """Solve Problem 410."""
    A = 10**8
    B = 10**9

    omega = omegas(A)
    ans = A * B
    for j in range(2, A + 1):
        if j % 2 == 0:
            res = (A // j // 2) * (B // j // 2) + (
                (A // j + 1) // 2
            ) * ((B // j + 1) // 2)
        else:
            res = (A // j) * (B // j)
        ans += (1 << omega[j]) * res
    ans *= 4
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
