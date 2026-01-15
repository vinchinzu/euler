"""Project Euler Problem 162: Hexadecimal numbers."""

MIN_DIGIT = 0
MAX_DIGIT = 15
TOTAL_DIGITS = 16
NON_ZERO_FIRST_DIGITS = 15  # {1-9,A-F}
NO_ZERO_DIGITS = 15  # {1-9,A-F} (excluding 0)
MAX_DIGITS = 16


def sum_series(first_count: int, base: int, max_k: int) -> int:
    """Compute sum of geometric series."""
    if first_count <= 0 or base < 0 or max_k <= 0:
        return 0
    # Closed-form: first_count * (base^max_k - 1) / (base - 1) if base != 1
    if base == 1:
        return first_count * max_k
    else:
        return first_count * (base ** max_k - 1) // (base - 1)


def main() -> int:
    """Main function."""
    # Total valid hexadecimal numbers (1 to 16 digits, no leading zeros)
    total = sum_series(NON_ZERO_FIRST_DIGITS, TOTAL_DIGITS, MAX_DIGITS)

    # Inclusion-exclusion: Count numbers missing at least one of {0,1,A}
    # Set A: Missing 0
    missing_0 = sum_series(NO_ZERO_DIGITS, NO_ZERO_DIGITS, MAX_DIGITS)

    # Set B: Missing 1
    missing_1_first = 14  # {2-9,A-F}
    missing_1 = sum_series(missing_1_first, NO_ZERO_DIGITS, MAX_DIGITS)

    # Set C: Missing A
    missing_a_first = 14  # {1-9,B-F}
    missing_a = sum_series(missing_a_first, NO_ZERO_DIGITS, MAX_DIGITS)

    # Intersections: Missing two required digits
    # A ∩ B: Missing 0 and 1
    missing_0_1 = sum_series(14, 14, MAX_DIGITS)

    # A ∩ C: Missing 0 and A
    missing_0_a = sum_series(14, 14, MAX_DIGITS)

    # B ∩ C: Missing 1 and A
    missing_1_a_first = 13  # {2-9,B-F}
    missing_1_a = sum_series(missing_1_a_first, 14, MAX_DIGITS)

    # A ∩ B ∩ C: Missing 0, 1, and A
    missing_all = sum_series(13, 13, MAX_DIGITS)

    # Inclusion-exclusion principle
    missing_at_least_one = (
        missing_0
        + missing_1
        + missing_a
        - missing_0_1
        - missing_0_a
        - missing_1_a
        + missing_all
    )

    # Numbers containing all three digits
    result = total - missing_at_least_one
    return result


if __name__ == "__main__":
    result = main()
    print(f"{result:X}")
