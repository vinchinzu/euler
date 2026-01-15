"""Project Euler Problem 164: Numbers for which no three consecutive digits have a sum greater than a given value."""

FIRST_DIGIT_RANGE = range(1, 10)
DIGIT_RANGE = range(10)
MAX_SUM = 9


def main() -> int:
    """Count valid n-digit numbers with no three consecutive digits summing > 9."""
    n = 20

    # Precompute all valid (d1, d2, d3) triples where d1 + d2 + d3 <= 9
    valid_triples: list[tuple[int, int, int]] = []
    for d1 in DIGIT_RANGE:
        for d2 in DIGIT_RANGE:
            for d3 in DIGIT_RANGE:
                if d1 + d2 + d3 <= MAX_SUM:
                    valid_triples.append((d1, d2, d3))

    # Edge cases
    if n < 1:
        return 0
    if n == 1:
        return len(FIRST_DIGIT_RANGE)  # 9 single-digit numbers (1-9)
    if n == 2:
        return 9 * 10  # 90 two-digit numbers (10-99)

    # Space-optimized DP: Use two 2D arrays (prev and current)
    prev_dp: list[list[int]] = [[0] * 10 for _ in range(10)]

    # Base case: Initialize for first two digits
    for first_digit in FIRST_DIGIT_RANGE:
        for second_digit in DIGIT_RANGE:
            prev_dp[first_digit][second_digit] = 1

    # Fill DP for positions 3 through n
    for pos in range(3, n + 1):
        current_dp: list[list[int]] = [[0] * 10 for _ in range(10)]

        for d1, d2, d3 in valid_triples:
            if prev_dp[d1][d2] > 0:
                current_dp[d2][d3] += prev_dp[d1][d2]

        prev_dp = current_dp

    # Sum all valid endings for the final position
    total = 0
    for d1 in DIGIT_RANGE:
        for d2 in DIGIT_RANGE:
            total += prev_dp[d1][d2]

    return total


if __name__ == "__main__":
    print(main())
