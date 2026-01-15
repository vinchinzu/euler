#!/usr/bin/env python3
"""
Project Euler 841 — Regular Star Polygons

Problem:
Find sum_{n=3}^{34} A(F_{n+1}, F_{n-1}).
A(p, q) is the area of alternating shading of {p/q} with inradius 1.

Solution:
Exact formula derived from winding number density:
A(p, q) = T_q + 2 * sum_{k=1}^{q-1} (-1)^(q-k) * T_k
where T_k = p * tan(k * pi / p).

This formula computes the area of regions with Odd winding numbers.
"""

from __future__ import annotations
from decimal import Decimal, getcontext
import mpmath
import time
from typing import List

# ---------------- Precision ----------------
# We need 10 digits of precision for the final answer.
# Intermediate sums involve large cancellations (alternating series).
# Use 50 digits to be safe.
getcontext().prec = 50
mpmath.mp.dps = 50
PI = mpmath.pi

# ---------------- Utilities ----------------
def fib_list(nmax: int) -> List[int]:
    F = [0, 1]
    for _ in range(2, nmax + 1):
        F.append(F[-1] + F[-2])
    return F

def solve_exact(p: int, q: int) -> mpmath.mpf:
    """
    Computes A(p,q) using the exact series formula.
    Time complexity: O(q).
    For max q ~ 3.5e6, this takes a few seconds.
    """
    
    # Precompute T(k) = p * tan(k * pi / p)
    # To optimize, we can compute tan iteratively or just call mpmath.tan.
    # mpmath.tan is reasonably fast.
    
    def T(k):
        return p * mpmath.tan(k * PI / p)
    
    # The summation is:
    # Sum = (-1)^(q-1)*T(1) + (-1)^(q-2)*T(2) + ... + (-1)^1*T(q-1)
    # We can loop from k=1 to q-1.
    
    # To minimize error accumulation, sum separate signs?
    # Or just sum sequentially. mpmath should handle it.
    
    s = mpmath.mpf(0)
    sign = 1 if (q - 1) % 2 == 0 else -1 
    # k=1: (-1)^(q-1).
    
    # Optimization: The terms T(k) are monotonic increasing?
    # k < p/2. tan is monotonic.
    # So T(k) increases.
    # Alternating sum of increasing sequence.
    
    for k in range(1, q):
        term = T(k)
        if sign > 0:
            s += term
        else:
            s -= term
        sign = -sign
        
    # Formula: A = T_q + 2 * Sum
    result = T(q) + 2 * s
    return result

def main():
    start_time = time.time()
    print("Project Euler 841 Solution")
    print("==========================")
    
    F = fib_list(36)
    total = mpmath.mpf(0)
    
    print(f"{'n':<3} {'p':<10} {'q':<10} {'A(p,q)':<20}")
    print("-" * 45)
    
    for n in range(3, 35):
        p = F[n+1]
        q = F[n-1]
        
        # Progress indication for large q
        t0 = time.time()
        area = solve_exact(p, q)
        dt = time.time() - t0
        
        print(f"{n:<3} {p:<10} {q:<10} {mpmath.nstr(area, 12)} ({dt:.2f}s)")
        total += area
        
    print("-" * 45)
    print(f"Total Sum: {total}")
    
    # Formatting for Euler
    # Round to 10 digits after decimal point.
    # "Give your answer rounded to 10 digits after the decimal point."
    
    # Use Decimal for final rounding to ensure correctness
    final_dec = Decimal(str(total))
    rounded = final_dec.quantize(Decimal("1e-10"), rounding="ROUND_HALF_UP")
    
    print(f"\nFinal Answer: {rounded}")
    print(f"Total Time: {time.time() - start_time:.2f}s")

if __name__ == "__main__":
    main()
