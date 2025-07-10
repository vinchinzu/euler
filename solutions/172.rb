# Euler Problem 172: Investigating numbers with few repeated digits
# How many 18-digit numbers n (without leading zeros) are there such that
# no digit occurs more than three times in n?

class Euler172
  NUM_LEN = 18
  MAX_FREQ = 3

  # Memoization table
  # Key: An array [idx, frozen_counts_array]
  # Value: count of numbers possible from this state
  MEMO = {}

  # idx: current digit position we are filling (0 to NUM_LEN - 1, from left)
  # counts: An array where counts[d] is the frequency of digit d used so far.
  def solve(idx, counts)
    # Base case: If we have successfully filled all NUM_LEN positions
    if idx == NUM_LEN
      return 1 # This path represents one validly formed number
    end

    # Create an immutable state representation for the memoization key.
    # Using counts.hash or counts.dup.freeze ensures that the array's content,
    # not its object_id, is used for memoization.
    # A simple way is [idx, counts_array_itself] if counts_array is never modified
    # after being used as a key, which is ensured by new_counts = counts.dup.
    # However, freezing is safer to guarantee immutability for the key.
    state_key = [idx, counts.dup.freeze]

    return MEMO[state_key] if MEMO.key?(state_key)

    current_number_of_ways = 0

    # Iterate through possible digits (0-9) to place at the current position 'idx'
    (0..9).each do |digit|
      # Constraint 1: No leading zeros.
      # If it's the first position (idx == 0) and the digit is 0, this is not allowed.
      if idx == 0 && digit == 0
        next
      end

      # Constraint 2: Digit frequency.
      # If the count of the current digit is already at MAX_FREQ, we cannot use it again.
      if counts[digit] < MAX_FREQ
        # If the digit choice is valid, prepare for the recursive call
        new_counts = counts.dup # Duplicate the counts array for modification
        new_counts[digit] += 1  # Increment the frequency of the chosen digit

        # Recursively call solve for the next position with updated counts
        current_number_of_ways += solve(idx + 1, new_counts)
      end
    end

    # Store the computed result in the memoization table
    MEMO[state_key] = current_number_of_ways
    return current_number_of_ways
  end

  def calculate_total_count
    # Initial state for the DP:
    # idx = 0 (starting at the first digit of the 18-digit number)
    # counts = [0, 0, ..., 0] (an array of 10 zeros, as no digits have been used yet)
    initial_counts = Array.new(10, 0)
    solve(0, initial_counts)
  end
end

# Main execution block
if __FILE__ == $PROGRAM_NAME
  solver = Euler172.new
  result = solver.calculate_total_count
  puts result # The script should only print the final answer
end
