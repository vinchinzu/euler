"""Project Euler Problem 764: Sum of Solutions to 16x²+y⁴=z².

Find S(N), the sum of x+y+z for all solutions to 16x²+y⁴=z² with
1≤x,y,z≤N and GCD(x,y,z)=1.

We have y⁴ = (z-4x)(z+4x). Note that if p≠2 divides both z-4x and z+4x,
then it divides x, y, and z, a contradiction. So the only possible cases are:

(1) z-4x = a⁴,  z+4x = b⁴  (a < b)
(2) z-4x = 2a⁴, z+4x = 8b⁴ (a < b√2)
(3) z-4x = 8a⁴, z+4x = 2b⁴ (a√2 < b)

for integers a,b. Note that 4a⁴ and 4b⁴ is not possible, because that would
imply x,y,z are all even.

For case (1), we must have a and b odd, otherwise x,y,z are all even. Then
x=(b⁴-a⁴)/8, y=ab, and z=(a⁴+b⁴)/2. For case (2), x=(8b⁴-2a⁴)/8, y=2ab, and
z=(2a⁴+8b⁴)/2, so a must be even. For case (3), x=(2b⁴-8a⁴)/8, y=2ab, and
z=(8a⁴+2b⁴)/2, so b must be even.

Iterating over all a,b works, but for speed we iterate only on b and sum x+y+z
over all valid a. We can ignore the GCD requirement in the usual way, by using
Inclusion Exclusion via µ(n), though with some care for odd/even GCDs because
of the above parity constraints.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


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


def mod_invs(n: int, mod: int) -> List[int]:
    """Precompute modular inverses of 1..n modulo mod."""
    invs = [0] * (n + 1)
    invs[1] = 1
    for i in range(2, n + 1):
        invs[i] = (mod - (mod // i) * invs[mod % i] % mod) % mod
    return invs


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def tr(n: int) -> int:
    """Triangular number: n*(n+1)//2."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 764."""
    N = 10**16
    M = 10**9
    L = int((2 * N) ** (1.0 / 4))

    mobius = pre_mobius(L)
    mod_invs_arr = mod_invs(8, M)

    # Precompute sum of fourth powers
    sum_fourth_powers = [0] * L
    sum_odd_fourth_powers = [0] * L
    for i in range(1, L):
        sum_fourth_powers[i] = (sum_fourth_powers[i - 1] + pow_mod(i, 4, M)) % M
        sum_odd_fourth_powers[i] = (
            sum_odd_fourth_powers[i - 1] + pow_mod(2 * i - 1, 4, M)
        ) % M

    S = 0
    for g in range(1, L):
        n = N // pow_mod(g, 4, M)
        deg4 = 0
        deg2 = 0

        if g % 2 == 1:
            # Case (1): a and b odd
            b = 1
            while pow_mod(b, 4, M) <= 2 * n:
                if b % 2 == 1:
                    max_a = min(
                        b // 2,
                        int(((2 * n - pow_mod(b, 4, M)) ** (1.0 / 4) + 1) // 2),
                    )
                    deg4 = (
                        deg4
                        + (
                            (5 * mod_invs_arr[8] % M)
                            * max_a
                            % M
                            * pow_mod(b, 4, M)
                            % M
                            + (3 * mod_invs_arr[8] % M)
                            * sum_odd_fourth_powers[max_a]
                            % M
                        )
                        % M
                    ) % M
                    deg2 = (deg2 + sq(max_a) * b % M) % M
                b += 2

        # Case (2): z-4x = 2a⁴, z+4x = 8b⁴
        b = 1
        while 4 * pow_mod(b, 4, M) <= n:
            max_a = int(
                min(
                    (2**0.5) * b,
                    (n - 4 * pow_mod(b, 4, M)) ** (1.0 / 4),
                )
            )
            if g % 2 == 1:
                max_a //= 2
            mult = 1 if g % 2 == 0 else 2
            deg4 = (
                deg4
                + (
                    5 * max_a * pow_mod(b, 4, M) % M
                    + (3 * mod_invs_arr[4] % M)
                    * pow_mod(mult, 4, M)
                    % M
                    * sum_fourth_powers[max_a]
                    % M
                )
                % M
            ) % M
            deg2 = (deg2 + 2 * mult * tr(max_a) * b % M) % M
            b += 1

        # Case (3): z-4x = 8a⁴, z+4x = 2b⁴
        mult = 1 if g % 2 == 0 else 2
        b = mult
        while pow_mod(b, 4, M) <= n:
            max_a = int(
                min(
                    b / (2**0.5),
                    ((n - pow_mod(b, 4, M)) / 4.0) ** (1.0 / 4),
                )
            )
            deg4 = (
                deg4
                + (
                    (5 * mod_invs_arr[4] % M)
                    * max_a
                    % M
                    * pow_mod(b, 4, M)
                    % M
                    + 3 * sum_fourth_powers[max_a] % M
                )
                % M
            ) % M
            deg2 = (deg2 + 2 * tr(max_a) * b % M) % M
            b += mult

        S = (
            S
            + mobius[g]
            * (
                deg4 % M * pow_mod(g, 4, M) % M
                + deg2 % M * sq(g) % M
            )
            % M
        ) % M

    return S % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
