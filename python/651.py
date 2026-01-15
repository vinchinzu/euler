"""Project Euler Problem 651: Patterns of Rectangular Stickers.

Let f(m,a,b) be the number of patterns of rectangular stickers on a cylinder
using exactly m colors, having the pattern repeat along the cylinder every a
stickers, and with b stickers around the circumference, if translations,
reflections, and rotations are not considered unique. Find Σ_{i=4}^N f(i,
F_{i-1}, F_i) where F_i are the Fibonacci numbers.

We use Burnside's Lemma. The group operations are the (a*b) translations,
doubled once for reflections in the x direction and doubled again for
reflections in the y direction. Each operation is a combination of a permutation
of the rows and a permutation of the columns, and the number of orbits of the
operation is the sum of the GCD of all pairs of cycles of the two permutations.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from math import gcd

from sympy import factorint, primerange


def mod_inverse(a: int, m: int) -> int:
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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def fibonacci_list(n: int) -> list[int]:
    """Generate first n Fibonacci numbers."""
    if n == 0:
        return []
    if n == 1:
        return [1]
    fib = [1, 1]
    for _ in range(2, n):
        fib.append(fib[-1] + fib[-2])
    return fib


def nCr_table(n: int, mod: int) -> list[list[int]]:
    """Precompute binomial coefficients up to n."""
    table = [[0] * (n + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        table[i][0] = 1
        for j in range(1, i + 1):
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % mod
    return table


def lphi(n: int) -> int:
    """Euler's totient function."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result = result // p * (p - 1)
        p += 1
    if n > 1:
        result = result // n * (n - 1)
    return result


def all_divisors(n: int) -> list[int]:
    """Get all divisors of n."""
    factors = factorint(n)
    divisors = [1]
    for p, exp in factors.items():
        new_divs = []
        for d in divisors:
            for e in range(1, exp + 1):
                new_divs.append(d * (p**e))
        divisors.extend(new_divs)
    return sorted(divisors)


def cycle_lens(n: int, reflect: bool) -> dict[Counter[int], int]:
    """Get cycle lengths for permutation of n elements."""
    if reflect:
        if n % 2 == 1:
            cycle_map = Counter({2: n // 2, 1: 1})
            return {cycle_map: n}
        else:
            cycle_map1 = Counter({2: n // 2})
            cycle_map2 = Counter({2: n // 2 - 1, 1: 2})
            return {cycle_map1: n // 2, cycle_map2: n // 2}

    cycle_lens_map: dict[Counter[int], int] = {}
    for d in all_divisors(n):
        cycle_map = Counter({n // d: d})
        cycle_lens_map[cycle_map] = lphi(n // d)
    return cycle_lens_map


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


def f(m: int, a: int, b: int, mod: int) -> int:
    """Compute f(m, a, b)."""
    nCrs = nCr_table(m, mod)
    result = 0

    for reflect_w in [False, True]:
        for reflect_b in [False, True]:
            cycle_lens_a = cycle_lens(a, reflect_w)
            cycle_lens_b = cycle_lens(b, reflect_b)

            for cycle_map_a, count_a in cycle_lens_a.items():
                for cycle_map_b, count_b in cycle_lens_b.items():
                    num_cycles = 0
                    for la in cycle_map_a:
                        for lb in cycle_map_b:
                            num_cycles += (
                                gcd(la, lb)
                                * cycle_map_a[la]
                                * cycle_map_b[lb]
                            )

                    for i in range(m):
                        term = (
                            parity(i)
                            * nCrs[m][i]
                            % mod
                            * pow_mod(m - i, num_cycles, mod)
                            % mod
                            * count_a
                            % mod
                            * count_b
                            % mod
                        )
                        result = (result + term) % mod

    inv = mod_inverse(4 * a * b, mod)
    return result * inv % mod


def solve() -> int:
    """Solve Problem 651."""
    N = 40
    M = 10**9 + 7

    fibonaccis = fibonacci_list(N + 1)
    ans = 0
    for i in range(4, N + 1):
        ans = (ans + f(i, fibonaccis[i - 1], fibonaccis[i], M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
