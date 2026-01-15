"""Project Euler Problem 178: Step Numbers."""

MAX_LEN = 40
DIGIT_COUNT = 10
FULL_MASK = (1 << DIGIT_COUNT) - 1


def main() -> int:
    """Main function."""
    # dp[digit][mask] -> number of step numbers of current length
    current: List[List[int]] = [[0] * (1 << DIGIT_COUNT) for _ in range(DIGIT_COUNT)]

    for digit in range(1, 10):
        current[digit][1 << digit] = 1

    total = 0

    for step in range(MAX_LEN - 1):
        next_dp: List[List[int]] = [[0] * (1 << DIGIT_COUNT) for _ in range(DIGIT_COUNT)]
        for digit in range(10):
            mask_counts = current[digit]
            for mask in range(FULL_MASK + 1):
                count = mask_counts[mask]
                if count == 0:
                    continue

                if digit > 0:
                    next_digit = digit - 1
                    new_mask = mask | (1 << next_digit)
                    next_dp[next_digit][new_mask] += count

                if digit < 9:
                    next_digit = digit + 1
                    new_mask = mask | (1 << next_digit)
                    next_dp[next_digit][new_mask] += count

        current = next_dp

        # After step k, we have numbers of length k+2
        # We want to count numbers of length >= DIGIT_COUNT
        if step + 2 >= DIGIT_COUNT:
            for digit in range(10):
                total += current[digit][FULL_MASK]

    return total


if __name__ == "__main__":
    print(main())
