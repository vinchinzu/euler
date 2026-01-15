"""Compute the variance for Project Euler Problem 389 dice process.

This module provides small, typed helpers for computing means and variances of
fair dice, and a specific computation of the expected value and variance of the
random variable I as defined in Project Euler Problem 389.

It is self-contained, uses only the Python standard library, and is compatible
with Python 3.12.
"""

from __future__ import annotations

from fractions import Fraction
from typing import Dict, Tuple


SIDES: Dict[str, int] = {
    "t": 4,   # T: 4-sided die
    "c": 6,   # C: 6-sided dice
    "o": 8,   # O: 8-sided dice
    "d": 12,  # D: 12-sided dice
    "i": 20,  # I: 20-sided dice
}


def die_mean(sides: int) -> Fraction:
    """Return the mean of a fair die with the given number of sides.

    Uses exact rational arithmetic for precision.
    """

    if sides <= 0:
        msg = "Number of sides must be positive."
        raise ValueError(msg)
    return Fraction(1 + sides, 2)


def die_variance(sides: int) -> Fraction:
    """Return the variance of a fair die with the given number of sides.

    Computed exactly using rational arithmetic.
    """

    if sides <= 0:
        msg = "Number of sides must be positive."
        raise ValueError(msg)

    mu = die_mean(sides)
    total_variance = sum((Fraction(face) - mu) ** 2 for face in range(1, sides + 1))
    return total_variance / Fraction(sides)


def hierarchical_variance(
    expected_count: Fraction,
    variance_count: Fraction,
    die_mean_value: Fraction,
    die_variance_value: Fraction,
) -> Fraction:
    """Return variance of a sum of a random number of i.i.d. dice.

    Given N (random) dice count with E[N] and Var(N), and X for a single die
    with mean die_mean_value and variance die_variance_value, this computes
    Var(sum_{k=1}^N X_k).
    """

    return die_variance_value * expected_count + (die_mean_value**2) * variance_count


def compute_variance_i() -> Tuple[Fraction, Fraction]:
    """Compute E[I] and Var(I) for the Euler 389 hierarchical dice process.

    Returns a pair of Fractions: (E[I], Var(I]).
    """

    # T stage (4-sided)
    mu_t = die_mean(SIDES["t"])
    var_t = die_variance(SIDES["t"])

    # C stage (6-sided)
    mu_6 = die_mean(SIDES["c"])
    var_6 = die_variance(SIDES["c"])
    e_c = mu_6 * mu_t
    var_c = hierarchical_variance(mu_t, var_t, mu_6, var_6)

    # O stage (8-sided)
    mu_8 = die_mean(SIDES["o"])
    var_8 = die_variance(SIDES["o"])
    e_o = mu_8 * e_c
    var_o = hierarchical_variance(e_c, var_c, mu_8, var_8)

    # D stage (12-sided)
    mu_12 = die_mean(SIDES["d"])
    var_12 = die_variance(SIDES["d"])
    e_d = mu_12 * e_o
    var_d = hierarchical_variance(e_o, var_o, mu_12, var_12)

    # I stage (20-sided)
    mu_20 = die_mean(SIDES["i"])
    var_20 = die_variance(SIDES["i"])
    e_i = mu_20 * e_d
    var_i = hierarchical_variance(e_d, var_d, mu_20, var_20)

    return e_i, var_i


def _main() -> None:
    """Run a small demonstration when executed as a script."""

    e_i, var_i = compute_variance_i()
    var_i_float = float(var_i)
    print(f"{var_i_float:.4f}")


if __name__ == "__main__":
    _main()
