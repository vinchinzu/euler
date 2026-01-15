"""Project Euler Problem 133: Repunit non-factors.

Find the sum of all primes below one-hundred thousand that will never be a factor of R(10^n).
A prime p is never a factor of R(10^n) if its order modulo p, ord_p(10),
is not of the form 2^a * 5^b.
"""

from sympy import primerange, factorint


def power(base: int, exp: int, mod: int) -> int:
    """Calculates (base^exp) % mod."""
    res = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            res = (res * base) % mod
        base = (base * base) % mod
        exp //= 2
    return res


def multiplicative_order(n: int, prime_mod: int) -> int:
    """Calculates the multiplicative order of n modulo prime_mod.
    
    Assumes prime_mod is prime and n is not a multiple of prime_mod
    """
    # For p > 5, 10 is not a multiple of p, so gcd(10, p) = 1
    # Thus, an order should always exist.

    phi = prime_mod - 1  # By Fermat's Little Theorem, since prime_mod is prime

    # Get prime factors of phi
    phi_factors = list(factorint(phi).keys())

    order = phi
    for factor in phi_factors:
        # Repeatedly divide order by factor as long as the congruence holds
        while (order % factor) == 0 and power(n, order // factor, prime_mod) == 1:
            order //= factor
    return order


def order_is_power_of_2_or_5(order: int) -> bool:
    """Checks if the order contains any prime factors other than 2 or 5.
    
    Returns true if the order IS of the form 2^a * 5^b, false otherwise.
    """
    # This case should ideally not be hit if multiplicative_order is called correctly
    # for p > 5 and n=10, as an order must exist.
    if order == 0:
        return False

    temp_order = order
    # Remove all factors of 2
    while temp_order % 2 == 0:
        temp_order //= 2
    # Remove all factors of 5
    while temp_order % 5 == 0:
        temp_order //= 5
    # If temp_order is now 1, then order was of the form 2^a * 5^b
    return temp_order == 1


def main() -> int:
    """Main function."""
    # Primes 2, 3, 5 are never factors of R(10^n) for any n >= 1
    total_sum = 2 + 3 + 5
    
    for p in primerange(2, 100_000):
        if p <= 5:
            continue
        order = multiplicative_order(10, p)
        if not order_is_power_of_2_or_5(order):
            total_sum += p
    return total_sum


if __name__ == "__main__":
    print(main())
