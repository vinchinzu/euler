"""Project Euler Problem 494: Collatz prefix families.

A Collatz prefix is a Collatz sequence that ends right before a power of 2.
Find the number of distinct families of Collatz prefixes of N terms.

The base count is fibonacci(N). We then find special starting values where
the ordering of terms doesn't follow the standard pattern, and count
additional families from extending those special sequences backward.
"""

from __future__ import annotations
import subprocess
import os


def fibonacci(n):
    """Compute nth Fibonacci number (F_1=1, F_2=1, F_3=2, ...)."""
    a, b = 1, 1
    for _ in range(n - 1):
        a, b = b, a + b
    return a


def is_power_of_2(n):
    """Check if n is a power of 2."""
    return n > 0 and (n & (n - 1)) == 0


def solve():
    """Solve Problem 494."""
    N = 90
    L = 100000
    M = 3 ** 39

    ans = fibonacci(N)

    # Find special sequences.
    # In the Java code, the reference to the collatz list is stored,
    # meaning the FULL list (at the end of the loop) is what gets used.
    # A start is "special" if at any point n > start and r < 1.
    special_seqs = {}
    for start in range(1, L):
        collatz = []
        n = start
        r = 1.0
        is_special = False
        for i in range(N):
            collatz.append(n)
            if n % 2 == 0:
                n //= 2
                r /= 2
            else:
                n = 3 * n + 1
                r *= 3
            if is_power_of_2(n):
                break
            if n > start and r < 1:
                is_special = True
        if is_special:
            special_seqs[start] = collatz  # Store the full list

    # Remove redundant sequences
    non_redundant = {}
    for start, collatz in special_seqs.items():
        is_redundant = False
        for i in range(1, len(collatz)):
            if collatz[i] in special_seqs:
                is_redundant = True
                break
        if not is_redundant:
            non_redundant[start] = collatz

    # Compile C helper if needed
    script_dir = os.path.dirname(os.path.abspath(__file__))
    helper_c = os.path.join(script_dir, "494_helper.c")
    helper_bin = os.path.join(script_dir, "494_helper")
    if not os.path.exists(helper_bin) or os.path.getmtime(helper_c) > os.path.getmtime(helper_bin):
        subprocess.run(["gcc", "-O2", "-o", helper_bin, helper_c], check=True)

    # Count paths for each non-redundant special sequence using C helper
    for start, collatz in non_redundant.items():
        num_steps = N - len(collatz)
        result = subprocess.run(
            [helper_bin, str(start), str(num_steps), "0"],
            capture_output=True, text=True, check=True
        )
        ans += int(result.stdout.strip())

    return ans


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
