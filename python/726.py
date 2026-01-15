"""Project Euler Problem 726: Falling Bottles.

Consider a triangular stack of bottles with n layers. At each step, choose a
bottle and remove it. If there are bottles above the removed bottle, then
one of them collapses into the empty space, and this happens recursively. If
f(n) is the number of ways to take all the bottles from a stack, where
different collapsing processes are counted separately, then find Σ_{n=1}^N
f(n).

Number the layers from 1 to N, where layer 1 is the bottom layer. Instead of
considering the bottles that are removed, we consider the spaces that are
opened up. For example, the first step must always cause the top-most space
to open up. If a space opens up on layer k, then that means a bottle was
removed t layers below (where t≥0). For any t, there are 2^t possible
collapsing processes, because as we progress down each layer we can assume
that the bottle had collapsed either left or right. This means that the total
number of collapsing processes for that space over any t is c(k) = 1 + 2 +
... + 2^{k-1} = 2^k - 1.

Over the course of removing all the bottles, we will end up causing every
distinct space to open up exactly once. Given a particular order of these
spaces, the number of total collapsing processes, a(n), is the product of
c(k) over all spaces.

We can see that a(n) obeys the recurrence a(n) = a(n-1)² * (2^n - 1) /
a(n-2).

Now we need to determine the number of orderings that these spaces can be
opened up. This is the Young Tableau of a triangular grid with size n. The
total number of grid squares is tr(n), and the hook length of a space on
layer k is 2k - 1.

The product of the hook lengths, b(n), obeys a similar recurrence,
b(n) = b(n-1)² * (2n - 1) / b(n-2). This gives us the formula
f(n) = a(n) * tr(n)! / b(n).
"""

from __future__ import annotations


def tr(n: int) -> int:
    """Triangular number: n*(n+1)//2."""
    return n * (n + 1) // 2


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def factorial(n: int, mod: int) -> list[int]:
    """Precompute factorials modulo mod."""
    result = [1] * (n + 1)
    for i in range(1, n + 1):
        result[i] = (result[i - 1] * i) % mod
    return result


def sq(n: int, mod: int) -> int:
    """Square of n modulo mod."""
    return (n * n) % mod


def solve() -> int:
    """Solve Problem 726."""
    n = 10000
    m = 10**9 + 33

    factorials = factorial(tr(n), m)

    a = [0] * (n + 1)
    a[0] = a[1] = 1
    for i in range(2, n + 1):
        a[i] = (
            sq(a[i - 1], m)
            * (pow_mod(2, i, m) - 1)
            % m
            * mod_inv(a[i - 2], m)
            % m
        )

    b = [0] * (n + 1)
    b[0] = b[1] = 1
    for i in range(2, n + 1):
        b[i] = (
            sq(b[i - 1], m)
            * (2 * i - 1)
            % m
            * mod_inv(b[i - 2], m)
            % m
        )

    ans = 0
    for i in range(1, n + 1):
        ans = (
            ans
            + a[i] * factorials[tr(i)] % m * mod_inv(b[i], m)
        ) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
