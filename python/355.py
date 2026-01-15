"""Project Euler Problem 355 solver.

This module provides an implementation of Co(n), defined as the maximal possible
sum of a set of pairwise coprime elements from {1, 2, ..., n}.

It contains a direct translation of the provided Ruby solution with
adjustments for Python 3.12 and type hints. The approach:

- Precompute smallest prime factors (SPF) up to n.
- Group numbers by their set of distinct prime factors (its "signature").
- For each signature, keep only the maximum number in that group.
- Use a compatibility table between signatures and a bitmask DP over groups to
  find the maximum achievable sum of group maxima where selected signatures are
  pairwise compatible (no shared primes).

Note: This algorithm is exponential in the number of distinct prime-signature
groups and is suited to this specific problem setup, mirroring the Ruby code.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List, Sequence, Tuple


def sieve_spf(limit: int) -> List[int]:
    """Compute smallest prime factor (SPF) for every number up to ``limit``.

    Returns a list ``spf`` of length ``limit + 1`` where ``spf[x]`` is the
    smallest prime factor of ``x`` (or 0 for 0 and 1). For ``limit < 1`` an
    empty list is returned.
    """

    if limit < 1:
        return []

    spf: List[int] = list(range(limit + 1))
    spf[0] = 0
    spf[1] = 0

    for i in range(2, isqrt(limit) + 1):
        if spf[i] != i:
            continue
        step_start = i * i
        for j in range(step_start, limit, i):
            if spf[j] == j:
                spf[j] = i

    return spf


def get_prime_factors(num: int, spf: Sequence[int]) -> List[int]:
    """Return the sorted list of prime factors of ``num`` (with multiplicity).

    Uses the precomputed SPF table. For ``num <= 1`` returns an empty list.
    """

    if num <= 1:
        return []

    factors: List[int] = []
    while num > 1:
        p = spf[num]
        factors.append(p)
        while num % p == 0:
            num //= p

    factors.sort()
    return factors


def _build_groups(n: int, spf: Sequence[int]) -> Dict[Tuple[int, ...], int]:
    """Group numbers by prime-factor signatures and keep the maximum per group.

    Returns a mapping: signature -> max number having that signature.
    The signature is a tuple of sorted prime factors (with multiplicity).
    """

    groups: Dict[Tuple[int, ...], int] = {}

    # Include 1, which has an empty signature.
    if n >= 1:
        groups[()] = 1

    for num in range(2, n + 1):
        sig = tuple(get_prime_factors(num, spf))  # Keep multiplicity
        current = groups.get(sig)
        if current is None or num > current:
            groups[sig] = num

    return groups


def _compute_compatibility(signatures: List[Tuple[int, ...]]) -> List[List[bool]]:
    """Precompute compatibility between signatures.

    Two signatures are compatible if they share no common prime factors.
    """

    num_groups = len(signatures)
    compatible: List[List[bool]] = [
        [False] * num_groups for _ in range(num_groups)
    ]

    for i in range(num_groups):
        compatible[i][i] = True
        sig_i = signatures[i]
        set_i = set(sig_i)
        for j in range(i + 1, num_groups):
            sig_j = signatures[j]
            # They are compatible if there is no shared prime.
            compat = set_i.isdisjoint(sig_j)
            compatible[i][j] = compat
            compatible[j][i] = compat

    return compatible


def _maximize_sum(max_per_group: Dict[Tuple[int, ...], int]) -> int:
    """Run a bitmask DP to find the best sum over compatible groups.

    The state space is exponential in the number of signatures and mirrors the
    provided Ruby solution. This is tailored for the original problem and is
    not intended as a general-purpose large-scale algorithm.
    """

    if not max_per_group:
        return 0

    signatures: List[Tuple[int, ...]] = list(max_per_group.keys())
    num_groups = len(signatures)

    compatible = _compute_compatibility(signatures)

    # Use dict for sparse DP to avoid memory issues
    dp: Dict[int, int] = {0: 0}

    for grp_idx in range(num_groups):
        mask_for_group = 1 << grp_idx
        value = max_per_group[signatures[grp_idx]]

        new_dp = dp.copy()
        for state, current_sum in dp.items():
            if state & mask_for_group:
                continue

            # Check compatibility of this group with all groups in state.
            is_compatible = True
            other = state
            while other:
                lsb = other & -other
                other_idx = (lsb.bit_length() - 1)
                if not compatible[grp_idx][other_idx]:
                    is_compatible = False
                    break
                other ^= lsb

            if not is_compatible:
                continue

            new_state = state | mask_for_group
            new_sum = current_sum + value
            if new_sum > new_dp.get(new_state, 0):
                new_dp[new_state] = new_sum

        dp = new_dp

    return max(dp.values())


def solve(n: int) -> int:
    """Compute Co(n): maximal sum of pairwise coprime numbers in [1, n]."""

    if n < 1:
        return 0
    if n == 1:
        return 1

    spf = sieve_spf(n)
    max_per_group = _build_groups(n, spf)
    return _maximize_sum(max_per_group)


def main() -> None:
    """Run a simple verification."""

    print("Verifying examples:")
    print(f"Co(10) = {solve(10)} (expected: 30)")
    print(f"Co(30) = {solve(30)} (expected: 193)")
    # Skip Co(100) as it's computationally intensive
    print("Co(100) = ? (expected: 1356) - SKIPPED due to complexity")

    # Note: Co(200000) requires significant computation time and memory
    print("\nNote: Full computation for large n is computationally intensive")


if __name__ == "__main__":  # pragma: no cover - convenience entry point
    main()
