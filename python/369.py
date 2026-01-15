"""Project Euler Problem 369 - Badugi Hands (approximate inclusion-exclusion).

This module provides an idiomatic Python 3.12 translation of the Ruby script for
Problem 369. It estimates the count of n-card hands (4 <= n <= 13) that contain
at least one 4-card Badugi (4 distinct ranks, 4 distinct suits).

The original Ruby code uses:
- exact inclusion-exclusion up to pair intersections,
- a Monte Carlo-style sampling/scale for triple intersections, and
- ignores higher-order intersections (with a warning for larger n).

This translation preserves that behavior while:
- using type hints,
- keeping functions focused and under 88 chars,
- avoiding external dependencies,
- remaining executable as a standalone module.
"""

from __future__ import annotations

from dataclasses import dataclass
from itertools import combinations, permutations
from math import comb, factorial
from typing import Dict, Iterable, List, Sequence, Tuple

NUM_SUITS: int = 4
NUM_RANKS: int = 13
TOTAL_CARDS: int = NUM_SUITS * NUM_RANKS


@dataclass(frozen=True)
class BadugiSet:
    """Represents a 4-card Badugi set as card indices in [0, TOTAL_CARDS).

    Cards are encoded as: card = rank * NUM_SUITS + suit.
    """

    cards: Tuple[int, int, int, int]

    def union_size(self, other_sets: Iterable["BadugiSet"]) -> int:
        """Return size of union of this set with the given Badugi sets."""

        unique_cards: set[int] = set(self.cards)
        for bset in other_sets:
            unique_cards.update(bset.cards)
        return len(unique_cards)


def compute_binomial_coefficients(max_n: int) -> List[List[int]]:
    """Precompute binomial coefficients C(n, k) for 0 <= n, k <= max_n.

    While math.comb is available and is used elsewhere, we mirror the Ruby
    structure for clarity and potential reuse where table lookup is preferred.
    """

    binoms: List[List[int]] = [[0] * (max_n + 1) for _ in range(max_n + 1)]
    for n in range(max_n + 1):
        binoms[n][0] = 1
        binoms[n][n] = 1
        for k in range(1, n):
            binoms[n][k] = binoms[n - 1][k - 1] + binoms[n - 1][k]
    return binoms


def binom(n: int, k: int, binoms: Sequence[Sequence[int]]) -> int:
    """Return C(n, k) using a precomputed table; 0 if k is out of range."""

    if k < 0 or k > n:
        return 0
    return binoms[n][k]


def generate_badugi_sets() -> List[BadugiSet]:
    """Generate all valid 4-card Badugi sets from a standard 52-card deck."""

    badugi_sets: List[BadugiSet] = []
    for rank_combo in combinations(range(NUM_RANKS), 4):
        for suit_perm in permutations(range(NUM_SUITS), 4):
            cards = tuple(r * NUM_SUITS + s for r, s in zip(rank_combo, suit_perm))
            badugi_sets.append(BadugiSet(cards))
    return badugi_sets


def expected_badugi_count() -> int:
    """Return the expected number of Badugi sets.

    Computed as C(NUM_RANKS, 4) * NUM_SUITS!.
    """

    return comb(NUM_RANKS, 4) * factorial(NUM_SUITS)


def validate_badugi_sets(badugi_sets: Sequence[BadugiSet]) -> None:
    """Validate that the generated Badugi sets match the expected count.

    Raises a ValueError if the count does not match.
    """

    expected = expected_badugi_count()
    if len(badugi_sets) != expected:
        msg = f"Expected {expected} Badugi sets, got {len(badugi_sets)}"
        raise ValueError(msg)
    print(f"✓ Generated {len(badugi_sets)} Badugi sets (expected {expected})")


def union_size(badugi_sets_slice: Iterable[BadugiSet]) -> int:
    """Return size of the union of the given Badugi sets."""

    unique_cards: set[int] = set()
    for bset in badugi_sets_slice:
        unique_cards.update(bset.cards)
    return len(unique_cards)


def compute_pair_counts_cached(
    badugi_sets: Sequence[BadugiSet],
) -> Dict[int, int]:
    """Precompute and cache pairwise union sizes.

    This is called once and reused for all n, avoiding recomputing
    147M pairs for each of the 10 values of n.
    """
    print("Precomputing pairwise union sizes (this may take a moment)...")
    pair_counts: Dict[int, int] = {}
    for b1, b2 in combinations(badugi_sets, 2):
        u_size = union_size((b1, b2))
        pair_counts[u_size] = pair_counts.get(u_size, 0) + 1
    return pair_counts


def compute_pair_contribution(
    pair_counts: Dict[int, int],
    binoms: Sequence[Sequence[int]],
    n: int,
) -> int:
    """Compute the pairwise inclusion-exclusion term for f(n) using cached counts."""

    total = 0
    for u_size, count in pair_counts.items():
        if n >= u_size:
            remaining = TOTAL_CARDS - u_size
            ways = binom(remaining, n - u_size, binoms)
            total += count * ways
    return total


def compute_triple_contribution(
    badugi_sets: Sequence[BadugiSet],
    binoms: Sequence[Sequence[int]],
    n: int,
) -> int:
    """Approximate triple-intersection contribution using sampling.

    This follows the Ruby approach:
    - Compute total number of triples.
    - Sample up to 10,000 triples from combinations(badugi_sets, 3) by
      taking the first sample_size triples (note: not random; deterministic).
    - Scale observed union-size counts by total_triples / sample_size.

    This is an approximation and can differ slightly from the exact value.
    """

    print("Computing triple intersections (this may take a moment)...")

    total_triples = comb(len(badugi_sets), 3)
    sample_size = min(10_000, total_triples)
    if sample_size == 0:
        return 0

    triple_counts: Dict[int, float] = {}
    # Deterministic sampling of first sample_size triples.
    for idx, (b1, b2, b3) in enumerate(combinations(badugi_sets, 3)):
        if idx >= sample_size:
            break
        u_size = union_size((b1, b2, b3))
        triple_counts[u_size] = triple_counts.get(u_size, 0.0) + 1.0

    scale_factor = float(total_triples) / float(sample_size)
    for u_size in list(triple_counts.keys()):
        triple_counts[u_size] *= scale_factor

    total = 0.0
    for u_size, count in triple_counts.items():
        if n >= u_size:
            remaining = TOTAL_CARDS - u_size
            ways = binom(remaining, n - u_size, binoms)
            total += count * ways

    # Rounded to nearest integer; the original Ruby code leaves this as float.
    return int(round(total))


def compute_f_n(
    badugi_sets: Sequence[BadugiSet],
    pair_counts: Dict[int, int],
    binoms: Sequence[Sequence[int]],
    n: int,
) -> int:
    """Estimate f(n): ways to choose n cards containing a 4-card Badugi subset.

    Implementation notes:
    - Exact single-set term.
    - Exact pairwise term (using precomputed pair_counts).
    - Approximate triple term (scaled sampling, as in the Ruby script).
    - Ignores higher-order terms; for n >= 16 this may be significant, but
      here we only use 4 <= n <= 13 as in the original problem.
    """

    if n < 4:
        raise ValueError("n must be at least 4")
    if n > TOTAL_CARDS:
        raise ValueError("n cannot exceed total cards")

    # Single-term: choose positions for the Badugi (4 cards), rest from others.
    single_term = binom(TOTAL_CARDS - 4, n - 4, binoms)
    f_n = len(badugi_sets) * single_term

    # Subtract pairwise overlaps using cached counts.
    f_n -= compute_pair_contribution(pair_counts, binoms, n)

    # Add approximate triple overlaps.
    f_n += compute_triple_contribution(badugi_sets, binoms, n)

    if n >= 16:
        # Kept for behavioral parity; not triggered for the target range.
        print("Warning: Higher-order terms may be significant for n >= 16")

    return max(0, int(f_n))


def run_computation() -> int:
    """Run validation and compute sum of f(n) for 4 <= n <= 13.

    Returns the final sum. Printed values mirror the Ruby script's output
    style and include basic sanity checks for f(4) and f(5).
    """

    print("Project Euler Problem 369 - Badugi Hands")
    print("=" * 40)

    print("Precomputing binomial coefficients...")
    binoms = compute_binomial_coefficients(TOTAL_CARDS)

    print("Generating Badugi sets...")
    badugi_sets = generate_badugi_sets()
    validate_badugi_sets(badugi_sets)

    # Precompute pairwise union sizes ONCE (avoids 10x redundant computation)
    pair_counts = compute_pair_counts_cached(badugi_sets)

    print("\nValidation tests:")
    test_f4 = compute_f_n(badugi_sets, pair_counts, binoms, 4)
    expected_f4 = len(badugi_sets)
    print(f"f(4) = {test_f4} (expected {expected_f4})")
    if test_f4 != expected_f4:
        raise ValueError("Test failed: f(4) incorrect")

    test_f5 = compute_f_n(badugi_sets, pair_counts, binoms, 5)
    expected_f5 = 514_800
    print(f"f(5) = {test_f5} (expected {expected_f5})")
    tolerance = 1_000
    if abs(test_f5 - expected_f5) >= tolerance:
        diff = abs(test_f5 - expected_f5)
        raise ValueError(f"Test failed: f(5) incorrect (diff = {diff})")

    print("\nComputing f(n) for n = 4 to 13...")
    sum_f = 0
    for n in range(4, 14):
        f_n = compute_f_n(badugi_sets, pair_counts, binoms, n)
        print(f"f({n}) = {f_n}")
        sum_f += f_n

    print("\n" + "=" * 40)
    print(f"Final result: f(n) for 4  n  13 = {sum_f}")
    print("=" * 40)
    return sum_f


def main() -> None:
    """Entry point used when running this module as a script."""

    result = run_computation()

    # Print only final answer for test harness
    print()
    print(result)


if __name__ == "__main__":  # pragma: no cover - CLI entry
    main()
