"""
Project Euler Problem 928: Cribbage Scoring

Find the number of hands in a normal pack of cards where the hand score
equals the cribbage score.

Hand score = sum of card values (Ace=1, court cards=10)
Cribbage score = pairs (2 pts each) + runs (length pts) + fifteens (2 pts each)

Time Complexity: O(5^13) = O(1) since we enumerate all possible multisets
Space Complexity: O(13)
"""

from itertools import product

RANKS = list(range(1, 14))  # 1=Ace, 11=Jack, 12=Queen, 13=King
MAX_COUNT_PER_RANK = 4
TARGET_SUM = 15


def rank_value(rank):
    """Get the numeric value of a rank."""
    return 1 if rank == 1 else min(rank, 10)


def binomial(n, k):
    """Compute C(n, k)."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = result * (n - i) // (i + 1)
    return result


def suit_combinations(count):
    """Number of ways to choose count cards from 4 suits."""
    return binomial(4, count)


def calculate_pairs_score(counts):
    """Calculate score from pairs."""
    pairs_score = 0
    for count in counts:
        if count >= 2:
            pairs_score += (count * (count - 1) // 2) * 2
    return pairs_score


def calculate_runs_score(counts):
    """Calculate score from runs (consecutive ranks)."""
    runs_score = 0

    i = 0
    while i < len(RANKS) - 2:
        run_length = 0
        j = i

        while j < len(RANKS) and counts[j] > 0:
            run_length += 1
            j += 1

        if run_length >= 3:
            run_product = 1
            for k in range(i, i + run_length):
                run_product *= counts[k]

            runs_score += run_length * run_product
            i += run_length
        else:
            i += 1

    return runs_score


def calculate_fifteens_score(counts):
    """Calculate score from fifteens using generating functions."""
    # gf[s] = number of ways to make sum s
    gf = [0] * (TARGET_SUM + 1)
    gf[0] = 1

    for idx, rank in enumerate(RANKS):
        value = rank_value(rank)
        count = counts[idx]

        new_gf = [0] * (TARGET_SUM + 1)

        for k in range(count + 1):
            coeff = suit_combinations(k)
            power = k * value

            if power <= TARGET_SUM:
                for s in range(TARGET_SUM + 1):
                    if s + power <= TARGET_SUM:
                        new_gf[s + power] += gf[s] * coeff

        gf = new_gf

    return gf[TARGET_SUM] * 2


def calculate_hand_score(counts):
    """Calculate total hand value."""
    total_value = 0
    for idx, rank in enumerate(RANKS):
        total_value += counts[idx] * rank_value(rank)
    return total_value


def calculate_cribbage_score(counts):
    """Calculate total cribbage score."""
    pairs = calculate_pairs_score(counts)
    runs = calculate_runs_score(counts)
    fifteens = calculate_fifteens_score(counts)
    return pairs + runs + fifteens


def calculate_num_hands(counts):
    """Calculate number of actual hands with this multiset."""
    product = 1
    for count in counts:
        product *= suit_combinations(count)
    return product


def solve_problem():
    """Enumerate all possible multisets and count matching hands."""
    total_count = 0

    # Generate all possible multisets (counts for each rank)
    for counts in product(range(MAX_COUNT_PER_RANK + 1), repeat=len(RANKS)):
        # Skip empty hand
        if sum(counts) == 0:
            continue

        hand_score = calculate_hand_score(counts)
        cribbage_score = calculate_cribbage_score(counts)

        if hand_score == cribbage_score:
            num_hands = calculate_num_hands(counts)
            total_count += num_hands

    return total_count


def main():
    print("Project Euler Problem 928 Solution")
    print("=" * 50)

    print("Computing solution...")
    result = solve_problem()

    print(result)
    return result


if __name__ == "__main__":
    main()
