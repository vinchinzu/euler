#!/usr/bin/env python3
"""
Project Euler Problem 76: Counting Summations

How many different ways can one hundred be written as a sum of at least two positive integers?
"""

TARGET_SUM = 100

def main():
    """
    Counting number of ways to make TARGET_SUM (100) as sum of at least two
    positive integers.
    Uses dynamic programming.
    """
    # ways[i] stores number of ways to make sum 'i' using allowed parts
    # Allowed parts: positive integers from 1 to TARGET_SUM - 1 (1 to 99)
    # ways[0] = 1 (one way to make sum 0: use no parts)
    ways = [0] * (TARGET_SUM + 1)
    ways[0] = 1

    # Loop through each part that can be used in the sum
    for part in range(1, TARGET_SUM):
        # Update ways to make sums from 'part' up to TARGET_SUM
        for current_sum in range(part, TARGET_SUM + 1):
            # The number of ways to make 'current_sum' can be increased
            # by using the current 'part'
            ways[current_sum] += ways[current_sum - part]

    # The answer is the number of ways to make TARGET_SUM using parts up to TARGET_SUM - 1
    # This directly counts partitions into at least two parts (which is what we want)
    result = ways[TARGET_SUM]

    print(result)


if __name__ == "__main__":
    main()
