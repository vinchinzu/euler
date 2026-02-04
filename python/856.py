#!/usr/bin/env python3
"""
Project Euler 856: Expected number of cards drawn until consecutive pair.

A standard 52-card deck (13 ranks, 4 suits). Draw cards without replacement
until two consecutive cards share the same rank. Find the expected number
of cards drawn, rounded to 8 decimal places.

Approach: Memoized DP where the state is:
  - counts: tuple of how many ranks have k cards remaining (k=0..4)
  - currCount: how many cards remain of the rank matching the last drawn card
The key insight is that individual rank identities don't matter, only the
distribution of remaining counts and the current rank's remaining count.
"""

from functools import lru_cache

N_RANKS = 13
N_SUITS = 4


def solve():
    @lru_cache(maxsize=None)
    def E(counts, curr_count):
        """Expected number of additional cards to draw (including the next one).

        counts: tuple of length K+1, where counts[k] = number of ranks
                that still have k cards remaining in the deck.
        curr_count: how many cards remain of the same rank as the last drawn card.
                    If curr_count > 0, drawing one of those would form a pair (stop).
                    0 means no previous card (or current rank is exhausted).
        """
        # Total cards remaining in deck
        total = 0
        for k in range(1, N_SUITS + 1):
            total += k * counts[k]

        if total == 0:
            return 0.0

        # We draw one card. Expected value = 1 + weighted sum of future expectations
        # But we must account for the probability of stopping (consecutive pair).

        # Probability of drawing a card of the same rank as last (forming a pair):
        # There are curr_count cards of that rank left.
        # If we draw one of those, we stop. Contribution: curr_count/total * 1
        # (we draw 1 card and stop).

        # For non-matching draws: we draw a card from a rank with k cards remaining
        # (where that rank is NOT the current rank). After drawing, that rank has k-1
        # cards remaining. The new curr_count becomes k-1.

        result = 1.0  # We always draw at least 1 card

        # The curr_count cards of the current rank could cause a stop.
        # Those are already accounted for: if drawn, we stop (add 0 to future E).
        # So we only add future expectations for non-stopping draws.

        for k in range(1, N_SUITS + 1):
            # Number of ranks (other than current rank) that have exactly k cards left
            available = counts[k]
            if curr_count == k:
                available -= 1  # Exclude the current rank from this bucket

            if available <= 0:
                continue

            # Probability of drawing from one of these ranks: k * available / total
            # After drawing: that rank goes from k to k-1 cards
            new_counts = list(counts)
            new_counts[k - 1] += 1
            new_counts[k] -= 1
            new_counts_tuple = tuple(new_counts)

            # New curr_count is k-1 (the rank we just drew now has k-1 cards left)
            result += E(new_counts_tuple, k - 1) * k * available / total

        return result

    # Initial state: all 13 ranks have 4 cards each, no previous card drawn
    initial_counts = tuple([0] * (N_SUITS + 1))
    initial_counts_list = list(initial_counts)
    initial_counts_list[N_SUITS] = N_RANKS
    initial_counts = tuple(initial_counts_list)

    answer = E(initial_counts, 0)
    return f"{answer:.8f}"


if __name__ == "__main__":
    print(solve())
