"""Project Euler Problem 111: Primes with runs."""

from itertools import combinations, product
from typing import List, Set


def is_prime(num: int) -> bool:
    """Check if a number is prime."""
    if num < 2:
        return False
    if num == 2:
        return True
    if num % 2 == 0:
        return False
    i = 3
    while i * i <= num:
        if num % i == 0:
            return False
        i += 2
    return True


class Euler111:
    """Solver for Problem 111."""

    def __init__(self, n: int) -> None:
        """Initialize with n-digit numbers."""
        self.n = n

    def solve(self) -> int:
        """Solve the problem."""
        total_s_sum = 0

        for d in range(10):  # For each digit d
            s_n_d = 0
            # M(n,d) is the largest k_repeats for which primes exist.
            # We iterate k_repeats from self.n down to 1.
            for k_repeats in range(self.n, 0, -1):
                num_other_digits = self.n - k_repeats

                current_sum_for_this_k = 0
                current_count_for_this_k = 0

                # Get all combinations of positions for the 'd' digit
                # list(range(self.n)) gives [0, 1, ..., self.n-1]
                for d_positions in combinations(range(self.n), k_repeats):
                    # d_positions is a tuple of indices where 'd' will be placed.

                    if num_other_digits == 0:
                        # All digits are 'd'
                        num_str = str(d) * self.n
                        # If d=0, num_str is "00...0", num is 0. is_prime(0) is False.
                        # No explicit 'leading zero' check needed here as d=0 and self.n > 1 makes it non-prime.
                        # Or, if d!=0, it's a repdigit like 11...1.
                        num = int(num_str)
                        if is_prime(num):
                            current_sum_for_this_k += num
                            current_count_for_this_k += 1
                    else:
                        # There are 'num_other_digits' to fill.
                        other_positions = [i for i in range(self.n) if i not in d_positions]

                        candidate_other_digits = [digit for digit in range(10) if digit != d]

                        # Generate all sequences of length num_other_digits from candidate_other_digits
                        for other_digits_sequence in product(candidate_other_digits, repeat=num_other_digits):
                            num_arr = [''] * self.n

                            for pos in d_positions:
                                num_arr[pos] = str(d)

                            for i, pos in enumerate(other_positions):
                                num_arr[pos] = str(other_digits_sequence[i])

                            # Critical check: n-digit numbers cannot start with '0' (unless n=1)
                            if self.n > 1 and num_arr[0] == '0':
                                continue

                            num_str = ''.join(num_arr)
                            num = int(num_str)

                            if is_prime(num):
                                current_sum_for_this_k += num
                                current_count_for_this_k += 1

                if current_count_for_this_k > 0:
                    # These are the primes for M(n,d) = k_repeats.
                    # S(n,d) is the sum of these primes.
                    s_n_d = current_sum_for_this_k
                    # N(n,d) = current_count_for_this_k (not strictly needed for final sum)
                    break  # Found M(n,d) and S(n,d) for this d, so break from k_repeats loop

            total_s_sum += s_n_d

        return total_s_sum


def main() -> int:
    """Main function."""
    # Project Euler Problem 111 specifies n=10
    solver = Euler111(10)
    return solver.solve()


if __name__ == "__main__":
    print(main())
