"""
Project Euler Problem 857: Beautiful Graphs

A graph is made up of vertices and coloured edges. 
Between every two distinct vertices there must be exactly one of the following:
- A red directed edge one way, and a blue directed edge the other way
- A green undirected edge
- A brown undirected edge

Such a graph is called beautiful if 
- A cycle of edges contains a red edge if and only if it also contains a blue edge
- No triangle of edges is made up of entirely green or entirely brown edges

Let G(n) be the number of beautiful graphs on the labelled vertices: 1,2,...,n.
You are given G(3)=24, G(4)=186 and G(15)=12472315010483328.

Find G(10^7). Give your answer modulo 10^9+7.
"""
import sys

# Precomputed A(n) values for n=1..5
# A(n) is the number of ways to color K_n with Green/Brown edges
# such that there are no monochromatic triangles.
# Calculated using analyze_A.py:
# A(1) = 1
# A(2) = 2
# A(3) = 6
# A(4) = 18
# A(5) = 12
# A(6) = 0 (and 0 for all n >= 6 by Ramsey's Theorem)
A_VALS = [0, 1, 2, 6, 18, 12]

def solve_g(n: int, modulus: int = 1_000_000_007) -> int:
    """
    Computes G(n) modulo modulus.
    Formula: G(n) = sum_{k=1 to min(n,5)} binom(n, k) * A(k) * G(n-k)
    """
    if n == 0:
        return 1
        
    # Use simple logic for small n to allow exact verification (modulus=None)
    if n < 100:
        g = [0] * (n + 1)
        g[0] = 1
        for i in range(1, n + 1):
            val = 0
            limit = min(i, 5)
            for k in range(1, limit + 1):
                # binom(i, k) computed iteratively
                comb = 1
                for x in range(k):
                    comb = comb * (i - x) // (x + 1)
                
                if modulus:
                    term = (comb * A_VALS[k] * g[i-k]) % modulus
                    val = (val + term) % modulus
                else:
                    val += comb * A_VALS[k] * g[i-k]
            g[i] = val
        return g[n]
    
    # For large n, optimize space and modular arithmetic
    # Precompute modular inverses for k! to fast compute binom
    # binom(n, k) = n * (n-1) * ... * (n-k+1) * inv(k!)
    
    if not modulus:
        raise ValueError("Modulus required for large n")
        
    # Precompute inverse factorials for 1..5
    inv_fact = [1] * 6
    fact = 1
    for i in range(1, 6):
        fact = (fact * i) % modulus
        # Modular inverse using Fermat's Little Theorem
        inv_fact[i] = pow(fact, modulus - 2, modulus)
        
    # History: stores G values.
    # We need the last 5 values to compute current G.
    # g_hist will maintain a sliding window of the last 5 values.
    # Initially, we represent G(-4)..G(0).
    # Since G(x)=0 for x<0, the initial window is [0, 0, 0, 0, 1].
    # g_hist[0] corresponds to G(current_n - 5)
    # g_hist[4] corresponds to G(current_n - 1)
    
    g_hist = [0, 0, 0, 0, 1] 
    
    for i in range(1, n + 1):
        val = 0
        
        # Compute binom(i, k) terms on the fly
        current_n_prod = 1
        
        for k in range(1, 6): # k = 1..5
            # prev_g corresponds to G(i-k)
            # If k=1, we want G(i-1), which is at index 4 (end) -> 5-1=4
            # If k=5, we want G(i-5), which is at index 0 (start) -> 5-5=0
            prev_g = g_hist[5-k]
            
            # Update numerator for binom(i, k)
            # binom(i, k) = (i * ... * (i-k+1)) / k!
            current_n_prod = (current_n_prod * (i - k + 1)) % modulus
            binom = (current_n_prod * inv_fact[k]) % modulus
            
            term = (binom * A_VALS[k] * prev_g) % modulus
            val = (val + term) % modulus
            
        # Update history: remove oldest G, add new G
        g_hist.pop(0)
        g_hist.append(val)
            
    return g_hist[-1]

def solve():
    target = 10**7
    result = solve_g(target)
    return result

if __name__ == "__main__":
    print(solve())
