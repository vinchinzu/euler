"""
Project Euler Problem 848

Two players play a game. At the start of the game each player secretly chooses an
integer; the first player from 1,...,n and the second player from 1,...,m. Then
they take alternate turns, starting with the first player. The player, whose turn
it is, displays a set of numbers and the other player tells whether their secret
number is in the set or not. The player to correctly guess a set with a single
number is the winner and the game ends.

Let p(m,n) be the winning probability of the first player assuming both players
play optimally. For example p(1, n) = 1 and p(m, 1) = 1/m.

You are also given p(7,5) approx 0.51428571.

Find sum_{i=0}^{20} sum_{j=0}^{20} p(7^i, 5^j) and give your answer rounded to
8 digits after the decimal point.
"""

from __future__ import annotations
from fractions import Fraction

def get_Sn(n: int) -> int:
    """
    Computes S_n = n * D_n, where D_n is the coefficient in p(n, m) = 1 - D_n/m.
    Sequence S_n has constant second differences within ranges.
    """
    if n <= 0: return 0
    if n == 1: return 0
    if n == 2: return 1
    if n == 3: return 3
    
    # For n >= 4, find range index j
    # Range j is (3*2^j, 3*2^(j+1)]
    # j = floor(log2((n-1)/3))
    
    # Integer arithmetic for j
    val = (n - 1) // 3
    j = val.bit_length() - 1
    
    # S_n formula derived from pattern analysis:
    # S_{U_{j-1}} = 3 * 4^j
    # S_n = S_{start} + (n - start) * diff
    
    term1 = 3 * (4**j)
    
    start_of_range = 3 * (2**j) # U_{j-1}
    diff = 3 * (2**j)
    
    term2 = (n - start_of_range) * diff
    
    return term1 + term2

def get_Tn(n: int) -> int:
    """
    Returns the transition threshold T(n).
    If m <= T(n), p(m, n) follows Regime 2.
    If m >= T(n), p(m, n) follows Regime 1.
    """
    if n == 1: return 1
    if n == 2: return 2
    if n == 3: return 3
    
    val = (n - 1) // 3
    j = val.bit_length() - 1
    
    # T(n) is upper bound of range j: 3 * 2^(j+1)
    return 3 * (2**(j+1))

def solve() -> str:
    total_sum = Fraction(0, 1)
    
    # Precompute powers
    pow7 = [7**i for i in range(21)]
    pow5 = [5**j for j in range(21)]
    
    for m in pow7:
        S_m = get_Sn(m)
        
        for n in pow5:
            # Determine Regime
            Tn = get_Tn(n)
            
            if m <= Tn:
                # Regime 2: p(m, n) = 1 - D_m / n
                # D_m = S_m / m
                # p = 1 - S_m / (m*n)
                term = 1 - Fraction(S_m, m * n)
            else:
                # Regime 1: p(m, n) = C_n / m
                # C_n values determined from pattern
                if n == 1:
                    Cn = Fraction(1, 1)
                elif n == 2:
                    Cn = Fraction(3, 2)
                else:
                    S_n = get_Sn(n)
                    Cn = Fraction(2 * S_n, n)
                
                term = Cn / m
            
            total_sum += term
            
    # Convert to float and format
    val = float(total_sum)
    return f"{val:.8f}"

if __name__ == "__main__":
    print(solve())
