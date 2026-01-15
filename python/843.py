# Project Euler Problem 843
#
# PROBLEM DESCRIPTION:
# This problem involves an iterative procedure that begins with a circle of n≥3 integers.
# At each step every number is simultaneously replaced with the absolute difference of its two neighbours.
#
# For any initial values, the procedure eventually becomes periodic.
#
# Let S(N) be the sum of all possible periods for 3≤n≤N. For example, S(6) = 6, because the possible periods
# for 3≤n≤6 are 1, 2, 3. Specifically, n=3 and n=4 can each have period 1 only, while n=5 can have period 1 or 3,
# and n=6 can have period 1 or 2.
#
# You are also given S(30) = 20381.
#
# Find S(100).

import sys
import math
import itertools
from sympy.polys.rings import ring
from sympy.polys.domains import GF
from sympy.polys.polytools import factor_list
from sympy.polys.specialpolys import cyclotomic_poly
from sympy.ntheory import divisors, factorint
from sympy import Poly, symbols

#wrong: 2816758647348

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
        f_pure = flist[0][0]
        f = R.from_list(f_pure.all_coeffs())
        
        exponent = 2**s
        mod_poly = f**exponent
        
        base_elem = P % mod_poly
        
        if base_elem == R.zero:
            component_sets.append({1})
            continue
            
        p_set = get_component_periods(base_elem, mod_poly, f, R)
        component_sets.append(p_set)
        
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
