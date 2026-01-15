"""
Project Euler Problem 947: Fibonacci Sequence Periods

S(10^6) mod 999999893

# O(M * m^2) time for computing all periods
"""

MOD = 999999893


def compute_period(a, b, m):
    """Compute period of Fibonacci-like sequence (a,b,m).

    # O(m^2) time worst case, O(m) space
    """
    if m <= 0:
        raise ValueError("m must be positive")

    prev, current = a % m, b % m
    seen = {}
    step = 0

    while step < m * m + 1:
        state = (prev, current)
        if state in seen:
            return step - seen[state]
        seen[state] = step
        prev, current = current, (prev + current) % m
        step += 1

    raise RuntimeError(f"Cycle not found for ({a},{b},{m})")


def main():
    """Main function."""
    print("Project Euler Problem 947 - Fibonacci Periods")
    print("=" * 50)

    # Test with example from problem
    p = compute_period(0, 1, 8)
    print(f"p(0,1,8) = {p}, expected 12")

    if p == 12:
        print("Verification: PASS")
    else:
        print("Verification: FAIL")

    print("\nFull solution requires computing s(m) and S(M) for M=10^6")

    return 0


if __name__ == "__main__":
    main()
