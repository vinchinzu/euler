#!/usr/bin/env python3
"""
Project Euler Problem 25: 1000-digit Fibonacci number

The Fibonacci sequence is defined by the recurrence relation:
F_n = F_{n-1} + F_{n-2}, where F_1 = 1 and F_2 = 1.

The 12th term, F_12, is the first term to contain three digits.
What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
"""

from math import log10, sqrt, ceil


def main():
    # Using the approximation: F_n ≈ phi^n / sqrt(5)
    # For F_n to have 1000 digits: 10^999 ≤ F_n < 10^1000
    # log10(F_n) ≈ n * log10(phi) - log10(sqrt(5)) ≥ 999
    # n ≥ (999 + log10(sqrt(5))) / log10(phi)
    
    phi = (1 + sqrt(5)) / 2
    
    n = ceil((999 + log10(sqrt(5))) / log10(phi))
    print(n)


if __name__ == "__main__":
    main()
