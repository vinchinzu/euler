# Euler Problem 173: Using up to one million tiles how many different
# square laminae can be formed?
# A square lamina is a square outline with a square "hole" possessing
# vertical and horizontal symmetry.

class Euler173
  TILE_LIMIT = 1_000_000

  def solve
    count = 0

    # Let L_out be the side length of the outer square, and L_in be for the inner (hole).
    # Number of tiles N = L_out^2 - L_in^2.
    # Constraints:
    # 1. L_out > L_in >= 1
    # 2. L_out and L_in must have the same parity for symmetry.
    #    This means (L_out - L_in) is even, and (L_out + L_in) is even.
    # 3. N <= TILE_LIMIT.

    # Let diff = L_out - L_in
    # Let sum_val = L_out + L_in
    # Then N = diff * sum_val.
    # From constraint (2), diff and sum_val must both be even.
    # From constraint (1):
    #   L_out > L_in  => diff >= 2 (since diff is even).
    #   L_in >= 1     => (sum_val - diff) / 2 >= 1 => sum_val - diff >= 2 => sum_val >= diff + 2.

    diff = 2 # Smallest possible even value for (L_out - L_in)

    loop do
      # Smallest possible sum_val for the current diff.
      # sum_val must be even and sum_val >= diff + 2.
      min_sum_val_for_diff = diff + 2

      # If diff * min_sum_val_for_diff already exceeds TILE_LIMIT,
      # then no sum_val can work for this diff.
      # Also, any larger diff will also result in diff * (diff+2) > TILE_LIMIT.
      # So, we can break the outer loop.
      if diff * min_sum_val_for_diff > TILE_LIMIT
        break
      end

      # Determine the maximum possible sum_val for the current diff.
      # sum_val <= TILE_LIMIT / diff.
      # And sum_val must be even.
      max_s_candidate = TILE_LIMIT / diff # Integer division

      actual_max_sum_val_for_diff = if max_s_candidate.odd?
                                     max_s_candidate - 1
                                   else
                                     max_s_candidate
                                   end

      # If the smallest valid sum_val is less than or equal to the largest valid sum_val,
      # then there are laminae to count for this diff.
      if min_sum_val_for_diff <= actual_max_sum_val_for_diff
        # The valid sum_val values form an arithmetic progression:
        # min_sum_val_for_diff, min_sum_val_for_diff + 2, ..., actual_max_sum_val_for_diff
        # Number of terms = (last_term - first_term) / step + 1
        num_laminae_for_this_diff = (actual_max_sum_val_for_diff - min_sum_val_for_diff) / 2 + 1
        count += num_laminae_for_this_diff
      end

      diff += 2 # Move to the next even diff
    end

    count
  end
end

# Main execution block
if __FILE__ == $PROGRAM_NAME
  solver = Euler173.new
  result = solver.solve
  puts result # The script should only print the final answer
end
