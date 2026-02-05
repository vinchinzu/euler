#!/usr/bin/env python3
"""Tests for Project Euler Problem 823."""

import sys
import os
import importlib.util

# Load the solution module
spec = importlib.util.spec_from_file_location(
    "solution_823",
    os.path.join(os.path.dirname(__file__), "823.py")
)
solution_module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(solution_module)


def compute_S_naive(n: int, m: int) -> int:
    """
    Compute S(n, m) naively by simulating the process.
    Only works for small values of n and m.
    """
    # Start with list [2, 3, ..., n]
    numbers = list(range(2, n + 1))

    def spf(num):
        """Compute smallest prime factor of num."""
        if num <= 1:
            return num
        for i in range(2, int(num**0.5) + 1):
            if num % i == 0:
                return i
        return num

    # Perform m rounds
    for _ in range(m):
        new_numbers = []
        product = 1

        for num in numbers:
            if num > 1:
                # Divide by smallest prime factor
                p = spf(num)
                product *= p
                new_num = num // p
                if new_num > 1:
                    new_numbers.append(new_num)

        # Add product to list
        new_numbers.append(product)
        numbers = new_numbers

    return sum(numbers)


def test_example_5_3():
    """Test S(5, 3) = 21."""
    result = compute_S_naive(5, 3)
    assert result == 21, f"Expected 21, got {result}"
    print(f"Test S(5, 3) = 21: PASSED")


def test_example_10_100():
    """Test S(10, 100) = 257."""
    result = compute_S_naive(10, 100)
    assert result == 257, f"Expected 257, got {result}"
    print(f"Test S(10, 100) = 257: PASSED")


def test_final_answer():
    """Test that the solution produces the correct answer."""
    result = solution_module.solve()
    expected = 865849519
    assert result == expected, f"Expected {expected}, got {result}"
    print(f"Test S(10^4, 10^16) mod 1234567891 = 865849519: PASSED")


if __name__ == "__main__":
    print("Running tests for Problem 823...")
    test_example_5_3()
    test_example_10_100()
    print("\nAll example tests passed!")
    print("\nTesting final solution...")
    test_final_answer()
    print("\nAll tests passed!")
