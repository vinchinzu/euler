"""Project Euler Problem 169: Exploring the number of different ways a number can be expressed as a sum of powers of 2."""


def compute_f(n: int) -> int:
    """
    Compute f(n): number of ways to write n as sum of powers of 2, each used ≤2 times.

    Uses binary digit DP with carry tracking for efficiency with large n.
    """
    if n < 0 or not isinstance(n, int):
        raise ValueError("n must be a non-negative integer")

    if n == 0:
        return 1  # Base case: one way to make 0 (empty sum)

    # Extract binary digits (least significant bit first)
    bits: list[int] = []
    temp = n
    while temp > 0:
        bits.append(temp % 2)
        temp //= 2

    # DP state: ways[j][c] = number of ways to form first j bits with carry c
    # Initialize before processing any bits: 1 way with no carry
    prev_ways = [1, 0]  # [ways_no_carry, ways_with_carry]

    # Process each binary bit position from LSB to MSB
    for bit in bits:
        new_ways = [0, 0]  # Reset for next position

        # For each possible incoming carry (0 or 1)
        for carry_in_idx, ways in enumerate(prev_ways):
            # Try each possible coefficient d for current power of 2 (0, 1, or 2)
            for coeff in range(3):
                # Compute difference: coeff + carry_in - bit
                diff = coeff + carry_in_idx - bit

                # Valid transitions: diff must be 0 or 2 (allows carry_out = 0 or 1)
                if diff == 0:
                    carry_out = 0
                    new_ways[carry_out] += ways
                elif diff == 2:
                    carry_out = 1
                    new_ways[carry_out] += ways
                # diff == 1 or other values are invalid

        prev_ways = new_ways

    # After all bits, return ways with no final carry (must exactly match n)
    return prev_ways[0]


def main() -> int:
    """Main function."""
    target_n = 10 ** 25
    return compute_f(target_n)


if __name__ == "__main__":
    print(main())
