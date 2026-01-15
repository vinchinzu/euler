"""Project Euler Problem 172: Investigating numbers with few repeated digits."""

from typing import List

N_DIGITS = 18
BASE = 4  # Base for state encoding (counts 0-3)
NUM_DIGITS = 10  # Digits 0-9
NUM_STATES = BASE ** NUM_DIGITS  # 4^10 = 1,048,576 states


def extract_count(state: int, digit: int, base: int, powers: List[int]) -> int:
    """Extract count for a specific digit from state."""
    return (state // powers[digit]) % base


def next_state(state: int, digit: int, powers: List[int]) -> int:
    """Compute next state after incrementing a digit's count."""
    return state + powers[digit]


def compute_valid_numbers(
    n_digits: int, base: int, num_digits: int, num_states: int, powers: List[int]
) -> int:
    """Main DP computation."""
    # dp[pos][state] = number of ways to fill first 'pos' digits with given state
    dp: List[List[int]] = [[0] * num_states for _ in range(n_digits + 1)]

    # Base case: before placing any digits, state 0 has 1 way
    dp[0][0] = 1

    # Fill DP table
    for pos in range(n_digits):
        for state in range(num_states):
            if dp[pos][state] == 0:
                continue

            # Try each possible digit
            for digit in range(10):
                # Skip leading zero
                if pos == 0 and digit == 0:
                    continue

                # Check if we can use this digit (count < 3)
                count = extract_count(state, digit, base, powers)
                if count >= 3:
                    continue

                # Compute new state and update
                new_state = next_state(state, digit, powers)
                dp[pos + 1][new_state] += dp[pos][state]

    # Sum all valid states at final position
    return sum(dp[n_digits])


def main() -> int:
    """Main function."""
    # Precompute powers of BASE for efficient state manipulation
    powers = [BASE ** d for d in range(NUM_DIGITS)]
    return compute_valid_numbers(N_DIGITS, BASE, NUM_DIGITS, NUM_STATES, powers)


if __name__ == "__main__":
    print(main())
