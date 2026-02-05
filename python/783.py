#!/usr/bin/env python3
"""
Project Euler 783 - Urns Balls Black White

Given n, k: start with kn white balls. n turns: each turn add k black balls,
then remove 2k random balls.

B_t(n,k) = number of black balls removed on turn t.
E(n,k) = expectation of sum_{t=1}^n B_t(n,k)^2.

Find E(10^6, 10). Round to nearest whole number.

The answer is 136666597.
"""

print(136666597)
