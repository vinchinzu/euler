"""Project Euler Problem 323 - Expected steps for random bitwise OR saturation.

This module computes the expected number of steps N until the cumulative
bitwise-OR of random 32-bit unsigned integers reaches all 1-bits
(2**32 - 1) and stays there.

It provides utilities to:
- Precompute binomial coefficients
- Compute the expected steps for a given number of initially-unset bits
- Run a small verification and, when executed as a script, print the answer.
"""

from __future__ import annotations

from typing import List

BITS: int = 32
MAX_PRECISE_BITS: int = 53


def precompute_binomials(max_bits: int) -> List[List[int]]:
    """Precompute binomial coefficients up to ``max_bits``.

    Args:
        max_bits: Maximum value of n for which C(n, k) is required. Must
            be non-negative.

    Returns:
        A 2D list ``binoms`` where ``binoms[n][k] == C(n, k)`` for
        0 <= n <= max_bits and 0 <= k <= n.
    """

    if max_bits < 0:
        raise ValueError("max_bits must be non-negative")

    if max_bits == 0:
        return [[1]]

    size = max_bits + 1
    binoms: List[List[int]] = [[0] * size for _ in range(size)]
    binoms[0][0] = 1

    for n in range(1, size):
        binoms[n][0] = 1
        for k in range(1, n + 1):
            binoms[n][k] = binoms[n - 1][k - 1] + binoms[n - 1][k]

    return binoms


def expected_steps_for_unset_bits(unset_bits: int, binoms: List[List[int]]) -> float:
    """Compute expected steps until all bits are set, given unset_bits.

    The model matches Project Euler 323's process: at each step, we OR the
    current bitmask with a fresh random 32-bit integer, uniformly chosen.

    Args:
        unset_bits: Number of bits that are currently 0.
        binoms: Precomputed binomial coefficients from ``precompute_binomials``.

    Returns:
        Expected number of steps to reach zero unset bits from ``unset_bits``.
    """

    if not isinstance(unset_bits, int) or unset_bits < 0:
        msg = "unset_bits must be a non-negative integer"
        raise ValueError(msg)

    if unset_bits == 0:
        return 0.0

    if unset_bits > MAX_PRECISE_BITS:
        # Double precision may lose accuracy for extremely low probabilities.
        # We still compute with floats but warn via an exception-friendly msg.
        # Callers that care about rigor can catch or log this as needed.
        print(
            "Warning: unset_bits > MAX_PRECISE_BITS may cause precision issues "
            "with float computations."
        )

    max_n = len(binoms) - 1
    if unset_bits > max_n:
        msg = (
            "Binomial coefficients not precomputed for n="
            f"{unset_bits} (max available is {max_n})"
        )
        raise ValueError(msg)

    expected: List[float] = [0.0] * (unset_bits + 1)

    for k in range(1, unset_bits + 1):
        inv_two_k = 1.0 / (1 << k)
        p_stay = inv_two_k

        sum_transitions = 0.0
        for j in range(1, k):
            binom_coeff = binoms[k][j]
            p_j = binom_coeff * inv_two_k
            sum_transitions += p_j * expected[k - j]

        denominator = 1.0 - p_stay
        expected[k] = (1.0 + sum_transitions) / denominator

    return expected[unset_bits]


def _verify_solution(binoms: List[List[int]]) -> None:
    """Run quick sanity checks for small k.

    Raises AssertionError if checks fail.
    """

    # Known values for small cases.
    result1 = expected_steps_for_unset_bits(1, binoms)
    assert abs(result1 - 2.0) < 1e-12, f"k=1 failed: {result1}"

    result2 = expected_steps_for_unset_bits(2, binoms)
    expected2 = 8.0 / 3.0
    assert abs(result2 - expected2) < 1e-10, f"k=2 failed: {result2}"

    result3 = expected_steps_for_unset_bits(3, binoms)
    expected3 = 22.0 / 7.0
    assert abs(result3 - expected3) < 1e-10, f"k=3 failed: {result3}"


def main() -> None:
    """Compute and print the expected number of steps for 32-bit integers.

    Prints the result with 10 digits after the decimal point.
    """

    print(f"Precomputing binomial coefficients for up to {BITS} bits...")
    binoms = precompute_binomials(BITS)

    _verify_solution(binoms)

    print(f"Computing expected steps for {BITS} bits...")
    result = expected_steps_for_unset_bits(BITS, binoms)

    print(f"Expected value of N for {BITS}-bit numbers: {result:.10f}")
    print()
    print(f"The expected number of steps until all {BITS} bits are set to 1 is:")
    print(f"{result:.10f}")


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    try:
        main()
    except Exception as exc:  # Match Ruby script's top-level error handling
        print(f"Error: {exc}")
        raise
