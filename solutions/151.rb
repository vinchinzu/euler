# Memoization cache for Problem 151
$memo_151 = {}

# Recursive solver for Problem 151
# counts: an array [c1, c2, c3, c4, c5] representing counts of A1 to A5 sheets
def solve_151(counts)
  state_tuple = counts.dup.freeze # Use a frozen copy of the array as a hash key
  return $memo_151[state_tuple] if $memo_151.key?(state_tuple)

  num_sheets = counts.sum
  if num_sheets == 0
    return 0.0 # Base case: envelope empty, no more events.
  end

  # If only one A5 sheet remains, this is a "last batch" scenario.
  # This specific "single sheet" event is not counted towards the expectation.
  if num_sheets == 1 && counts[4] == 1 # counts[4] is c5 (A5 sheets)
    return 0.0
  end

  # Value added by the current state, if it's a countable single sheet event.
  # A "single sheet event" occurs if num_sheets == 1.
  # It's countable if it's not the very first A1 sheet (which is handled by final subtraction)
  # and not the "single A5 sheet remaining" case (which is handled by returning 0.0 above).
  current_event_contribution = 0.0
  if num_sheets == 1
    # This implies it's a single A1, A2, A3, or A4 sheet.
    current_event_contribution = 1.0
  end

  # Expected value from future states
  future_expected_value = 0.0

  if num_sheets == 1
    # This state has a single sheet (A1, A2, A3, or A4). It will be cut.
    # Find which sheet it is (0 for A1, 1 for A2, etc.)
    idx_of_single_sheet = counts.find_index(1)

    new_counts = counts.clone
    new_counts[idx_of_single_sheet] = 0
    new_counts[idx_of_single_sheet + 1] = 2 # Cut into two of the next size

    future_expected_value = solve_151(new_counts)
  else # num_sheets > 1
    # Multiple sheets in the envelope. One is chosen randomly.
    counts.each_with_index do |count_of_ax, idx_ax| # idx_ax: 0 for A1, ..., 4 for A5
      if count_of_ax > 0
        prob_pick_ax = count_of_ax.to_f / num_sheets

        new_counts = counts.clone
        new_counts[idx_ax] -= 1 # Remove the picked sheet

        if idx_ax < 4 # If A1, A2, A3, or A4 was picked (index 0-3)
          new_counts[idx_ax + 1] += 2 # It's cut and two smaller sheets are added
        end
        # If A5 was picked (idx_ax == 4), it's just removed. No new sheets are added from cutting.

        future_expected_value += prob_pick_ax * solve_151(new_counts)
      end
    end
  end

  result = current_event_contribution + future_expected_value
  $memo_151[state_tuple] = result
  return result
end

# Problem 151: A Preference for A5
def problem_151
  initial_counts = [1, 0, 0, 0, 0] # Starts with one A1 sheet

  # The result from solve_151 includes the count for the initial single A1 sheet.
  # The problem asks to exclude this first one.
  # $memo_151 must be cleared for multiple runs if this function were called multiple times.
  # For a single call per script execution, it's fine.
  $memo_151.clear
  raw_expected_value = solve_151(initial_counts)
  final_expected_value = raw_expected_value - 1.0

  final_expected_value # The main block will format it.
end
