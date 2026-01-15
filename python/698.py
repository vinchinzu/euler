"""Project Euler Problem 698: 123 Numbers.

If 1 is the smallest 123-number, and the 123-numbers are those that when
written in base 10, contain only the digits "1", "2", "3", the frequency of
each digit also being a 123-number. Find the Nth 123-number.
"""

from __future__ import annotations

from itertools import product


def multinomial_coefficient(counts: list[int]) -> int:
    """Multinomial coefficient."""
    total = sum(counts)
    result = 1
    for i in range(1, total + 1):
        result *= i
    for count in counts:
        for i in range(1, count + 1):
            result //= i
    return result


def solve() -> int:
    """Solve Problem 698."""
    N = 111_111_111_111_222_333
    K = 3
    NUMS = [0, 1, 2, 3, 11, 12, 13, 21, 22, 23, 31, 32, 33]
    M = 123123123

    length = 0
    limit = N

    # Find the length of the Nth number
    while True:
        total_count = 0
        for counts in product(NUMS, repeat=K):
            if sum(counts) == length:
                total_count += multinomial_coefficient(list(counts))
        if total_count > limit:
            break
        limit -= total_count
        length += 1

    # Build the number digit by digit
    sb = []
    for i in range(length):
        for d in range(K):
            total_count = 0
            for counts in product(NUMS, repeat=K):
                if sum(counts) == length:
                    counts_arr = list(counts)
                    # Check if current prefix is valid
                    valid = True
                    for c in sb:
                        idx = int(c) - 1
                        if idx < 0 or idx >= len(counts_arr):
                            valid = False
                            break
                        counts_arr[idx] -= 1
                        if counts_arr[idx] < 0:
                            valid = False
                            break
                    if not valid:
                        continue
                    # Check if adding d is valid
                    if d < len(counts_arr):
                        counts_arr[d] -= 1
                        if counts_arr[d] < 0:
                            continue
                    total_count += multinomial_coefficient(counts_arr)

            if total_count > limit:
                sb.append(str(d + 1))
                break
            limit -= total_count

    result = int(''.join(sb))
    return result % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
