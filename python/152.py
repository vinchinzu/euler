"""Project Euler Problem 152: Writing 1/2 as a sum of inverse squares."""

from fractions import Fraction
from typing import Dict, List


def main() -> int:
    """Count ways to write 1/2 as sum of distinct reciprocals of squares."""
    # Relevant candidates with prime factors 2,3,5,7,13
    candidates = [
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        12,
        13,
        14,
        15,
        16,
        18,
        20,
        21,
        24,
        27,
        28,
        30,
        32,
        35,
        36,
        39,
        40,
        42,
        45,
        48,
        52,
        54,
        56,
        60,
        63,
        64,
        65,
        70,
        72,
        80,
    ]

    target = Fraction(1, 2)
    threshold = 40

    # Precompute suffix sums for pruning
    remaining_suffix: List[Fraction] = [Fraction(0)] * (len(candidates) + 1)
    for i in range(len(candidates) - 1, -1, -1):
        remaining_suffix[i] = Fraction(1, candidates[i] ** 2) + remaining_suffix[
            i + 1
        ]

    # Identify large numbers (>= threshold)
    large = [x for x in candidates if x >= threshold]

    # Precompute all subset sums for large numbers
    last_sums: Dict[Fraction, int] = {}
    for mask in range(1 << len(large)):
        sum_r = Fraction(0)
        for j in range(len(large)):
            if (mask & (1 << j)) != 0:
                sum_r += Fraction(1, large[j] ** 2)
        last_sums[sum_r] = last_sums.get(sum_r, 0) + 1

    def search(
        next_idx: int,
        current: Fraction,
        candidates: List[int],
        remaining_suffix: List[Fraction],
        target: Fraction,
        threshold: int,
        last_sums: Dict[Fraction, int],
    ) -> int:
        """Recursive search function returning count."""
        if current == target:
            return 1
        if current > target or next_idx == len(candidates):
            return 0

        if current + remaining_suffix[next_idx] < target:
            return 0

        number = candidates[next_idx]
        if number >= threshold:
            diff = target - current
            return last_sums.get(diff, 0)

        res = 0
        # Without current number
        res += search(
            next_idx + 1,
            current,
            candidates,
            remaining_suffix,
            target,
            threshold,
            last_sums,
        )

        # With current number
        add = Fraction(1, number ** 2)
        if current + add <= target:
            res += search(
                next_idx + 1,
                current + add,
                candidates,
                remaining_suffix,
                target,
                threshold,
                last_sums,
            )

        return res

    count = search(0, Fraction(0), candidates, remaining_suffix, target, threshold, last_sums)
    return count


if __name__ == "__main__":
    print(main())
