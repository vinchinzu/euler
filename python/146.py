"""Project Euler Problem 146.

Sum of n < 150,000,000 such that n^2 + 1,3,7,9,13,27 are consecutive primes.
"""

from sympy import isprime

def main() -> int:
    """Main function."""
    LIMIT = 150_000_000
    total_sum = 0
    
    # Constraints found:
    # n % 2 == 0
    # n % 5 == 0  (so n % 10 == 0)
    # n^2 % 3 == 1 (to avoid n^2+3, n^2+9, n^2+27 div by 3 or make them mod 3 != 0)
    # n^2 % 7 == 2 (to avoid n^2+1, n^2+3, etc div by 7)
    # n^2 % 13 in {1, 3, 9}
    
    # We iterate n with step 10.
    # Check modulo constraints first.
    
    for n in range(10, LIMIT, 10):
        # Modulo 3 check: n % 3 != 0 => n^2 % 3 == 1
        if n % 3 == 0:
            continue
            
        # Modulo 7 check: n^2 % 7 == 2 => n % 7 in (3, 4)
        r7 = n % 7
        if r7 != 3 and r7 != 4:
            continue
            
        # Modulo 13 check: n^2 % 13 in {1, 3, 9}
        # n % 13 in {1, 3, 4, 9, 10, 12}
        r13 = n % 13
        if r13 not in (1, 3, 4, 9, 10, 12):
            continue
            
        # If passed modulo filters, n^2 is potentially good.
        sq = n * n
        
        # Check primary primes
        # Order by likelihood of failure? 
        # 1, 3, 7, 9, 13, 27
        if not isprime(sq + 1): continue
        if not isprime(sq + 3): continue
        if not isprime(sq + 7): continue
        if not isprime(sq + 9): continue
        if not isprime(sq + 13): continue
        if not isprime(sq + 27): continue
        
        # Check intermediate composites
        #Evens are composite. Div by 5 are composite (n%5==0 => n^2+k check).
        # n%5==0, so sq%5==0. 
        # sq+5, sq+15, sq+25 are div by 5.
        # Remaining odd offsets: 11, 17, 19, 21, 23
        
        if isprime(sq + 11): continue
        if isprime(sq + 17): continue
        if isprime(sq + 19): continue
        if isprime(sq + 21): continue
        if isprime(sq + 23): continue
        
        total_sum += n
        
    return total_sum

if __name__ == "__main__":
    print(main())
