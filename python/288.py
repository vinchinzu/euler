"""Project Euler Problem 288: An enormous factorial.

Let N(P, Q) be a number written with Q+1 digits in base P. Find the number of
factors P in N(P, Q)!, mod P^E.

The number of factors P is ⌊N(P, Q) / P⌋ + ⌊N(P, Q) / P²⌋ + ..., and since
we only need to compute the answer (mod P^E), for each ⌊N(P, Q) / P^i⌋ we
need to only look at up to the E digits of N(P, Q) ending at place value i.
"""

from __future__ import annotations


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod == 1:
        return 0
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp //= 2
        base = (base * base) % mod
    return result


def blum_blum_shub(seed: int, n: int, p: int) -> list[int]:
    """Generate Blum Blum Shub sequence."""
    result: list[int] = []
    x = seed
    for _ in range(n):
        x = (x * x) % p
        result.append(x % p)
    return result


def solve() -> int:
    """Solve Problem 288."""
    P = 61
    Q = ipow(10, 7)
    E = 10
    M = pow_mod(P, E, 10**20)  # Large enough mod

    # Generate Blum Blum Shub sequence
    T = blum_blum_shub(0, Q + 1, P)

    # Precompute powers of P
    pows = [1] * E
    for i in range(1, E):
        pows[i] = pows[i - 1] * P

    ans = 0
    for i in range(1, len(T)):
        for j in range(E):
            if i + j < len(T):
                ans = (ans + T[i + j] * pows[j]) % M

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
