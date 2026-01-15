# Project Euler Problem 869
#
# PROBLEM DESCRIPTION:
# <p>
# A prime is drawn uniformly from all primes not exceeding $N$. The prime is written in binary notation, and a player tries to guess it bit-by-bit starting at the least significant bit. The player scores one point for each bit they guess correctly. Immediately after each guess, the player is informed whether their guess was correct, and also whether it was the last bit in the number - in which case the game is over.</p>
# 
# <p>
# Let $E(N)$ be the expected number of points assuming that the player always guesses to maximize their score. For example, $E(10)=2$, achievable by always guessing "1". You are also given $E(30)=2.9$.</p>
# 
# <p>
# Find $E(10^8)$. Give your answer rounded to eight digits after the decimal point.</p>
#
# RUBY CODE INSIGHTS:
# # NOTE: Placeholder runner added to keep the file executable.
# # The original solution draft from solutions/sky_solutions is preserved below __END__ for reference.
# puts "Problem 869 placeholder implementation."
# __END__
# require 'prime'
# def compute_e(n)
#   # Generate all primes up to n
#   primes = Prime.each(n).to_a
#   num_primes = primes.length
#   # Precompute binary representations and their lengths
#   binary_info = primes.map do |p|
#     bin = p.to_s(2)
#     len = bin.length
#     [bin, len]
#   end
#   # Group primes by their binary length
#   max_len = binary_info.map { |_, len| len }.max
#   groups = Array.new(max_len + 1) { [] }
#   binary_info.each do |bin, len|
#     groups[len] << bin
#   end
#   # Precompute for each length the number of primes with that length
#   count_by_len = groups.map { |g| g.length }
#   # Precompute suffix counts for each length
#   suffix_counts = Array.new(max_len + 1) { [] }
#   (1..max_len).each do |len|
#     if count_by_len[len] > 0
#       suffix_counts[len] = Array.new(len + 1, 0)
#       groups[len].each do |bin|
#         suffix_len = len
#         suffix_counts[len][suffix_len] += 1
#         while suffix_len > 0
#           suffix_len -= 1
#           suffix_counts[len][suffix_len] += 1
#         end
#       end
#     end
#   end
#   # Dynamic programming table: dp[len][pos] = expected score from position pos for numbers of length len
#   # We compute it bottom-up for each length
#   total_score = 0.0
#   (1..max_len).each do |len|
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

#!/usr/bin/env python3
"""
Project Euler 869 - Prime Bit Guessing Game

Key insight: Instead of tracking individual primes, track binary patterns.
Group primes by their binary representation and work with pattern frequencies.
"""

def sieve_of_eratosthenes(limit):
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []

    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False

    return [i for i in range(2, limit + 1) if is_prime[i]]


def compute_e(n):
    """
    Compute E(N) - expected score when guessing binary representation of random prime ≤ N.

    Efficient approach: Group primes by binary patterns and track frequencies.
    """
    primes = sieve_of_eratosthenes(n)
    if not primes:
        return 0.0

    total_primes = len(primes)

    # Group primes by their binary representation
    from collections import defaultdict
    pattern_counts = defaultdict(int)

    for p in primes:
        binary = bin(p)[2:]
        pattern_counts[binary] += 1

    # Now compute expected score using pattern frequencies
    # At each bit position, optimal strategy is to guess the more common bit

    total_score = 0.0

    # Process each unique binary pattern
    for pattern, count in pattern_counts.items():
        pattern_length = len(pattern)
        score_for_pattern = 0.0

        # For this pattern, compute score at each bit position
        # Key insight: We can compute the expected score by considering all patterns together
        # at each bit position

        # Actually, let me reconsider the problem structure more carefully.
        # The player doesn't know which pattern they're guessing initially.
        # They learn bit-by-bit and update their belief.

        # Let's use a different approach: simulate the decision tree more efficiently
        # by tracking pattern groups rather than individual primes.

        for bit_pos in range(pattern_length):
            # For this bit position, the player guesses based on all patterns
            # that are still possible given previous guesses
            # This is still complex...
            pass

    # Let me try yet another approach: forward simulation by bit position
    # Track sets of patterns that remain possible after each bit

    # State = dictionary mapping remaining patterns to their counts
    def expected_score_recursive(pattern_dict, bit_pos):
        """
        Compute expected score starting from bit_pos with given pattern counts.

        pattern_dict: {pattern: count} for patterns still possible
        bit_pos: current bit position (0 = LSB)
        """
        if not pattern_dict:
            return 0.0

        # Separate patterns by bit value at this position
        patterns_0 = {}
        patterns_1 = {}

        total_count = 0
        for pattern, count in pattern_dict.items():
            if bit_pos >= len(pattern):
                # Pattern too short - already fully guessed
                continue

            bit_val = pattern[-(bit_pos + 1)]
            total_count += count

            if bit_val == '0':
                patterns_0[pattern] = count
            else:
                patterns_1[pattern] = count

        if total_count == 0:
            return 0.0

        count_0 = sum(patterns_0.values())
        count_1 = sum(patterns_1.values())

        prob_0 = count_0 / total_count
        prob_1 = count_1 / total_count

        # Filter to patterns that continue beyond this bit
        patterns_0_continue = {p: c for p, c in patterns_0.items() if len(p) > bit_pos + 1}
        patterns_1_continue = {p: c for p, c in patterns_1.items() if len(p) > bit_pos + 1}

        # Memoization would help but dictionary keys are tricky
        future_0 = expected_score_recursive(patterns_0_continue, bit_pos + 1) if patterns_0_continue else 0.0
        future_1 = expected_score_recursive(patterns_1_continue, bit_pos + 1) if patterns_1_continue else 0.0

        # Optimal guess is the more common bit
        immediate_score = max(prob_0, prob_1)

        # Weighted average of future scores
        count_continue_0 = sum(patterns_0_continue.values())
        count_continue_1 = sum(patterns_1_continue.values())
        total_continue = count_continue_0 + count_continue_1

        if total_continue > 0:
            future_score = (count_continue_0 * future_0 + count_continue_1 * future_1) / total_count
        else:
            future_score = 0.0

        return immediate_score + future_score

    return expected_score_recursive(pattern_counts, 0)


if __name__ == "__main__":
    result = compute_e(10**8)
    print(f"{result:.8f}")
