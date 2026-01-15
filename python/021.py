#!/usr/bin/env python3
"""
Project Euler Problem 21: Amicable numbers

Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
Evaluate the sum of all the amicable numbers under 10000.
"""

LIMIT = 10000
EXTENDED_LIMIT = 20000


def sum_of_proper_divisors(n: int) -> int:
    """Calculate the sum of proper divisors of n."""
    if n <= 1:
        return 0
    
    sum_val = 1
    sqrt_n = int(n ** 0.5)
    
    for i in range(2, sqrt_n + 1):
        if n % i == 0:
            sum_val += i
            counterpart = n // i
            if counterpart != i and counterpart != n:
                sum_val += counterpart
    
    return sum_val


def precompute_divisor_sums(limit: int) -> list[int]:
    """Sieve-based precomputation of divisor sums."""
    divisor_sums = [0] * (limit + 1)
    
    for i in range(1, limit + 1):
        for multiple in range(2 * i, limit + 1, i):
            divisor_sums[multiple] += i
    
    return divisor_sums


def find_amicable_numbers_sum(limit: int) -> int:
    """Find the sum of all amicable numbers under the limit."""
    extended_divisor_sums = precompute_divisor_sums(EXTENDED_LIMIT)
    
    amicable_numbers = []
    
    for a in range(1, limit):
        b = extended_divisor_sums[a]
        
        if b == a or b > EXTENDED_LIMIT:
            continue
        
        if extended_divisor_sums[b] == a and a != b:
            amicable_numbers.append(a)
    
    return sum(set(amicable_numbers))


def main():
    result = find_amicable_numbers_sum(LIMIT)
    print(result)


if __name__ == "__main__":
    main()
