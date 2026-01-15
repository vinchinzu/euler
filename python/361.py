"""Project Euler Problem 361 - Thue-Morse based sequence A_n.

This module is a Python 3.12 translation of a Ruby script exploring Problem 361.

Key ideas:
- Generate (a prefix of) the Thue-Morse sequence.
- Define A_n as the sorted sequence of non-negative integers whose binary
  representation appears as a contiguous subsequence of the Thue-Morse sequence.
- Provide utilities to:
  - Generate Thue-Morse iteratively.
  - Test whether a number's binary representation appears in a given prefix.
  - Compute or approximate the n-th valid number A_n.
  - Run small verification experiments and a demonstration summation.

Note:
The original Ruby script contains a heuristic `find_nth_valid_large` used for
large indices. This translation keeps the heuristic and documents its
limitations. The implementation is intentionally self-contained and uses only
Python's standard library.
"""

from __future__ import annotations

from typing import List, Set
import math


def generate_thue_morse_iterative(length: int) -> List[int]:
    """Generate the first ``length`` terms of the Thue-Morse sequence.

    The Thue-Morse sequence {T_n} is defined by:
    - T_0 = 0
    - T_{2n} = T_n
    - T_{2n + 1} = 1 - T_n

    This implementation uses an iterative doubling strategy roughly following
    the Ruby version's structure while remaining simple and explicit.
    """

    if length <= 0:
        return []

    t: List[int] = [0]
    current_length = 1

    while current_length < length:
        new_length = current_length * 2
        cap = min(new_length, length)
        new_t = [0] * cap

        for i in range(current_length):
            idx0 = i * 2
            idx1 = idx0 + 1
            if idx0 >= cap:
                break
            if t[i] == 0:
                new_t[idx0] = 0
                if idx1 < cap:
                    new_t[idx1] = 1
            else:
                new_t[idx0] = 1
                if idx1 < cap:
                    new_t[idx1] = 0

        t = new_t
        current_length = cap

    return t


def appears_as_substring(num: int, t: List[int]) -> bool:
    """Return True if ``num``'s binary representation appears in sequence ``t``.

    A value 0 is considered valid by definition.

    This is a direct translation of the Ruby version, using a string-based
    search. It is intentionally straightforward rather than optimal.
    """

    if num == 0:
        return True

    bin_str = format(num, "b")
    t_str = "".join(str(bit) for bit in t)
    return bin_str in t_str


def _count_valid_in_range(
    start: int,
    end: int,
    t: List[int],
) -> int:
    """Count numbers in [start, end] whose binary appears in ``t``.

    Internal helper mirroring the original brute-force counting behavior.
    """

    count = 0
    for num in range(start, end + 1):
        if appears_as_substring(num, t):
            count += 1
    return count


def find_nth_valid(n: int, max_t_length: int = 10_000) -> int:
    """Compute the n-th element A_n via brute force search.

    The method:
    - Builds a prefix of the Thue-Morse sequence of length ``max_t_length``.
    - Uses exponential search plus verification to locate a range containing
      at least ``n + 1`` valid numbers (including 0).
    - Collects and sorts all valid numbers in that range and returns A_n.

    This is a direct, non-optimized translation of the Ruby logic. It is
    practical only for relatively small ``n`` and relies on having a sufficiently
    long Thue-Morse prefix. If ``max_t_length`` is too small, the search may
    fail and raise a RuntimeError.
    """

    if n < 0:
        raise ValueError("n must be non-negative")

    t = generate_thue_morse_iterative(max_t_length)
    # ``t_str`` in the original is unused during search aside from substring
    # checks inside appears_as_substring; we keep logic aligned but do not
    # pre-convert here to avoid unnecessary memory.

    valid_numbers: Set[int] = {0}
    current_max = 0
    target_count = n + 1  # include 0

    low = 0
    high = 1

    # Exponential search for an upper bound that can contain enough valid nums.
    while len(valid_numbers) < target_count:
        high *= 2
        mid = (low + high) // 2

        temp_count = _count_valid_in_range(current_max + 1, mid, t)

        if temp_count + len(valid_numbers) >= target_count:
            high = mid
        else:
            low = mid

        # Safety: if range is exhausted without progress, break to avoid loops.
        if mid <= current_max + 1 and temp_count == 0:
            break

    final_max = high

    for num in range(current_max + 1, final_max + 1):
        if appears_as_substring(num, t):
            valid_numbers.add(num)
            if len(valid_numbers) == target_count:
                break

    sorted_valid = sorted(valid_numbers)
    if len(sorted_valid) > n:
        return sorted_valid[n]

    raise RuntimeError(
        f"Could not find {n}-th valid number within range up to {final_max}"
    )


def find_nth_valid_large(n: int) -> int:
    """Return a heuristic approximation for A_n when n is large.

    The original Ruby script uses ``exp(0.5 * sqrt(n))`` as a very rough
    approximation for large ``n``. This is a placeholder and not a proven
    formula for Project Euler 361. It exists solely to emulate the behavior
    of the Ruby script for demonstration runs.

    TODO: Replace this with a mathematically justified and efficient method
    for computing A_n for large indices.
    """

    if n < 0:
        raise ValueError("n must be non-negative")

    if n < 1000:
        return find_nth_valid(n)

    approx = int(math.exp(0.5 * math.sqrt(n)))
    return approx


def verify_small_cases() -> None:
    """Run simple sanity checks mirroring the Ruby script's verification.

    This prints:
    - A check of the first 20 Thue-Morse terms.
    - Tests for specific numbers appearing or not appearing as substrings.
    - The first few A_n values as computed by ``find_nth_valid``.
    """

    print("=== Verification of Small Cases ===")

    t = generate_thue_morse_iterative(20)
    expected = [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1]
    if t == expected:
        print("\u2713 Thue-Morse generation correct")
    else:
        print("\u2717 Thue-Morse generation failed")
        print(f"Expected: {expected}")
        print(f"Got:      {t}")

    test_cases = [
        (0, True),   # binary: 0
        (1, True),   # binary: 1
        (2, True),   # binary: 10
        (3, True),   # binary: 11
        (4, True),   # binary: 100
        (5, True),   # binary: 101
        (6, True),   # binary: 110
        (7, False),  # binary: 111
        (9, True),   # binary: 1001
        (14, False), # binary: 1110
        (18, True),  # binary: 10010
    ]

    t = generate_thue_morse_iterative(100)
    for num, expected_ok in test_cases:
        result = appears_as_substring(num, t)
        status = "\u2713" if result == expected_ok else "\u2717"
        print(f"{status} {num:3d} ({num:5b}) = {result}")

    print("\nFirst 15 A_n values (using naive search):")
    for i in range(15):
        try:
            a_i = find_nth_valid(i)
            print(f"A_{i:2d} = {a_i:4d} (binary {a_i:b})")
        except RuntimeError as exc:  # pragma: no cover - diagnostic only
            print(f"A_{i:2d}: error: {exc}")


def compute_sum_last_9_digits(max_exact_k: int = 6) -> int:
    """Compute the last 9 digits of sum(A_{10^k}) for k = 1..18.

    This mirrors the Ruby script's main logic:
    - For small k (up to ``max_exact_k``), attempts exact computation using
      ``find_nth_valid``. On failure, it falls back to the heuristic.
    - For larger k, uses the heuristic approximation directly.

    Note:
    This function reproduces the behavior of the original script, including the
    use of the heuristic and is not guaranteed to produce the official PE361
    answer in a mathematically rigorous manner.
    """

    MOD = 10**9
    total_sum = 0

    for k in range(1, max_exact_k + 1):
        n = 10**k
        try:
            a_n = find_nth_valid(n)
        except RuntimeError:
            a_n = find_nth_valid_large(n)
        total_sum += a_n

    for k in range(max_exact_k + 1, 19):
        n = 10**k
        a_n = find_nth_valid_large(n)
        total_sum += a_n

    return total_sum % MOD


def main() -> None:
    """Run a demonstration similar to the original Ruby script's CLI.

    This prints small verifications and then computes the requested sum's
    last nine digits using the mixed exact/heuristic approach.
    """

    print("=" * 60)
    print("Project Euler Problem 361 Solution (translated to Python)")
    print("=" * 60)
    print("Computing the last 9 digits of sum_{k=1}^{18} A_{10^k}")
    print()

    verify_small_cases()
    print()

    last_9_digits = compute_sum_last_9_digits()

    print("=" * 60)
    print("RESULTS")
    print("=" * 60)
    print(f"Last 9 digits (heuristic-inclusive): {last_9_digits:09d}")
    print("=" * 60)


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    try:
        main()
    except Exception as exc:  # pragma: no cover - defensive logging
        print(f"Error during computation: {exc}")
