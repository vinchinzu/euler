#!/usr/bin/env python3
"""
Project Euler Problem 1: Multiples of 3 or 5

If we list all the natural numbers below 10 that are multiples of 3 or 5,
we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
"""

def sum_of_multiples(divisor: int, limit: int) -> int:
    """Calculate sum of multiples of a divisor using arithmetic series formula."""
    if limit <= 1:
        return 0
    num_terms = (limit - 1) // divisor
    return divisor * num_terms * (num_terms + 1) // 2


def inclusion_exclusion_sum(limit: int, divisors: list[int] = [3, 5]) -> int:
    """Calculate sum of multiples of any divisors using inclusion-exclusion principle."""
    if limit <= 1:
        return 0
    
    # For two divisors, simply: sum3 + sum5 - sum(lcm)
    sum_result = 0
    from math import gcd
    lcm = divisors[0] * divisors[1] // gcd(divisors[0], divisors[1])
    
    for d in divisors:
        sum_result += sum_of_multiples(d, limit)
    sum_result -= sum_of_multiples(lcm, limit)
    
    return sum_result


def main():
    limit = 1000
    result = inclusion_exclusion_sum(limit)
    print(result)


if __name__ == "__main__":
    main()
