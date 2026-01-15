#!/usr/bin/env python3
"""
Prime power triples (Problem 87)

How many numbers below fifty million can be expressed as the sum of a prime
square, prime cube, and prime fourth power?
"""

import math

LIMIT = 50_000_000


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Generate primes up to limit using sieve."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    
    for p in range(2, int(math.sqrt(limit)) + 1):
        if is_prime[p]:
            for multiple in range(p * p, limit + 1, p):
                is_prime[multiple] = False
    
    return [num for num in range(2, limit + 1) if is_prime[num]]


def main() -> None:
    """Count numbers expressible as sum of prime powers."""
    # Determine maximum primes needed
    p1_max_val = int(math.sqrt(LIMIT))
    p2_max_val = int(LIMIT ** (1.0/3.0))
    p3_max_val = int(LIMIT ** (1.0/4.0))
    
    sieve_limit = p1_max_val
    primes_list = sieve_of_eratosthenes(sieve_limit)
    
    # Filter primes for each power
    primes_for_square = [p for p in primes_list if p <= p1_max_val]
    primes_for_cube = [p for p in primes_list if p <= p2_max_val]
    primes_for_fourth = [p for p in primes_list if p <= p3_max_val]
    
    # Store unique sums
    found_sums = set()
    
    # Iterate through combinations
    for p3 in primes_for_fourth:
        p3_fourth = p3 ** 4
        if p3_fourth >= LIMIT:
            break
        
        for p2 in primes_for_cube:
            p2_cube = p2 ** 3
            sum_p3_p2 = p3_fourth + p2_cube
            if sum_p3_p2 >= LIMIT:
                break
            
            for p1 in primes_for_square:
                p1_square = p1 ** 2
                current_total_sum = sum_p3_p2 + p1_square
                
                if current_total_sum < LIMIT:
                    found_sums.add(current_total_sum)
                else:
                    break
    
    print(len(found_sums))


if __name__ == "__main__":
    main()
