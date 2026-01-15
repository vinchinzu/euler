"""Project Euler Problem 592: Factorial.

Find the last K hexadecimal digits before the trailing zeroes in N!.

If we compute the product of all prime factors of N! other than 2, and
multiply that with the remaining factors of 2 when all quadruples of 2s
are removed (a quadruple of 2s multiply to 16, which corresponds to a
trailing zero in hexadecimal), then the remainder of that product
(mod 16^K) are the last K hexadecimal digits.
"""

from __future__ import annotations

from math import factorial


def mod_inverse(a: int, m: int) -> int:
    """Compute modular inverse using extended Euclidean algorithm."""
    def ext_gcd(a: int, b: int) -> tuple[int, int, int]:
        if b == 0:
            return (a, 1, 0)
        g, x1, y1 = ext_gcd(b, a % b)
        return (g, y1, x1 - (a // b) * y1)

    g, x, _y = ext_gcd(a, m)
    if g != 1:
        raise ValueError("Modular inverse does not exist")
    return (x % m + m) % m


def solve() -> str:
    """Solve Problem 592."""
    N = factorial(20)
    K = 12
    C = 1 << (2 * K)  # 16^K

    # Precompute products of odd numbers
    prod_odds = [1] * (C + 1)
    for i in range(1, C // 2 + 1):
        prod_odds[2 * i - 1] = prod_odds[2 * i - 2] * (2 * i - 1) % C
        prod_odds[2 * i] = prod_odds[2 * i - 1]

    # Precompute modular inverses
    mod_invs = [0] * (C + 1)
    mod_invs[1] = 1
    for i in range(3, C):
        if (C % i) % 2 == 0:
            mod_invs[i] = ((C // i + 1) * mod_invs[i - C % i]) % C
        else:
            mod_invs[i] = (C - (C // i) * mod_invs[C % i] % C) % C

    # Precompute sum of modular inverses
    sum_mod_invs = [0] * (C + 1)
    for i in range(1, C + 1):
        sum_mod_invs[i] = (sum_mod_invs[i - 1] + mod_invs[i]) % C

    num_twos = 0
    ans = 1

    lim = N
    while lim > 0:
        # Parity adjustment
        if (lim // (C * C)) % 2 == 1:
            ans = (ans * (-1)) % C

        R = lim % (C * C)
        # Process blocks
        for i in range(R // C):
            term = (prod_odds[C] * (i * C * sum_mod_invs[C] + 1)) % C
            ans = (ans * term) % C

        r = int(R % C)
        term = (prod_odds[r] * ((R // C) * C * sum_mod_invs[r] + 1)) % C
        ans = (ans * term) % C

        num_twos += lim // 2
        lim //= 2

    # Add remaining factors of 2
    ans = (ans * pow(2, num_twos % 4, C)) % C
    ans = ans % (C * C)

    return format(ans, "X")


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
