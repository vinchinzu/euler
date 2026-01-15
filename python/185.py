"""Project Euler Problem 185: Number Mind."""

from typing import List, Tuple
import random

GUESSES: List[Tuple[str, int]] = [
    ("5616185650518293", 2),
    ("3847439647293047", 1),
    ("5855462940810587", 3),
    ("9742855507068353", 3),
    ("4296849643607543", 3),
    ("3174248439465858", 1),
    ("4513559094146117", 2),
    ("7890971548908067", 3),
    ("8157356344118483", 1),
    ("2615250744386899", 2),
    ("8690095851526254", 3),
    ("6375711915077050", 1),
    ("6913859173121360", 1),
    ("6442889055042768", 2),
    ("2321386104303845", 0),
    ("2326509471271448", 2),
    ("5251583379644322", 2),
    ("1748270476758276", 3),
    ("4895722652190306", 1),
    ("3041631117224635", 3),
    ("1841236454324589", 3),
    ("2659862637316867", 2),
]

DIGITS = len(GUESSES[0][0])
NUM_GUESSES = len(GUESSES)

PARSED_GUESSES: List[Tuple[List[int], int]] = [
    ([int(c) for c in digits], count) for digits, count in GUESSES
]
TARGETS = [count for _, count in PARSED_GUESSES]

MATCH_TABLE: List[List[List[int]]] = [
    [
        [1 if digits[pos] == digit else 0 for digits, _ in PARSED_GUESSES]
        for digit in range(10)
    ]
    for pos in range(DIGITS)
]


def random_sequence(rng: random.Random) -> List[int]:
    """Generate random sequence."""
    return [rng.randint(0, 9) for _ in range(DIGITS)]


def main() -> str:
    """Main function."""
    rng = random.Random(2024)
    sequence = random_sequence(rng)
    matches = [0] * NUM_GUESSES
    contrib = [0] * NUM_GUESSES

    for pos in range(DIGITS):
        digit = sequence[pos]
        for gi in range(NUM_GUESSES):
            if MATCH_TABLE[pos][digit][gi] == 1:
                matches[gi] += 1

    for gi in range(NUM_GUESSES):
        contrib[gi] = abs(matches[gi] - TARGETS[gi])

    current_cost = sum(contrib)

    best_sequence = sequence.copy()
    best_cost = current_cost

    max_attempts = 200
    max_iterations = 6_000
    attempt = 0

    while current_cost > 0 and attempt < max_attempts:
        iterations = 0
        while current_cost > 0 and iterations < max_iterations:
            improved = False

            positions = list(range(DIGITS))
            rng.shuffle(positions)

            for pos in positions:
                original_digit = sequence[pos]
                original_matches = MATCH_TABLE[pos][original_digit]

                best_digit = original_digit
                best_delta = 0

                for trial_digit in range(10):
                    if trial_digit == original_digit:
                        continue

                    delta_cost = 0
                    for gi in range(NUM_GUESSES):
                        old_match = matches[gi]
                        old_contribution = contrib[gi]

                        delta_match = (
                            MATCH_TABLE[pos][trial_digit][gi] - original_matches[gi]
                        )
                        if delta_match == 0:
                            continue

                        new_match = old_match + delta_match
                        new_contribution = abs(new_match - TARGETS[gi])
                        delta_cost += new_contribution - old_contribution

                    if delta_cost < best_delta:
                        best_delta = delta_cost
                        best_digit = trial_digit

                if best_digit == original_digit:
                    continue

                # Apply best digit change
                for gi in range(NUM_GUESSES):
                    old_match = matches[gi]
                    matches[gi] += (
                        MATCH_TABLE[pos][best_digit][gi]
                        - MATCH_TABLE[pos][original_digit][gi]
                    )
                    contrib[gi] = abs(matches[gi] - TARGETS[gi])

                sequence[pos] = best_digit
                current_cost += best_delta
                improved = True

                if current_cost < best_cost:
                    best_cost = current_cost
                    best_sequence = sequence.copy()

                if current_cost == 0:
                    break

            if current_cost == 0:
                break

            if not improved:
                # Random perturbation
                for _ in range(2):
                    pos = rng.randint(0, DIGITS - 1)
                    new_digit = rng.randint(0, 9)
                    if new_digit == sequence[pos]:
                        continue

                    old_digit = sequence[pos]
                    for gi in range(NUM_GUESSES):
                        matches[gi] += (
                            MATCH_TABLE[pos][new_digit][gi]
                            - MATCH_TABLE[pos][old_digit][gi]
                        )
                        contrib[gi] = abs(matches[gi] - TARGETS[gi])
                    sequence[pos] = new_digit

                current_cost = sum(contrib)
                if current_cost < best_cost:
                    best_cost = current_cost
                    best_sequence = sequence.copy()

            iterations += 1

        if current_cost == 0:
            break

        # Restart
        sequence = random_sequence(rng)
        matches = [0] * NUM_GUESSES
        contrib = [0] * NUM_GUESSES
        for pos in range(DIGITS):
            digit = sequence[pos]
            for gi in range(NUM_GUESSES):
                matches[gi] += MATCH_TABLE[pos][digit][gi]

        for gi in range(NUM_GUESSES):
            contrib[gi] = abs(matches[gi] - TARGETS[gi])
        current_cost = sum(contrib)
        attempt += 1

    if current_cost == 0:
        return "".join(str(d) for d in sequence)
    else:
        return "".join(str(d) for d in best_sequence)


if __name__ == "__main__":
    print(main())
