"""Project Euler Problem 194: Coloured Configurations."""

import math

MOD = 100_000_000
A_COUNT = 25
B_COUNT = 75
C = 1984


def mod_pow(base: int, exponent: int) -> int:
    """Modular exponentiation."""
    base %= MOD
    result = 1
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % MOD
        base = (base * base) % MOD
        exponent >>= 1
    return result


def binomial_coefficient(n: int, k: int) -> int:
    """Compute binomial coefficient C(n,k) mod MOD using math.comb."""
    k = min(k, n - k)
    if k == 0:
        return 1
    # Use math.comb which handles large numbers correctly, then take modulo
    return math.comb(n, k) % MOD


def compute_value(a: int, b: int, c: int) -> int:
    """Compute the value."""
    base_factor = (c % MOD) * ((c - 1) % MOD) % MOD
    comb = binomial_coefficient(a + b, a)
    transfer_a = (2 * (c - 1)) % MOD
    transfer_b = (20 * c * c - 58 * c) % MOD

    result = base_factor
    result = (result * comb) % MOD
    result = (result * mod_pow(transfer_a, a)) % MOD
    result = (result * mod_pow(transfer_b, b)) % MOD
    return result


def main() -> int:
    """Main function."""
    return compute_value(A_COUNT, B_COUNT, C)


if __name__ == "__main__":
    print(main())
