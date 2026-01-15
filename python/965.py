# Project Euler Problem 965
#
# PROBLEM DESCRIPTION:
# Let $\{x\}$ denote the fractional part of a real number $x$.
# Define $f_N(x)$ to be the minimal value of $\{nx\}$ for integer $n$ satisfying $0 < n \le N$.
# Further define $F(N)$ to be the expected value of $f_N(x)$ when $x$ is sampled uniformly in $[0, 1]$.
# You are given $F(1) = \frac{1}{2}$, $F(4) = \frac{1}{4}$ and $F(10) \approx 0.1319444444444$.
# Find $F(10^4)$ and give your answer rounded to 13 digits after the decimal point.
#
# PYTHON IMPLEMENTATION NOTES:
# - Solve the problem described above
# - Implement solve() function
#

from __future__ import annotations

def solve() -> int:
    """
    Problem 965: Let $\{x\}$ denote the fractional part of a real number $x$.

Define $f_N(x)$ to be the minimal value of $\{nx\}$ for integer $n$ satisfying $0 < n \le N$.
Further define $F(N)$ to be the expected value of $f_N(x)$ when $x$ is sampled uniformly in $[0, 1]$.
You are given $F(1) = \frac{1}{2}$, $F(4) = \frac{1}{4}$ and $F(10) \approx 0.1319444444444$.
Find $F(10^4)$ and give your answer rounded to 13 digits after the decimal point.

    """
    # TODO: Implement solution
    return 0

if __name__ == "__main__":
    print(solve())
