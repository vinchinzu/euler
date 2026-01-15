"""Project Euler Problem 170: Find the largest 0 to 9 pandigital that can be formed by concatenating products."""

from typing import List, Optional, Tuple

ALL_DIGITS_MASK = (1 << 10) - 1
POPCOUNT: List[int] = [bin(mask).count("1") for mask in range(1 << 10)]
DIGITS_DESC = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]


def digit_info(number: int) -> Optional[Tuple[int, int, str]]:
    """Get digit mask, length, and string representation."""
    if number <= 0:
        return None
    mask = 0
    length = 0
    n = number
    while n > 0:
        digit = n % 10
        bit = 1 << digit
        if (mask & bit) != 0:
            return None
        mask |= bit
        n //= 10
        length += 1
    return (mask, length, str(number))


def each_number_from_mask(
    mask: int, length: int
) -> List[Tuple[int, int]]:
    """Generate all numbers from mask with given length."""
    results: List[Tuple[int, int]] = []

    def build_number(
        target_len: int,
        current_value: int,
        used_mask: int,
        first_digit: bool,
    ) -> None:
        if target_len == 0:
            results.append((current_value, used_mask))
            return

        for digit in DIGITS_DESC:
            bit = 1 << digit
            if (mask & bit) == 0:
                continue
            if (used_mask & bit) != 0:
                continue
            if first_digit and target_len > 1 and digit == 0:
                continue

            new_value = current_value * 10 + digit
            build_number(target_len - 1, new_value, used_mask | bit, False)

    build_number(length, 0, 0, True)
    return results


def main() -> str:
    """Main function."""
    base_length = 2
    best_output: Optional[str] = None

    for base, base_mask in each_number_from_mask(ALL_DIGITS_MASK, base_length):
        if base == 0:
            continue

        remaining_mask = ALL_DIGITS_MASK ^ base_mask
        total_remaining_digits = POPCOUNT[remaining_mask]
        if total_remaining_digits != 10 - base_length:
            continue

        for len1 in range(1, total_remaining_digits):
            len2 = total_remaining_digits - len1

            expected_len1 = len1 + 1
            expected_len2 = len2 + 1

            for mult1, mask1 in each_number_from_mask(remaining_mask, len1):
                next_remaining = remaining_mask ^ mask1
                if POPCOUNT[next_remaining] != len2:
                    continue

                info1 = digit_info(base * mult1)
                if not info1:
                    continue
                mask_prod1, len_prod1, str_prod1 = info1
                if len_prod1 != expected_len1:
                    continue

                for mult2, _mask2 in each_number_from_mask(next_remaining, len2):
                    info2 = digit_info(base * mult2)
                    if not info2:
                        continue
                    mask_prod2, len_prod2, str_prod2 = info2
                    if len_prod2 != expected_len2:
                        continue
                    if (mask_prod1 & mask_prod2) != 0:
                        continue

                    combined_mask = mask_prod1 | mask_prod2
                    if combined_mask != ALL_DIGITS_MASK:
                        continue

                    candidate = str_prod1 + str_prod2
                    if best_output is None or candidate > best_output:
                        best_output = candidate

    return best_output or ""


if __name__ == "__main__":
    print(main())
