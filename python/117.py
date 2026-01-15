"""Project Euler Problem 117: Mixed-colour oblong tiles."""

from typing import List


class Euler117:
    """Solver for Problem 117."""

    def __init__(self, length: int) -> None:
        """Initialize with row length."""
        self.length = length
        # dp[i] will store the number of ways to tile a row of length i.
        # Initialize with 0s. dp[0] will be the base case.
        self.dp: List[int] = [0] * (length + 1)

    def solve(self) -> int:
        """Solve the problem."""
        # Base case: There is one way to tile a row of length 0 (the empty row).
        self.dp[0] = 1

        # Iterate from length 1 up to the target length
        for i in range(1, self.length + 1):
            # Initialize current dp_val to 0 for summation
            current_dp_i_val = 0

            # Option 1: The last tile is a grey square (length 1).
            # Builds upon a valid tiling of length i-1.
            if i >= 1:
                current_dp_i_val += self.dp[i - 1]

            # Option 2: The last tile is a red oblong (length 2).
            # Builds upon a valid tiling of length i-2.
            if i >= 2:
                current_dp_i_val += self.dp[i - 2]

            # Option 3: The last tile is a green oblong (length 3).
            # Builds upon a valid tiling of length i-3.
            if i >= 3:
                current_dp_i_val += self.dp[i - 3]

            # Option 4: The last tile is a blue oblong (length 4).
            # Builds upon a valid tiling of length i-4.
            if i >= 4:
                current_dp_i_val += self.dp[i - 4]

            self.dp[i] = current_dp_i_val

        # The final answer is stored in dp[target_length]
        return self.dp[self.length]


def main() -> int:
    """Main function."""
    ROW_LENGTH = 50
    solver = Euler117(ROW_LENGTH)
    return solver.solve()


if __name__ == "__main__":
    print(main())
