#!/usr/bin/env python3
"""
Project Euler Problem 48: Self powers

Find the last ten digits of the series 1^1 + 2^2 + ... + 1000^1000.
"""

from math import prod


def main():
    MOD = 10 ** 10
    total = sum(pow(i, i, MOD) for i in range(1, 1001)) % MOD
    print(total)


if __name__ == "__main__":
    main()
