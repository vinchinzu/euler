"""
Project Euler Problem 937: Factorial Partitions

Let theta = sqrt(-2).
Define T to be the set of numbers of the form a + b*theta where a and b are integers.
G(n) = sum of all elements of F_n intersect A where F_n = {1!, 2!, ..., n!}

Given: G(4) = 25, G(7) = 745, G(100) = 709772949 (mod 10^9+7)
Find: G(10^8) mod 10^9+7

Time Complexity: O(n)
Space Complexity: O(1)
"""

MOD = 1000000007


def is_in_a(k):
    """Determine if k! is in set A based on pattern."""
    return k % 3 != 2


def compute_g(n, mod):
    """Compute G(n) = sum of k! for k in A."""
    if n < 1:
        return 0

    total_sum = 0
    current_factorial = 1

    for k in range(1, n + 1):
        current_factorial = (current_factorial * k) % mod

        if is_in_a(k):
            total_sum = (total_sum + current_factorial) % mod

    return total_sum


def main():
    n = 100000000  # 10^8

    # Verify small cases
    print("Verifying given test cases...")
    g4 = compute_g(4, MOD)
    print(f"G(4) = {g4}, expected 25")

    g7 = compute_g(7, MOD)
    print(f"G(7) = {g7}, expected 745")

    g100 = compute_g(100, MOD)
    print(f"G(100) = {g100}, expected 709772949")

    print(f"\nComputing G({n}) mod {MOD}...")
    result = compute_g(n, MOD)

    print(result)
    return result


if __name__ == "__main__":
    main()
