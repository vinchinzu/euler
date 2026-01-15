"""Project Euler Problem 155: Counting Capacitor Circuits."""

from math import gcd
from typing import Set, Tuple, List

def main() -> int:
    """Count distinct resistance values using n=18 1-ohm resistors."""
    N = 18
    # Using tuples (n, d) to represent fraction n/d
    # exact[k] stores set of values for k resistors.
    # We implicitly know 1/v is also in exact[k] if v is.
    # But for simplicity and set merging, we store all canonical forms.
    # Actually, we can just store ALL.
    # Optimization: Only compute sums (Series), then add reciprocals (Parallel).
    
    exact = [set() for _ in range(N + 1)]
    exact[1].add((1, 1))
    
    # Store exact[k] as list for faster iteration? No, set for 'in' check? 
    # Actually we iterate one and iterate other.
    # List is better for triangular loop.
    exact_lists = [[] for _ in range(N + 1)]
    exact_lists[1] = [(1, 1)]

    for k in range(2, N + 1):
        found_values = set()
        
        # Iterate split
        for i in range(1, (k // 2) + 1):
            j = k - i
            
            # Get sets (lists)
            L1 = exact_lists[i]
            L2 = exact_lists[j]
            
            # Series combination: x + y
            # x = n1/d1, y = n2/d2
            # sum = (n1 d2 + n2 d1) / (d1 d2)
            
            # If i == j, use triangular
            if i == j:
                len_L1 = len(L1)
                for idx1 in range(len_L1):
                    n1, d1 = L1[idx1]
                    # Self pair
                    # x+x = 2x. (2n1, d1)
                    # Parallel x,x = x/2. (n1, 2d1)
                    
                    # Optimization: x+x is sum. 1/(x+x) is parallel.
                    # Add sums
                    
                    # Inner loop
                    for idx2 in range(idx1, len_L1):
                        n2, d2 = L1[idx2]
                        
                        # Add
                        num = n1 * d2 + n2 * d1
                        den = d1 * d2
                        g = gcd(num, den)
                        s = (num // g, den // g)
                        found_values.add(s)
                        # Also add reciprocal (Parallel)
                        found_values.add((s[1], s[0]))
            else:
                # Cartesian product
                for n1, d1 in L1:
                    for n2, d2 in L2:
                         # Add
                        num = n1 * d2 + n2 * d1
                        den = d1 * d2
                        g = gcd(num, den)
                        s = (num // g, den // g)
                        found_values.add(s)
                        # Also add reciprocal
                        found_values.add((s[1], s[0]))

        exact[k] = found_values
        exact_lists[k] = list(found_values)

    # Collect all
    all_values = set()
    for k in range(1, N + 1):
        all_values.update(exact[k])
        
    return len(all_values)

if __name__ == "__main__":
    print(main())
