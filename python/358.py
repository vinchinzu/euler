"""Project Euler Problem 358: Cyclic Numbers

A cyclic number with n digits has the property that when multiplied by 1, 2, 3, ..., n,
all products contain the same digits in identical order but cyclically rotated.

Cyclic numbers are related to full reptend primes p where 10 is a primitive root mod p.
The cyclic number is c = (10^(p-1) - 1) / p, which equals the repeating decimal of 1/p.

We need to find the cyclic number where:
- First 11 digits are "00000000137"
- Last 5 digits are "56789"

Then compute the sum of all its digits.

Key insights:
1. The first digits of the cyclic number = first digits of 1/p (the decimal expansion)
   So 1/p starts with 0.00000000137... which means p is between 10^11/138 and 10^11/137

2. The last 5 digits being 56789 means: cyclic * p ≡ 99999 (mod 10^5)
   Since cyclic = (10^(p-1) - 1) / p, we have cyclic * p = 10^(p-1) - 1
   So 56789 * p ≡ 99999 (mod 10^5), i.e., 56789 * p + 1 ≡ 0 (mod 10^5)

3. We can compute the digit sum via long division without computing all p-1 digits.
"""

from __future__ import annotations


def is_prime(n: int) -> bool:
    """Miller-Rabin primality test for integers."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    if n < 9:
        return True
    if n % 3 == 0:
        return False

    # Write n-1 as 2^r * d
    r, d = 0, n - 1
    while d % 2 == 0:
        r += 1
        d //= 2

    # Witnesses to test (sufficient for all n < 3,317,044,064,679,887,385,961,981)
    witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]

    for a in witnesses:
        if a >= n:
            continue
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(r - 1):
            x = pow(x, 2, n)
            if x == n - 1:
                break
        else:
            return False
    return True


def compute_digit_sum_and_verify(p: int) -> int | None:
    """
    Compute the digit sum of the cyclic number for prime p.

    The cyclic number is the repeating decimal of 1/p, computed via long division.
    Also verifies that the cycle length is exactly p-1 (full reptend prime check).

    Returns the digit sum if p is a full reptend prime, None otherwise.
    """
    digit_sum = 0
    n = 1  # Start with 1/p, numerator = 1

    # We need to perform p-1 divisions to get all digits
    # and verify n returns to 1 at exactly step p-1
    for i in range(1, p):
        n *= 10
        digit = n // p
        digit_sum += digit
        n = n % p

        # If n becomes 1 before we've done p-1 steps, cycle length < p-1
        # This means p is not a full reptend prime
        if n == 1 and i < p - 1:
            return None

    # After p-1 steps, n should be back to 1 for a full reptend prime
    if n != 1:
        return None

    return digit_sum


def solve() -> int:
    """
    Find the cyclic number with first 11 digits "00000000137" and last 5 digits "56789".
    Return the sum of all its digits.

    Approach:
    1. First digits constraint: 1/p starts with 0.00000000137...
       This means 10^11/138 < p < 10^11/137
       i.e., 724637681.159... < p < 729927007.299...

    2. Last digits constraint: 56789 * p + 1 ≡ 0 (mod 10^5)
       i.e., 56789 * p ≡ -1 ≡ 99999 (mod 10^5)

    3. For each candidate prime in the range that satisfies the last-digits constraint,
       verify it's a full reptend prime and compute the digit sum.
    """
    # Range from first-digits constraint
    # 1/p must start with 0.00000000137
    # So p must be in the range where floor(10^11 / p) starts with 137
    lower = 10**11 // 138 + 1  # p > 10^11/138
    upper = 10**11 // 137      # p < 10^11/137

    # Find modular inverse to solve 56789 * p ≡ 99999 (mod 10^5)
    # p ≡ 99999 * inverse(56789) (mod 10^5)
    # We need to find inverse of 56789 mod 10^5

    # Extended Euclidean algorithm for inverse
    def mod_inverse(a: int, m: int) -> int:
        g, x, _ = extended_gcd(a, m)
        if g != 1:
            return -1  # No inverse
        return x % m

    def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
        if a == 0:
            return b, 0, 1
        g, x, y = extended_gcd(b % a, a)
        return g, y - (b // a) * x, x

    inv = mod_inverse(56789, 100000)
    target_remainder = (99999 * inv) % 100000  # p ≡ this (mod 10^5)

    # Find first candidate >= lower with p ≡ target_remainder (mod 10^5)
    start = lower + (target_remainder - lower % 100000) % 100000

    # Check each candidate
    for p in range(start, upper + 1, 100000):
        if not is_prime(p):
            continue

        # Verify first digits more precisely
        # We need 1/p to have decimal starting with 0.00000000137
        # Check: floor(10^13 / p) should start with 137 (we use more precision)
        first_digits = 10**13 // p
        if not (13700 <= first_digits <= 13799):
            continue

        # Verify last digits: 56789 * p ≡ 99999 (mod 10^5)
        if (56789 * p + 1) % 100000 != 0:
            continue

        # Now compute digit sum and verify full reptend
        digit_sum = compute_digit_sum_and_verify(p)
        if digit_sum is not None:
            return digit_sum

    return -1  # Should not reach here


if __name__ == "__main__":
    print(solve())
