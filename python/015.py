#!/usr/bin/env python3
"""
Project Euler Problem 15: Lattice paths

Starting in the top left corner of a 20×20 grid, and only being able to move right or down,
how many routes are there to the bottom right corner?
"""

from math import comb


def main():
    # 20x20 grid means we need 40 moves total (20 right, 20 down)
    # C(40, 20) = 40! / (20! * 20!)
    result = comb(40, 20)
    print(result)


if __name__ == "__main__":
    main()
