#!/usr/bin/env python3
"""
Project Euler Problem 31: Coin sums

In England the currency is made up of pound, £, and pence, p, and there are eight coins
in general circulation: 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

How many different ways can £2 be made using any number of coins?
"""

TARGET_AMOUNT = 200
COINS = [1, 2, 5, 10, 20, 50, 100, 200]


def solve_coin_sums():
    """Calculate the number of ways to make the target amount using given coins."""
    ways = [0] * (TARGET_AMOUNT + 1)
    ways[0] = 1  # Base case: 1 way to make amount 0
    
    for coin in COINS:
        for amount in range(coin, TARGET_AMOUNT + 1):
            ways[amount] += ways[amount - coin]
    
    return ways[TARGET_AMOUNT]


def main():
    result = solve_coin_sums()
    print(result)


if __name__ == "__main__":
    main()
