"""Project Euler Problem 156: Counting Digits."""

from typing import Callable


def count_single(d: int, v: int) -> int:
    """Count occurrences of digit d in number v."""
    if v == 0 and d == 0:
        return 1
    res = 0
    while v > 0:
        if (v % 10) == d:
            res += 1
        v //= 10
    return res


def count_digit(d: int, v: int) -> int:
    """Count occurrences of digit d in numbers from 0 to v."""
    base = 10
    if v < 0:
        return 0
    if v < base:
        return 1 if v >= d else 0
    shift = 1
    multi = 0
    while shift * base <= v:
        shift *= base
        multi += 1
    multi *= shift // base
    first = v // shift
    rem = v % shift
    res = first * multi + count_digit(d, rem)
    if d == first:
        res += rem + 1
    if d < first and d > 0:
        res += shift
    return res


def find_all(d: int, fr: int, to_n: int) -> int:
    """Find all fixed points where count_digit(d, n) == n in range [fr, to_n]."""
    center = (fr + to_n) // 2
    if fr == center:
        return fr if count_digit(d, fr) == fr else 0
    result = 0
    count_fr = count_digit(d, fr)
    cur_fr = fr
    cur_count = count_fr
    while cur_count == cur_fr and cur_fr < to_n:
        result += cur_fr
        cur_fr += 1
        cur_count += count_single(d, cur_fr)
    if cur_fr >= to_n + 1:
        return result
    fr = cur_fr
    count_fr = cur_count
    center = (fr + to_n) // 2
    count_center = count_digit(d, center)
    count_to = count_digit(d, to_n)
    if count_center >= fr and center >= count_fr and center > fr:
        result += find_all(d, fr, center)
    if count_to >= center and to_n >= count_center and center < to_n:
        result += find_all(d, center, to_n)
    return result


def main() -> int:
    """Main function."""
    max_n = 10 ** 12
    total = 0
    for d in range(1, 10):
        total += find_all(d, 0, max_n)
    return total


if __name__ == "__main__":
    print(main())
