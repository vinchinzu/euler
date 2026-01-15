"""
Project Euler Problem 863
Using only a six-sided fair dice and a five-sided fair dice, we would like to emulate an n-sided fair dice.
...
Find S(1000). Give your answer rounded to 6 decimal places.
"""

import sys
from typing import List

def solve_r(n: int) -> float:
    # Value iteration to find min expected rolls
    # V[r] = expected additional rolls from remainder r
    # V[r] = min(1 + (r*5 % n) / (r*5) * V[r*5 % n],
    #            1 + (r*6 % n) / (r*6) * V[r*6 % n])
    # V[0] = 0
    
    v: List[float] = [0.0] * n
    
    # Increase iterations for safety with larger N
    # Typical convergence is related to 1/(1 - max_prob_continue)
    # max_prob_continue approx 1 - 1/(6N) ? No, approx 1.
    # But we assume the graph structure forces reduction eventually.
    # 50000 should be plenty for N=1000.
    
    max_iter = 200000
    tol = 1e-11
    
    for _ in range(max_iter):
        max_diff = 0.0
        # We can't update in place for synchronous updates (Value Iteration),
        # or we can (Gauss-Seidel) which is faster.
        # Gauss-Seidel is usually safe for this type of problem.
        # But let's stick to synchronous to be theoretically clean, 
        # or just use 2 arrays.
        
        new_v = list(v)
        
        for r in range(1, n):
            # Try D5
            r5 = r * 5
            rem5 = r5 % n
            # If rem5 == 0, V[0] is 0, term becomes 0
            term5 = (rem5 / r5) * v[rem5] if rem5 != 0 else 0.0
            val5 = 1.0 + term5
            
            # Try D6
            r6 = r * 6
            rem6 = r6 % n
            term6 = (rem6 / r6) * v[rem6] if rem6 != 0 else 0.0
            val6 = 1.0 + term6
            
            best = val5 if val5 < val6 else val6
            
            diff = abs(best - v[r])
            if diff > max_diff:
                max_diff = diff
            new_v[r] = best
            
        v = new_v
        if max_diff < tol:
            break
            
    return v[1]

def solve() -> float:
    total_s = 0.0
    limit = 1000
    for k in range(2, limit + 1):
        total_s += solve_r(k)
    return total_s

if __name__ == "__main__":
    result = solve()
    print(f"{result:.6f}")
