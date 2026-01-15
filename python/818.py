"""Project Euler Problem 818: SET.

Given a SET deck of 81 = B^D cards, define S(C) to be the number of SETs
in the set of cards C. Find Σ_C S(C)^K over all subsets of cards C of
size N.

We can write S(C) = Σ_k s_k, where s_k is the indicator variable for
whether SET k appears in the subset. That means we can expand S(C)^K into
a polynomial over the indicator variables, each term containing from 1 to
K distinct variables. For each 1≤e≤K, we can count the total number of
terms with e distinct variables: for any k, we can find the number of
ways that e SETS contain k distinct cards each, after which there are
nCr(81-k, N-k) ways to choose the remaining cards.

The number of different "term shapes" is simple: each of the K variables
must be one of the e distinct variables, so there are e^K shapes, but we
use Inclusion Exclusion to remove the shapes consisting of fewer than e
variables.

The answer is the sum of these values over all e and k. We perform one
final optimization: when iterating over SET combinations, we assume
without loss of generality that the first SET is always (0,0,0,0),
(0,0,0,1), (0,0,0,2). We need to multiply the final value by the number
of SETS (since the first SET could be any of them), and divide by the
number of SETs e (this could have been any SET).
"""

from __future__ import annotations

from itertools import product
from math import comb
from typing import List, Set, Tuple


def imod(a: int, m: int) -> int:
    """Integer mod handling negatives."""
    return ((a % m) + m) % m


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    return comb(n, r)


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 818."""
    N = 12
    K = 4
    B = 3
    D = 4

    # Generate all cards
    cards: List[Tuple[int, ...]] = list(product(range(B), repeat=D))
    card_to_index = {card: i for i, card in enumerate(cards)}

    # Find all SETs
    sets: List[List[int]] = []
    for i in range(len(cards)):
        for j in range(i + 1, len(cards)):
            card1 = cards[i]
            card2 = cards[j]
            # Find third card in SET
            card3 = tuple(imod(-(card1[k] + card2[k]), B) for k in range(D))
            if card3 in card_to_index:
                k = card_to_index[card3]
                if k > j:
                    sets.append([i, j, k])

    ans = 0
    card_counts = [0] * len(cards)
    for i in range(B):
        card_counts[i] = 1

    for e in range(1, K + 1):
        num_distinct_counts = [0] * (N + 1)

        def helper(min_set_index: int, num_sets: int, num_distinct: int) -> None:
            """Recursive helper to count distinct card combinations."""
            if num_sets == 0:
                num_distinct_counts[num_distinct] += 1
                return
            for set_index in range(min_set_index, len(sets)):
                new_num_distinct = num_distinct
                for card_index in sets[set_index]:
                    card_counts[card_index] += 1
                    if card_counts[card_index] == 1:
                        new_num_distinct += 1
                helper(set_index + 1, num_sets - 1, new_num_distinct)
                for card_index in sets[set_index]:
                    card_counts[card_index] -= 1

        helper(1, e - 1, B)

        # Compute number of shapes using inclusion-exclusion
        num_shapes = 0
        for i in range(e + 1):
            num_shapes += parity(i) * nCr(e, i) * pow(e - i, K)

        for k in range(N + 1):
            ans += (
                num_shapes
                // e
                * len(sets)
                * nCr(len(cards) - k, N - k)
                * num_distinct_counts[k]
            )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
