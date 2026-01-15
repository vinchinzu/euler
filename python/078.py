#!/usr/bin/env python3
"""
Project Euler Problem 78: Coin Partitions

Let p(n) represent the number of different ways in which n coins can be
separated into two piles. Find the least n for which p(n) is divisible by one million.
"""

TARGET_DIVISOR = 1_000_000

def pentagonal(n):
    """Calculate pentagonal number: P(n) = n(3n-1)/2"""
    return n * (3 * n - 1) // 2

def main():
    """
    Use Euler's Pentagonal Number Theorem to calculate p(n), number of partitions of n.
    The recurrence relation is:
    p(n) = Σ(-1)^(k-1) * [p(n - g(k)) + p(n - g(-k))]
    where g(k) = k(3k-1)/2 are generalized pentagonal numbers
    """
    p = [1]  # p(0) = 1
    n = 0

    while True:
        n += 1
        p_n = 0
        k = 1

        while True:
            # Generalized pentagonal numbers: g(k) = k(3k-1)/2
            g_pos = k * (3 * k - 1) // 2
            g_neg = k * (3 * k + 1) // 2  # g(-k)

            if g_pos > n:
                break

            # Add p(n - g(k)) with sign (-1)^(k-1)
            sign = 1 if (k - 1) % 2 == 0 else -1
            if g_pos <= n:
                p_n += sign * p[n - g_pos]

            # Add p(n - g(-k)) with sign (-1)^(k-1)
            if g_neg <= n:
                p_n += sign * p[n - g_neg]

            k += 1

        # Store modulo TARGET_DIVISOR to avoid overflow
        p_n_mod = p_n % TARGET_DIVISOR
        p.append(p_n_mod)

        if p_n_mod == 0:
            print(n)
            return


if __name__ == "__main__":
    main()
