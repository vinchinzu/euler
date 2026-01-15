#!/usr/bin/env python3
"""
Project Euler Problem 62: Cubic permutations

The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3)
and 66430125 (405^3). In fact, 41063625 is the smallest cube which has exactly
three permutations of its digits which are also cube.

Find the smallest cube for which exactly five permutations of its digits are cube.
"""


class CubicPermutationsSolver:
    def __init__(self, target_perms_count):
        """
        Initializes solver for finding the smallest cube that is one of a set
        of 'target_perms_count' cubes, all of which are permutations of each other's digits.

        @param target_perms_count [Integer] The exact number of permutations (cubes) required.
        """
        self.target_perms_count = target_perms_count
        self.min_overall_solution = float('inf')

    def solve(self):
        """
        Solves the problem by iterating through cubes n^3.

        The process:
        1. Generate n^3 for n = 1, 2, 3, ...
        2. Cubes are processed in batches based on their number of digits.
           When the number of digits of n^3 increases, it signifies completion
           of a batch of cubes with the previous number of digits.

        3. This completed batch (stored in `cubes_by_signature_current_digits`) is then processed:
           a. For each signature group in batch, if its size is exactly `target_perms_count`,
              its smallest cube (the first one added to its list) is a candidate for solution.
           b. `min_overall_solution` is updated if this candidate is smaller.

        4. After processing a batch, storage for that batch is cleared.

        5. Stopping condition:
           Once a `min_overall_solution` is found, let its number of digits be D_sol.
           If the current batch of cubes being generated (n^3) has more than D_sol digits,
           we can stop. Any further cubes or groups of cubes found will be larger than
           `min_overall_solution` or have more digits.

        @return [Integer] The smallest cube satisfying the condition, or float('inf') if none found.
        """
        n = 0
        # Stores cubes for current number of digits being processed.
        # Key: signature (sorted string of digits), Value: list of cubes.
        # This hash is cleared when the number of digits of n^3 changes.
        cubes_by_signature_current_digits = {}

        # Tracks the number of digits for cubes in the current_batch.
        # Initialized to 0; will be set to length of 1^3 in the first iteration.
        current_batch_digit_length = 0

        while True:
            n += 1
            cube = n ** 3
            s_cube = str(cube)
            num_digits = len(s_cube)

            if num_digits > current_batch_digit_length:
                # The new cube (n^3) has more digits than those in the current batch.
                # This signifies that the batch for `current_batch_digit_length` is complete.
                # Process this completed batch.

                if current_batch_digit_length > 0:
                    # Avoid processing if current_batch_digit_length was 0 (initial state)
                    self._process_completed_batch(cubes_by_signature_current_digits)
                    cubes_by_signature_current_digits.clear()

                # Update to the new number of digits for the next batch.
                current_batch_digit_length = num_digits

                # Check stopping condition:
                # If a solution has been found (`min_overall_solution` is not Inf),
                # and the cubes in the new batch (current_batch_digit_length)
                # already have more digits than this solution, then we can stop.
                # Any further cubes or groups of cubes found will be larger than
                # `min_overall_solution` or have more digits.
                if self.min_overall_solution != float('inf') and \
                   current_batch_digit_length > len(str(self.min_overall_solution)):
                    break

            # Add current cube to the batch for its number of digits.
            signature = ''.join(sorted(s_cube))
            cubes_by_signature_current_digits[signature] = cubes_by_signature_current_digits.get(signature, [])
            cubes_by_signature_current_digits[signature].append(cube)

        # The loop terminates when current_batch_digit_length exceeds the digit length
        # of a found solution. The batch corresponding to `min_overall_solution.to_s.length`
        # would have been processed just before `current_batch_digit_length` was incremented
        # to be too large.
        # If no solution is found (e.g. target_perms_count is very high),
        # `min_overall_solution` remains float('inf').

        return self.min_overall_solution

    def _process_completed_batch(self, cubes_map):
        """
        Processes a completed batch of cubes (all having the same number of digits).
        Updates `min_overall_solution` if a valid group with `target_perms_count` is found.

        @param cubes_map [Hash] A hash where keys are digit signatures and values are lists of cubes.
        """
        for list_of_cubes in cubes_map.values():
            # Check if this group has exactly the target number of permutable cubes.
            if len(list_of_cubes) == self.target_perms_count:
                # The first cube added to this list is the smallest numerically for this group,
                # because we generate cubes by iterating n upwards (n=1, 2, 3...).
                smallest_cube_in_group = list_of_cubes[0]

                # If this is the first solution found, or if this group's smallest cube
                # is smaller than the current overall minimum, update it.
                if self.min_overall_solution == float('inf') or \
                   smallest_cube_in_group < self.min_overall_solution:
                    self.min_overall_solution = smallest_cube_in_group


def main():
    solver = CubicPermutationsSolver(5)
    result = solver.solve()

    print(result)


if __name__ == "__main__":
    main()
