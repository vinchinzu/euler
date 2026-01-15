"""Project Euler Problem 123.

Let p_n be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when
(p_n - 1)^n + (p_n + 1)^n is divided by p_n^2.

For n even, remainder is 2.
For n odd, remainder is 2*n*p_n mod p_n^2.
For large n, 2*n < p_n, so remainder is 2*n*p_n.

We want smallest n such that remainder > 10^10.
"""

from sympy import sieve

def solve():
    target = 10**10
    
    # We can start searching from a reasonable n.
    # 2 * n * p_n > 10^10 => 2 * n * (n log n) approx > 10^10
    # n approx 21000.
    
    # Ensure sieve has enough primes
    limit_n = 50000
    sieve.extend_to_no(limit_n)
    
    for n in range(1, limit_n + 1, 2):  # Only check odd n
        p_n = sieve[n]
        remainder = 2 * n * p_n
        if remainder > target:
            return n
            
    return None

if __name__ == "__main__":
    print(solve())
