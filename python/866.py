"""
Project Euler Problem 866
=========================

A small child has a “number caterpillar” consisting of N jigsaw pieces, each with one number on it, which, when connected together in a line, reveal the numbers 1 to N in order.

Every night, the child's father has to pick up the pieces of the caterpillar that have been scattered across the play room. He picks up the pieces of the caterpillar that have been scattered across the play room. He picks up the pieces at random and places them in the correct order.
As the caterpillar is built up in this way, it forms distinct segments that gradually merge together.

Any time the father places a new piece in its correct position, a segment of length k is formed and he writes down the kth hexagonal number k*(2k-1). Once all pieces have been placed and the full caterpillar constructed he calculates the product of all the numbers written down. Interestingly, the expected value of this product is always an integer. For example if N=4 then the expected value is 994.

Find the expected value of the product for a caterpillar of N=100 pieces.
Give your answer modulo 987654319.
"""

from __future__ import annotations

MOD = 987654319

def hexagonal(k: int) -> int:
    """Returns the kth hexagonal number."""
    return k * (2 * k - 1)

def solve_for_n(n: int) -> int:
    """
    Computes the expected value of the product for a caterpillar of n pieces.
    
    Recurrence relation:
    E[n] = (H_n / n) * sum(E[i] * E[n-1-i] for i in 0..n-1)
    where H_n is the nth hexagonal number.
    E[0] = 1.
    """
    if n == 0:
        return 1
        
    E = [0] * (n + 1)
    E[0] = 1
    
    for k in range(1, n + 1):
        # Calculate sum(E[i] * E[k-1-i])
        # Since convolution is symmetric, we can optimize, but for N=100 it's not needed.
        sum_val = 0
        for i in range(k):
            term = (E[i] * E[k - 1 - i]) % MOD
            sum_val = (sum_val + term) % MOD
            
        # H_k = k * (2k - 1)
        # E[k] = H_k * (1/k) * sum_val
        
        # We can simplify: k * (2k - 1) / k = 2k - 1
        # So E[k] = (2k - 1) * sum_val
        
        factor = (2 * k - 1) % MOD
        E[k] = (factor * sum_val) % MOD
        
    return E[n]

def solve() -> int:
    return solve_for_n(100)

if __name__ == "__main__":
    print(solve())
