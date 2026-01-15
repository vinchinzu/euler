"""Project Euler Problem 195: Inscribed circles of triangles with one angle of 60 degrees."""

import math

def main() -> int:
    T = 1_053_779
    # We need r <= T
    # Derived relationship for 60 degree triangle:
    # r = (sqrt(3)/2) * n * k  (parameterized)
    # or scaled if divisibility conditions met.
    
    limit_val = 2 * T / math.sqrt(3)
    L = int(limit_val)
    
    total = 0
    limit_3L = 3 * L
    
    for n in range(1, limit_3L + 1):
        max_k = limit_3L // n
        if max_k < 1: break
        
        for k in range(1, max_k + 1):
            if math.gcd(n, k) != 1:
                continue
                
            m = n + k
            
            # If (m+n) is divisible by 3, we can scale down the sides by 3.
            # This increases the effective r limit (or allows larger n, k for same r).
            if (m + n) % 3 == 0:
                res = int( (3 * limit_val) / (n * k) )
                total += res
            else:
                res = int( limit_val / (n * k) )
                total += res

    # The loop counts each scalene shape twice (once for (n,k) and once for (k,n)).
    # Equilateral shapes (n=k=1) are counted once.
    # We want: Scalene + Equilateral
    # Computed Total = 2*Scalene + Equilateral
    # We need to adjust.
    # Count_Eq approx 3 * limit_val (since n=k=1 => m+n=3 => div 3 case).
    
    count_eq = int(3 * limit_val)
    
    return (total + count_eq) // 2

if __name__ == "__main__":
    print(main())
