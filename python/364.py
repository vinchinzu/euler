"""Project Euler Problem 364: Comfortable Distance

N people sequentially fill N seats following a priority-based seating algorithm:
1. "If there is any seat whose adjacent seat(s) are not occupied take such a seat."
2. "If there is no such seat and there is any seat for which only one adjacent
   seat is occupied take such a seat."
3. "Otherwise take one of the remaining available seats."

Let T(N) represent the count of distinct filling sequences for N seats.
Known values: T(4) = 8, T(10) = 61,632, T(1,000) mod 100,000,007 = 47,255,094
Required: Calculate T(1,000,000) modulo 100,000,007.
"""

from __future__ import annotations

from functools import lru_cache

MOD: int = 100000007


def compute_T(n: int) -> int:
    """Compute T(n) modulo MOD using dynamic programming on seat configurations.

    We use bitmask DP where each state represents which seats are occupied.
    At each state, we determine which seats are valid choices based on priority
    rules, then sum up the ways to complete from each choice.

    Args:
        n: Number of seats

    Returns:
        T(n) mod MOD - the number of valid seating sequences
    """
    if not isinstance(n, int) or n < 0:
        raise ValueError("n must be a non-negative integer")

    if n == 0:
        return 1
    if n == 1:
        return 1

    # For large n, the bitmask approach won't work (2^n states)
    # We need to optimize for n <= ~20-25 with this approach
    if n > 25:
        # For larger n, we'd need a different algorithm
        # For now, return a placeholder indicating this needs optimization
        # TODO: Implement efficient recurrence or mathematical formula
        raise NotImplementedError(
            f"Current algorithm cannot handle n={n} (too large for bitmask DP). "
            f"Maximum practical n is around 25."
        )

    @lru_cache(maxsize=None)
    def dp(occupied_mask: int) -> int:
        """Count valid ways to complete seating from this configuration.

        Args:
            occupied_mask: Bitmask where bit i is 1 if seat i is occupied

        Returns:
            Number of valid completion sequences mod MOD
        """
        # Count occupied seats
        bits_set = bin(occupied_mask).count('1')

        if bits_set == n:
            return 1  # All seated, exactly one way to complete (done)

        # Classify empty seats by priority
        priority1 = []  # No occupied neighbors
        priority2 = []  # Exactly one occupied neighbor
        priority3 = []  # Two occupied neighbors

        for i in range(n):
            if (occupied_mask >> i) & 1:
                continue  # Seat i is occupied, skip

            left_occupied = (i > 0 and (occupied_mask >> (i - 1)) & 1)
            right_occupied = (i < n - 1 and (occupied_mask >> (i + 1)) & 1)
            neighbor_count = left_occupied + right_occupied

            if neighbor_count == 0:
                priority1.append(i)
            elif neighbor_count == 1:
                priority2.append(i)
            else:
                priority3.append(i)

        # Choose from highest non-empty priority level
        if priority1:
            valid_choices = priority1
        elif priority2:
            valid_choices = priority2
        else:
            valid_choices = priority3

        # Sum ways to complete from each valid choice
        total = 0
        for seat in valid_choices:
            new_mask = occupied_mask | (1 << seat)
            total = (total + dp(new_mask)) % MOD

        return total

    return dp(0)


def verify_solution() -> None:
    """Run built-in checks against known values from the problem statement."""
    print("Verifying solution...")

    test_cases = [
        (1, 1),
        (4, 8),
        (10, 61632),
    ]

    all_passed = True
    for n, expected in test_cases:
        result = compute_T(n)
        expected_mod = expected % MOD
        status = "✓ PASS" if result == expected_mod else "✗ FAIL"
        print(f"T({n}) = {result} (expected {expected_mod}) {status}")
        if result != expected_mod:
            all_passed = False

    # Test T(1000) if practical
    try:
        print("\nNote: T(1000) requires algorithmic optimization beyond bitmask DP")
        print("Current implementation cannot handle n > 25")
    except Exception as e:
        print(f"Error testing T(1000): {e}")
        all_passed = False

    if all_passed:
        print("\nAll verifications PASSED!")
    else:
        print("\nSome verifications FAILED")


def main() -> None:
    """CLI entry point - prints only the final numeric answer."""
    import sys

    n: int
    if len(sys.argv) > 1:
        try:
            arg = int(sys.argv[1])
            n = arg if arg > 0 else 1_000_000
        except ValueError:
            n = 1_000_000
    else:
        n = 1_000_000

    try:
        result = compute_T(n)
        print(result)
    except NotImplementedError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
