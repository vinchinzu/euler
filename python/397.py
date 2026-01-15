"""Efficient solution for Project Euler Problem 397 in Python 3.12.

This module provides a fast implementation of the helper sums and F(K, X)
function derived from the geometric formulation in the original problem.

Public API:
- compute_sums(k): preprocesses totients and divisor sums and returns
  (phi, sigma, s1, s2).
- compute_F(k, x): returns F(k, x) modulo 2**64.
- run_tests(): lightweight self-checks for small parameters.

The implementation avoids external dependencies and uses only the Python
standard library.
"""

from __future__ import annotations

from typing import List, Tuple

MOD: int = 2**64


def compute_sums(k: int) -> Tuple[List[int], List[int], int, int]:
    """Compute helper arrays and sums up to k.

    Returns (phi, sigma, s1, s2) where:
    - phi[n]  is Euler's totient function for 0 <= n <= k
    - sigma[n] is the sum of divisors of n for 0 <= n <= k
    - s1 = sum_{i=1..k} i * phi[i]
    - s2 = sum_{i=1..k} phi[i] * sigma[i]
    """
    if k < 1:
        raise ValueError("k must be positive integer")

    # Initialize phi as identity; phi[0] unused, phi[1] set explicitly.
    phi: List[int] = list(range(k + 1))
    sigma: List[int] = [0] * (k + 1)

    phi[1] = 1

    # Linear-style sieve for Euler's totient via prime multiples update.
    for i in range(2, k + 1):
        if phi[i] == i:  # i is prime (unchanged from initial value)
            step = i
            for j in range(i, k + 1, step):
                phi[j] = phi[j] // i * (i - 1)

    # Sum-of-divisors using a standard multiples loop.
    for i in range(1, k + 1):
        for j in range(i, k + 1, i):
            sigma[j] += i

    s1 = sum(i * phi[i] for i in range(1, k + 1))
    s2 = sum(phi[i] * sigma[i] for i in range(1, k + 1))

    return phi, sigma, s1, s2


def compute_F(k: int, x: int) -> int:
    """Compute F(k, x) modulo 2**64.

    Both k and x must be non-negative integers with k >= 1.
    """
    if k < 1 or x < 0:
        raise ValueError("k and x must be positive integers with x >= 0")

    _, _, s1, s2 = compute_sums(k)

    result = 2 * (s1 * (2 * x + 1) - s2)
    return result % MOD


def run_tests() -> bool:
    """Run basic correctness tests.

    This is a lightweight self-check mirroring the original Ruby tests.
    Returns True if all tests pass, False otherwise.
    """
    print("Running test suite...")

    result1 = compute_F(1, 10)
    print(f"F(1,10) = {result1} (expected: 40)")
    if result1 == 40:
        print("\u2713 Test 1 passed")
    else:
        print("\u2717 Test 1 failed")
        return False

    result2 = compute_F(10, 100)
    print(f"F(10,100) = {result2} (expected: 86570)")
    if result2 == 86570:
        print("\u2713 Test 2 passed")
    else:
        print("\u2717 Test 2 failed")
        return False

    result3 = compute_F(1, 0)
    print(f"F(1,0) = {result3} (expected: 0)")
    if result3 == 0:
        print("\u2713 Test 3 passed")
    else:
        print("\u2717 Test 3 failed")
        return False

    _, _, s1, s2 = compute_sums(2)
    print(f"S1(2) = {s1} (expected: 3)")
    print(f"S2(2) = {s2} (expected: 4)")
    if s1 == 3 and s2 == 4:
        print("\u2713 Test 4 passed")
    else:
        print("\u2717 Test 4 failed")
        return False

    phi, _, _, _ = compute_sums(10)
    expected_phi = [0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]
    phi_match = phi[1:11] == expected_phi[1:11]
    print(f"\u03c6[1..10] = {phi[1:11]} (expected: {expected_phi[1:11]})")
    if phi_match:
        print("\u2713 Test 5 passed")
    else:
        print("\u2717 Test 5 failed")
        return False

    print("All tests passed! \u2713")
    return True


def _main() -> None:
    """Execute self-tests and compute the target Project Euler answer."""
    if not run_tests():
        print("Tests failed. Aborting.")
        raise SystemExit(1)

    k = 10**6
    x = 10**9

    print(f"\nComputing F({k}, {x})...")
    print(f"This may take a few seconds for k={k}...")

    result = compute_F(k, x)
    print(f"\nF(10^6, 10^9) = {result}")


if __name__ == "__main__":  # pragma: no cover - manual execution entry point
    _main()
