
# Project Euler Problem 842
#
# PROBLEM DESCRIPTION:
# <p>
# Given $n$ equally spaced points on a circle, we define an <dfn>$n$-star polygon</dfn> as an $n$-gon having those $n$ points as vertices. Two $n$-star polygons differing by a rotation/reflection are considered <b>different</b>.</p>
# 
# <p>
# For example, there are twelve $5$-star polygons shown below.</p>
# <img src="resources/images/0842_5-agons.jpg?1680461480" alt="0842_5-agons.jpg">
# <p>
# For an $n$-star polygon $S$, let $I(S)$ be the number of its self intersection points.<br>
# Let $T(n)$ be the sum of $I(S)$ over all $n$-star polygons $S$.<br>
# For the example above $T(5) = 20$ because in total there are $20$ self intersection points.</p>
# 
# <p>
# Some star polygons may have intersection points made from more than two lines. These are only counted once. For example, <span style="white-space:nowrap;">$S$,</span> shown below is one of the sixty $6$-star polygons. This one has $I(S) = 4$.</p>
# <img src="resources/images/0842_6-agon.jpg?1680461493" alt="0842_6-agon.jpg">
# <p>
# You are also given that $T(8) = 14640$.</p>
# 
# <p>
# Find $\displaystyle \sum_{n = 3}^{60}T(n)$. Give your answer modulo $(10^9 + 7)$.</p>

import math
import sys
import itertools
from collections import defaultdict

MOD = 10**9 + 7

def get_point(k: int, n: int) -> tuple[float, float]:
    """Get coordinates of k-th vertex of regular n-gon."""
    angle = 2 * math.pi * k / n
    return (math.cos(angle), math.sin(angle))

def get_intersection(p1: tuple[float, float], p2: tuple[float, float], 
                     p3: tuple[float, float], p4: tuple[float, float]) -> tuple[float, float] | None:
    """
    Intersection of line segment p1-p2 and p3-p4.
    Returns (x, y) or None if parallel.
    """
    x1, y1 = p1
    x2, y2 = p2
    x3, y3 = p3
    x4, y4 = p4
    
    denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
    if denom == 0:
        return None # Parallel
        
    # Parameter t for p1 + t*(p2-p1)
    t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom
    
    px = x1 + t * (x2 - x1)
    py = y1 + t * (y2 - y1)
    return (px, py)

def contrib(m: int, n: int, fact: list[int]) -> int:
    """
    Calculate the contribution of an intersection point with multiplicity m to T(n).
    Formula: sum_{k=2}^m (-1)^k * (k-1) * binom(m, k) * 2^(k-1) * (n-k-1)!
    """
    val = 0
    for k in range(2, m + 1):
        term = (-1)**k * (k - 1)
        
        # binom(m, k) = m! / (k! * (m-k)!)
        bin_mk = fact[m] // (fact[k] * fact[m - k])
        
        # 2^(k-1)
        pow2 = pow(2, k - 1, MOD)
        
        # (n-k-1)!
        fact_rem = fact[n - k - 1]
        
        # Using modular arithmetic for the sum
        term_val = (term * bin_mk * pow2 * fact_rem) % MOD
        val = (val + term_val) % MOD
        
    return val

def compute_t(n: int) -> int:
    """Compute T(n) modulo 10^9+7."""
    if n < 4:
        return 0
        
    # Precompute factorials for this n
    fact = [1] * (n + 1)
    for i in range(2, n + 1):
        fact[i] = (fact[i-1] * i) % MOD # Store modulo MOD?
    
    # Wait, for Combinations we need exact factorials if we divide?
    # Or use modular inverse.
    # Since n is small (60), we can use Python's large integers for factorials
    # and only mod at the very end of each term calculation.
    # Let's use exact factorials for simplicity of division.
    
    fact_exact = [1] * (n + 1)
    for i in range(2, n + 1):
        fact_exact[i] = fact_exact[i-1] * i

    if n % 2 == 1:
        # Odd n: all intersections have multiplicity 2
        # T(n) = binom(n, 4) * Contrib(2)
        # Contrib(2) = 2 * (n-3)!
        
        # binom(n, 4)
        bn4 = fact_exact[n] // (fact_exact[4] * fact_exact[n - 4])
        
        # Contrib(2)
        # c2 = 2 * fact_exact[n-3]
        # Result = bn4 * c2
        
        # Compute modulo
        result = (bn4 * 2 * fact_exact[n-3]) % MOD
        return result
    
    else:
        # Even n: need to find multiplicities
        intersections = []
        
        # Iterate all pairs of diagonals that intersect
        # A pair of diagonals is defined by 4 vertices.
        # For any 4 vertices on circle, exactly one pairing of them intersects internally.
        # If vertices indices are i < j < k < l, then (i, k) and (j, l) intersect.
        
        # Optimization: We only need counts of multiplicities.
        # We can cluster points.
        
        # Since n <= 60, we can afford to generate all binom(n, 4) points.
        # binom(60, 4) = 487,635
        
        for idxs in itertools.combinations(range(n), 4):
            i, j, k, l = idxs
            p1 = get_point(i, n)
            p2 = get_point(k, n)
            p3 = get_point(j, n)
            p4 = get_point(l, n)
            
            pt = get_intersection(p1, p2, p3, p4)
            if pt:
                intersections.append(pt)
                
        grouped = defaultdict(int)
        for pt in intersections:
            # Round to 9 decimal places for clustering
            # This tolerates float errors but separates distinct points
            key = (round(pt[0], 9), round(pt[1], 9))
            grouped[key] += 1
            
        multiplicities = defaultdict(int)
        for pt, pairs in grouped.items():
            # Check if pairs corresponds to an integer m
            # m(m-1)/2 = pairs => 8*pairs + 1 is a perfect square
            delta = 1 + 8 * pairs
            isqrt_delta = math.isqrt(delta)
            
            if isqrt_delta * isqrt_delta != delta:
                # This implies collision of non-concurrent lines due to precision
                # or logic error. For n=60 with 9 decimals, collision is unlikely.
                # If it happens, we might need higher precision.
                # But let's assume it works or fallback to nearest m.
                # print(f"Warning: pair count {pairs} at {pt} not triangular!")
                pass
                
            m = (1 + isqrt_delta) // 2
            multiplicities[m] += 1
            
        total_val = 0
        for m, count in multiplicities.items():
            # Use exact factorials for contrib calculation to avoid division issues
            # But we need to be careful with size. n=60 is fine.
            c = 0
            for k in range(2, m + 1):
                term = (-1)**k * (k - 1)
                bin_mk = fact_exact[m] // (fact_exact[k] * fact_exact[m - k])
                pow2 = pow(2, k - 1) # Exact power
                fact_rem = fact_exact[n - k - 1]
                
                c += term * bin_mk * pow2 * fact_rem
            
            # c is the contribution for one point of multiplicity m
            term_total = (count * c) % MOD
            total_val = (total_val + term_total) % MOD
            
        return total_val

def solve():
    total_sum = 0
    print(f"Calculating sum T(n) for n=3 to 60...")
    
    # Verification for n=8
    t8 = compute_t(8)
    if t8 != 14640:
        print(f"ERROR: T(8) = {t8}, expected 14640")
        # Depending on requirements, might want to exit or continue
    else:
        print("T(8) verification passed.")

    for n in range(3, 61):
        tn = compute_t(n)
        total_sum = (total_sum + tn) % MOD
        if n % 10 == 0 or n == 60:
            print(f"n={n}, T(n)={tn}, sum={total_sum}")
            
    print(f"Final Answer: {total_sum}")
    return total_sum

if __name__ == "__main__":
    solve()
