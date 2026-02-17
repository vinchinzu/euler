# Project Euler Problem 843 - Periodic Circles
#
# PROBLEM: Find the sum of all possible periods for circles of size 3 to 100
# in the absolute difference iteration process.
#
# MATHEMATICAL APPROACH:
# The iteration corresponds to multiplication by (1+x) in GF(2)[x]/(x^n - 1).
# The ring decomposes via Chinese Remainder Theorem as:
#   GF(2)[x]/(x^n - 1) ≅ ∏_{d|n_odd} GF(2)[x]/(Phi_d(x)^{2^s})
# where n = 2^s * n_odd.
#
# CRITICAL BUG FIX (2024-02-16):
# When Phi_d factors into multiple irreducible polynomials over GF(2), 
# x + x^(n-1) can have DIFFERENT periods in different factors. We must:
# 1. Factor Phi_d into irreducible polynomials over GF(2)
# 2. Compute periods from EACH factor separately
# 3. Take LCMs of periods across all factors for the same d
#
# Example for n=51, d=17: Phi_17 has 2 factors over GF(2)
#   - Factor 1: period of x + x^50 is 15
#   - Factor 2: period of x + x^50 is 5
# Taking only factor 1 misses period 5, causing wrong answer.
#
# Missing periods: 5, 9, 10, 20, 85, 16777215
# Wrong answer:  2816758647348
# Right answer:  2816775424692
# Difference:    16777344 = 2^24

import sys
import math
import itertools
from sympy.polys.rings import ring
from sympy.polys.domains import GF
from sympy.polys.polytools import factor_list
from sympy.polys.specialpolys import cyclotomic_poly
from sympy.ntheory import divisors, factorint
from sympy import Poly, symbols

def poly_pow_mod(base, exp, mod):
    res = base.ring.one
    curr = base
    while exp > 0:
        if exp % 2 == 1:
            res = (res * curr) % mod
        curr = (curr * curr) % mod
        exp //= 2
    return res

def get_component_periods(elem, mod_poly, f, R):
    d_f = f.degree()
    M = 2**d_f - 1
    
    prime_factors = factorint(M)
    m = M
    for p, e in prime_factors.items():
        for _ in range(e):
            if poly_pow_mod(elem, m // p, f) == R.one:
                m //= p
            else:
                break
    
    periods = {1}
    curr = m
    periods.add(curr)
    
    while poly_pow_mod(elem, curr, mod_poly) != R.one:
        curr *= 2
        periods.add(curr)
        
    return periods

def solve_n(n):
    n_odd = n
    s = 0
    while n_odd % 2 == 0:
        n_odd //= 2
        s += 1
        
    R, x = ring('x', GF(2))
    P = x + x**(n-1)
    xs = symbols('x')
    
    component_sets = []
    
    for d in divisors(n_odd):
        if d == 1:
            component_sets.append({1})
            continue
            
        expr = cyclotomic_poly(d, xs)
        phi_poly = Poly(expr, xs, domain=GF(2))
        flist = factor_list(phi_poly)[1]
        
        # FIXED: Get periods from ALL irreducible factors, not just the first
        # When Phi_d factors into multiple polynomials over GF(2), each factor
        # corresponds to a different embedding of the field. The element 
        # x + x^(n-1) can have different periods in different embeddings, so
        # we need to take LCMs of periods across all factors.
        factor_period_sets = []
        for f_pure, _ in flist:
            f = R.from_list(f_pure.all_coeffs())
            exponent = 2**s
            mod_poly = f**exponent
            
            base_elem = P % mod_poly
            
            if base_elem == R.zero:
                factor_period_sets.append({1})
            else:
                p_set = get_component_periods(base_elem, mod_poly, f, R)
                factor_period_sets.append(p_set)
        
        # Take LCMs of periods from all factors for this divisor
        d_periods = set()
        for p_tuple in itertools.product(*factor_period_sets):
            val = 1
            for p in p_tuple:
                val = math.lcm(val, p)
            d_periods.add(val)
        
        component_sets.append(d_periods)
        
    valid_periods = set()
    for p_tuple in itertools.product(*component_sets):
        val = 1
        for p in p_tuple:
            val = math.lcm(val, p)
        valid_periods.add(val)
        
    return valid_periods

def solve() -> int:
    full_set = set()
    for n in range(3, 101):
        pers = solve_n(n)
        full_set.update(pers)
        
    return sum(full_set)

if __name__ == "__main__":
    print(solve())
