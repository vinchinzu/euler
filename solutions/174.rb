# Euler Problem 174: Counting the number of "hollow" square laminae of a particular type
# A square lamina of type L(n) is one formed with t tiles, where there are
# exactly n distinct ways to form a lamina with t tiles.
# We need to find sum_{n=1 to 10} N(n), where N(n) is the number of t <= 1,000,000
# such that t is of type L(n).

class Euler174
  TILE_LIMIT = 1_000_000
  MAX_N_TYPE = 10 # We are interested in N(n) for n = 1 to 10

  def solve
    # tile_ways_counts will store: number_of_tiles (t) => count_of_ways_to_form_t
    # e.g., if t=24 can be formed by (L1_out,L1_in) and (L2_out,L2_in), then tile_ways_counts[24] = 2.
    tile_ways_counts = Hash.new(0)

    # Step 1: Populate tile_ways_counts
    # Iterate through possible (diff, sum_val) pairs.
    # diff = L_out - L_in
    # sum_val = L_out + L_in
    # Number of tiles t = diff * sum_val
    # Constraints:
    #   diff is even, diff >= 2
    #   sum_val is even, sum_val >= diff + 2
    #   t <= TILE_LIMIT

    diff = 2 # Smallest possible even value for (L_out - L_in)
    loop do
      # Smallest possible sum_val for the current diff.
      min_sum_val_for_diff = diff + 2

      # If diff * min_sum_val_for_diff already exceeds TILE_LIMIT, then no
      # sum_val can work for this diff (or any larger diff).
      break if diff * min_sum_val_for_diff > TILE_LIMIT

      sum_val = min_sum_val_for_diff
      loop do
        num_tiles = diff * sum_val
        break if num_tiles > TILE_LIMIT # Current sum_val (and larger) makes t too big

        # This (diff, sum_val) pair forms a lamina with 'num_tiles' tiles.
        # Increment the count of ways this 'num_tiles' can be formed.
        tile_ways_counts[num_tiles] += 1

        sum_val += 2 # Move to the next even sum_val
      end
      diff += 2 # Move to the next even diff
    end

    # Step 2: Calculate N(n) for n = 1 to MAX_N_TYPE
    # N_counts[n] will store N(n), i.e., how many t values are of type L(n).
    # N_counts is 0-indexed; N_counts[0] is unused. Size is MAX_N_TYPE + 1.
    n_counts = Array.new(MAX_N_TYPE + 1, 0)

    tile_ways_counts.each_value do |ways_for_t|
      # 'ways_for_t' is the number of distinct ways a lamina with t tiles can be formed.
      # This corresponds to 'n' in "type L(n)".
      # We are interested if this 'n' is between 1 and MAX_N_TYPE (inclusive).
      if ways_for_t >= 1 && ways_for_t <= MAX_N_TYPE
        n_counts[ways_for_t] += 1
      end
    end

    # Step 3: Calculate the final sum: sum_{n=1 to MAX_N_TYPE} N(n)
    total_sum = 0
    (1..MAX_N_TYPE).each do |n|
      total_sum += n_counts[n]
    end

    total_sum
  end
end

# Main execution block
if __FILE__ == $PROGRAM_NAME
  solver = Euler174.new
  result = solver.solve
  puts result # The script should only print the final answer
end
