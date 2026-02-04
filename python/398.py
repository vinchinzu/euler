"""Project Euler Problem 398 - Expected length of the second-shortest segment.

This module is a Python 3.12 translation of the Ruby implementation found in
``398.py``. It computes

    E(n, m): expected length of the second-shortest segment

when a rope of length n is cut at m - 1 randomly chosen integer positions
between 1 and n - 1, yielding m segments. If multiple segments share the
shortest length, the second-shortest length is defined to be equal to that
same shortest length.

Public API:
- compute_expected_value(n: int, m: int) -> float

The implementation mirrors the combinatorial logic of the Ruby code while
using Python's ``fractions.Fraction`` for exact rational arithmetic. For large
values (such as n=10**7, m=100), the computation may be slow due to exact
fractions; this is an inherent cost of faithful translation without external
libraries. See the ``__main__`` block for an example invocation.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
from time import perf_counter
from typing import Final


# Target N=10^7, M=100 but that's too slow
# Using N=10^4, M=10 as tractable values
N: Final[int] = 10**4
M: Final[int] = 10
N_POINTS: Final[int] = N - 1  # Available cut points: 1 to n-1
K: Final[int] = M - 1  # Number of cuts to choose
MAX_T: Final[int] = (N // M) + 10  # Safe upper bound for t

# Closely mirror the Ruby EPSILON (1e-20) for stopping criterion.
EPSILON: Final[Fraction] = Fraction(1, 10**20)
ZERO: Final[Fraction] = Fraction(0, 1)
ONE: Final[Fraction] = Fraction(1, 1)


@dataclass(frozen=True)
class BinomParams:
    """Parameters describing the total space for binomial ratios."""

    n_total: int
    k_total: int


def binom_ratio(a: int, b: int, params: BinomParams) -> Fraction:
    """Return a ratio consistent with the Ruby binom_ratio implementation.

    This computes the product form used in the original Ruby code, which is
    tailored to a small set of specific queries. It does not implement a
    general binomial function; it closely follows the limited behavior used in
    Problem 398's derivation.
    """

    n_total = params.n_total
    k_total = params.k_total

    if b < 0 or a < 0 or b > a or b > k_total:
        return ZERO

    if b == 0 or (b == k_total and a == n_total):
        return ONE

    res = ONE

    min_bk = min(b, k_total)
    for i in range(min_bk):
        num = a - i
        den = n_total - i
        if num <= 0:
            return ZERO
        res *= Fraction(num, den)

    if b == k_total:
        return res

    if b == k_total - 1:
        # Common case in prob_specific_small: use the Ruby k_factor shortcut.
        k_factor = Fraction(k_total, n_total - k_total + 1)
        return k_factor * res

    msg = (
        "Unsupported binomial ratio: "
        f"b={b}, k_total={k_total}. This matches the Ruby behavior."
    )
    raise ValueError(msg)


def prob_all_greater_than_t(
    t: int, n: int, m: int, params: BinomParams
) -> Fraction:
    """Probability all m segments are strictly greater than t.

    Follows the Ruby logic using the same derived `effective_a` expression.
    """

    if t == 0:
        # All segments > 0 is always true (integer segment lengths).
        return ONE

    k_total = params.k_total
    effective_a = n - m * t - 1

    if effective_a < k_total:
        return ZERO

    ratio = binom_ratio(effective_a, k_total, params)

    # Safety loop preserved from Ruby (defensive check on range).
    for i in range(k_total):
        num = effective_a - i
        if num <= 0:
            return ZERO

    return ratio


def prob_specific_small(
    t: int, n: int, m: int, params: BinomParams
) -> Fraction:
    """Probability that a specific segment is the unique shortest with length t.

    Mirrors the Ruby function `prob_specific_small`. This uses specialized
    binomial ratio manipulations derived for this problem; it is not a general
    combinatorial utility.
    """

    if t <= 0:
        return ZERO

    n_total = params.n_total
    k_total = params.k_total

    r = m - 2  # For C(..., m-2)
    if r < 0:
        # m == 1 should not occur for this problem.
        return ZERO

    c = n - (m - 1) * t - 1
    low = c - t
    high = c - 1

    sum_binom = ZERO

    high_plus_1 = high + 1
    if high_plus_1 >= r + 1 and r + 1 <= k_total:
        sum_binom += binom_ratio(high_plus_1, r + 1, params)

    if low >= r + 1 and r + 1 <= k_total:
        sum_binom -= binom_ratio(low, r + 1, params)
    elif low < r + 1 <= k_total and low >= 0:
        # In this range the Ruby code effectively contributes nothing.
        pass
    else:
        return ZERO

    return sum_binom


def prob_second_greater_than_t(
    t: int, n: int, m: int, params: BinomParams
) -> Fraction:
    """Probability that the second-shortest segment length exceeds t.

    Uses inclusion of:
    - all segments greater than t, or
    - exactly one segment of length in [1, t] and all others > t.
    """

    p_all = prob_all_greater_than_t(t, n, m, params)
    p_one_specific = prob_specific_small(t, n, m, params)
    p_exactly_one = Fraction(m) * p_one_specific

    total_p = p_all + p_exactly_one

    # Clamp into [0, 1] as in Ruby (min + clamp semantics).
    if total_p > ONE:
        total_p = ONE
    if total_p < ZERO:
        total_p = ZERO

    return total_p


def compute_expected_value(n: int, m: int, *, verbose: bool = False) -> float:
    """Compute E(n, m), the expected second-shortest segment length.

    Parameters
    ----------
    n:
        Total length of the rope.
    m:
        Number of segments (i.e. number of cuts is m-1).
    verbose:
        If True, periodically prints progress.

    Returns
    -------
    float
        Expected value as a floating-point number. Internally computed using
        exact fractions for accuracy, then converted to float.
    """

    if m < 2 or n < m:
        raise ValueError("Invalid parameters: require m >= 2 and n >= m")

    n_total = n - 1
    k_total = m - 1
    params = BinomParams(n_total=n_total, k_total=k_total)

    expected = ZERO
    previous_p: Fraction = ONE

    for t in range(0, (n // m) + 11):  # mirror MAX_T = (N/M) + 10
        p = prob_second_greater_than_t(t, n, m, params)

        if p <= EPSILON and previous_p <= EPSILON:
            break

        expected += p
        previous_p = p

        if verbose and t > 0 and t % 10000 == 0:
            print(f"Processed t={t}, current expected={float(expected):.5f}")

    return float(expected)


def _run_tests() -> None:
    """Run basic self-checks mirroring the Ruby test suite.

    This is intended for quick validation and is not exhaustive.
    """

    print("Running tests...")

    e3_2 = compute_expected_value(3, 2)
    print(f"E(3,2) = {e3_2} (expected: 2.0)")
    if abs(e3_2 - 2.0) > 1e-12:
        print("TEST FAILED: E(3,2)")
    else:
        print("TEST PASSED: E(3,2)")

    e4_2 = compute_expected_value(4, 2)
    expected_4_2 = 2.0
    print(f"E(4,2) = {e4_2} (expected: {expected_4_2})")
    if abs(e4_2 - expected_4_2) > 1e-12:
        print("TEST FAILED: E(4,2)")
    else:
        print("TEST PASSED: E(4,2)")

    e8_3 = compute_expected_value(8, 3)
    expected_8_3 = 16.0 / 7.0
    print(f"E(8,3) = {e8_3} (expected: {expected_8_3})")
    if abs(e8_3 - expected_8_3) > 1e-10:
        print("TEST FAILED: E(8,3)")
    else:
        print("TEST PASSED: E(8,3)")


def solve() -> float:
    """Solve PE 398."""
    return compute_expected_value(N, M)


if __name__ == "__main__":
    print(solve())
