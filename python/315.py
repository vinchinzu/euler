"""Project Euler Problem 315 - Digital root clocks.

Sam's clock always clears the display between successive terms of the digital
root sequence, while Max's clock keeps common segments lit. The task is to feed
all prime numbers A <= p <= B (A = 10**7, B = 2 * 10**7) into both clocks and
return the total transitions Sam makes minus the total transitions Max makes.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, Iterable, Iterator, Tuple

# Segment mask per digit (7-bit encoding: top, top-left, top-right, middle,
# bottom-left, bottom-right, bottom). Stored as tuples for fast indexing.
TOP = 1 << 0
TOP_LEFT = 1 << 1
TOP_RIGHT = 1 << 2
MIDDLE = 1 << 3
BOTTOM_LEFT = 1 << 4
BOTTOM_RIGHT = 1 << 5
BOTTOM = 1 << 6

DIGIT_MASKS: Tuple[int, ...] = (
    TOP | TOP_LEFT | TOP_RIGHT | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM,  # 0
    TOP_RIGHT | BOTTOM_RIGHT,  # 1
    TOP | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM,  # 2
    TOP | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM,  # 3
    TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT,  # 4
    TOP | TOP_LEFT | MIDDLE | BOTTOM_RIGHT | BOTTOM,  # 5
    TOP | TOP_LEFT | MIDDLE | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM,  # 6
    TOP | TOP_LEFT | TOP_RIGHT | BOTTOM_RIGHT,  # 7
    TOP
    | TOP_LEFT
    | TOP_RIGHT
    | MIDDLE
    | BOTTOM_LEFT
    | BOTTOM_RIGHT
    | BOTTOM,  # 8
    TOP | TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM,  # 9
)

SEGMENT_COUNTS: Tuple[int, ...] = tuple(mask.bit_count() for mask in DIGIT_MASKS)
DIFF_COUNTS: Tuple[Tuple[int, ...], ...] = tuple(
    tuple((DIGIT_MASKS[a] ^ DIGIT_MASKS[b]).bit_count() for b in range(10))
    for a in range(10)
)

# Cache the (segment count, digit tuple, digit sum) triplet for small numbers.
SegmentInfo = Tuple[int, Tuple[int, ...], int]
SEGMENT_INFO_CACHE: Dict[int, SegmentInfo] = {
    d: (SEGMENT_COUNTS[d], (d,), d) for d in range(10)
}


def segment_info(value: int) -> SegmentInfo:
    """Return lit segments, digits (least-significant first), and digit sum."""

    cached = SEGMENT_INFO_CACHE.get(value)
    if cached is not None:
        return cached

    total_segments = 0
    digit_sum = 0
    digits: list[int] = []
    remaining = value

    while remaining:
        digit = remaining % 10
        digits.append(digit)
        total_segments += SEGMENT_COUNTS[digit]
        digit_sum += digit
        remaining //= 10

    info: SegmentInfo = (total_segments, tuple(digits), digit_sum)

    if value < 1000:
        SEGMENT_INFO_CACHE[value] = info

    return info


def transitions_between_digits(prev: Tuple[int, ...], curr: Tuple[int, ...]) -> int:
    """Return segment transitions between two displays (digits stored LSD first)."""

    shared = min(len(prev), len(curr))
    total = 0

    for idx in range(shared):
        total += DIFF_COUNTS[prev[idx]][curr[idx]]

    for idx in range(shared, len(prev)):
        total += SEGMENT_COUNTS[prev[idx]]

    for idx in range(shared, len(curr)):
        total += SEGMENT_COUNTS[curr[idx]]

    return total


def sam_minus_max_transitions(value: int) -> int:
    """Return Sam transitions minus Max transitions for the digital root chain."""

    segments, digits, next_value = segment_info(value)
    sam_total = 2 * segments
    max_total = segments
    prev_digits = digits
    prev_segments = segments
    current = value
    next_number = next_value

    while current >= 10:
        current = next_number
        segments, digits, next_number = segment_info(current)
        sam_total += 2 * segments
        max_total += transitions_between_digits(prev_digits, digits)
        prev_digits = digits
        prev_segments = segments

    max_total += prev_segments
    return sam_total - max_total


def base_primes(limit: int) -> Tuple[int, ...]:
    """Return all primes <= limit using a simple sieve."""

    if limit < 2:
        return ()

    sieve = bytearray(b"\x01") * (limit + 1)
    sieve[0:2] = b"\x00\x00"
    upper = isqrt(limit)

    for num in range(2, upper + 1):
        if sieve[num]:
            step = num
            start = num * num
            sieve[start : limit + 1 : step] = b"\x00" * (((limit - start) // step) + 1)

    return tuple(idx for idx in range(2, limit + 1) if sieve[idx])


def primes_in_range(start: int, stop: int) -> Iterator[int]:
    """Yield primes in the inclusive range [start, stop] via segmented sieve."""

    if stop < 2 or stop < start:
        return

    start = max(start, 2)
    root = isqrt(stop)
    primes = base_primes(root)
    segment_size = 1_000_000
    low = start

    while low <= stop:
        high = min(low + segment_size - 1, stop)
        segment = bytearray(b"\x01") * (high - low + 1)

        for prime in primes:
            multiple = max((low + prime - 1) // prime, prime) * prime
            if multiple > high:
                continue
            step = prime
            start_idx = multiple - low
            segment[start_idx : high - low + 1 : step] = b"\x00" * (
                ((high - multiple) // step) + 1
            )

        for offset, is_prime_flag in enumerate(segment):
            if is_prime_flag:
                yield low + offset

        low = high + 1


def solve() -> int:
    """Compute Sam's total minus Max's total for primes between 10^7 and 2*10^7."""

    lower = 10**7
    upper = 2 * 10**7
    return sum(sam_minus_max_transitions(prime) for prime in primes_in_range(lower, upper))


if __name__ == "__main__":
    print(solve())
