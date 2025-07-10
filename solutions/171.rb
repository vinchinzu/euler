require 'set'

# Find the last nine digits of the sum of all n, 0 < n < 10^20,
# such that f(n) (the sum of the squares of the digits of n) is a perfect square.

class Euler171
  MAX_DIGITS = 20
  # We need the sum modulo 10^9 for the last nine digits.
  MOD = 1_000_000_000

  # Maximum possible sum of squares of digits for a 20-digit number:
  # 20 * (9^2) = 20 * 81 = 1620.
  MAX_SUM_SQ = MAX_DIGITS * 9 * 9

  # Precompute powers of 10 modulo MOD
  # POWERS_OF_10[i] stores 10^(MAX_DIGITS - 1 - i) % MOD
  # POWERS_OF_10[0] = 10^19 % MOD (for the leftmost digit)
  # POWERS_OF_10[19] = 10^0 % MOD = 1 (for the rightmost digit)
  POWERS_OF_10 = Array.new(MAX_DIGITS)

  # Precompute perfect squares up to MAX_SUM_SQ
  PERFECT_SQUARES = Set.new

  # Memoization table for the DP states
  MEMO = {}

  def initialize
    # Initialize POWERS_OF_10
    # POWERS_OF_10[idx] corresponds to the place value of the digit at index idx (0-indexed from left)
    # Example for MAX_DIGITS = 3: POWERS_OF_10 should be [100, 10, 1]
    # POWERS_OF_10[0] = 10^(3-1-0) = 10^2 = 100
    # POWERS_OF_10[1] = 10^(3-1-1) = 10^1 = 10
    # POWERS_OF_10[2] = 10^(3-1-2) = 10^0 = 1
    current_power = 1
    POWERS_OF_10[MAX_DIGITS - 1] = current_power # For 10^0 (rightmost digit)
    (MAX_DIGITS - 2).downto(0) do |i| # Iterate from MAX_DIGITS-2 down to 0
      current_power = (current_power * 10) % MOD
      POWERS_OF_10[i] = current_power
    end

    # Initialize PERFECT_SQUARES
    i = 1
    loop do
      sq = i * i
      break if sq > MAX_SUM_SQ
      PERFECT_SQUARES.add(sq)
      i += 1
    end
  end

  # dp_solve(digit_idx, current_sum_sq, is_tight, is_started)
  # - digit_idx: Current digit position being filled (0 to MAX_DIGITS-1, from left).
  # - current_sum_sq: Sum of squares of digits chosen so far.
  # - is_tight: Boolean, true if we are restricted by the digits of the upper limit N (here, N = 10^20-1, so "99...9").
  #             If true, current digit can go up to N_str[digit_idx]. If false, up to 9.
  #             (Since N_str is all 9s, limit is always 9. is_tight tracks if we are *still* on the "all 9s" path).
  # - is_started: Boolean, true if at least one non-zero digit has been placed.
  #               This ensures we count n > 0 and handles f(0) correctly.
  # Returns: [count, sum_of_numbers_mod_MOD]
  #   - count: How many such numbers can be formed from this state.
  #   - sum_of_numbers_mod_MOD: The sum (modulo MOD) of these numbers.
  def dp_solve(idx, sum_sq, tight, started)
    # Base case: All digits have been placed
    if idx == MAX_DIGITS
      if started && PERFECT_SQUARES.include?(sum_sq)
        return [1, 0] # Found one valid number. Its sum is built by callers.
      else
        return [0, 0] # Not a valid number or f(n) is not a perfect square.
      end
    end

    # Memoization
    state = [idx, sum_sq, tight, started]
    return MEMO[state] if MEMO.key?(state)

    total_count_from_this_state = 0
    total_sum_from_this_state = 0

    upper_digit_limit = tight ? 9 : 9

    (0..upper_digit_limit).each do |digit|
      new_tight = tight && (digit == upper_digit_limit)

      if !started && digit == 0
        # Placing a leading zero; the number hasn't effectively started yet.
        # current_sum_sq does not change (it's 0 if only leading zeros so far).
        # is_started remains false.
        count, sum_val = dp_solve(idx + 1, sum_sq, new_tight, false)

        total_count_from_this_state += count
        total_sum_from_this_state = (total_sum_from_this_state + sum_val) % MOD
      else
        # Placing a non-zero digit, or continuing a number that has already started.
        new_sum_sq = sum_sq + digit * digit

        # Pruning: if new_sum_sq already exceeds max possible, no solution from here.
        next if new_sum_sq > MAX_SUM_SQ

        # is_started becomes true because digit > 0 or started was already true.
        count, sum_val = dp_solve(idx + 1, new_sum_sq, new_tight, true)

        if count > 0
          total_count_from_this_state += count

          # Contribution of the current digit to the sum of numbers:
          # current_digit_value * number_of_ways_to_complete_suffix
          # POWERS_OF_10[idx] is 10^(MAX_DIGITS - 1 - idx) % MOD
          term_value_for_this_digit = (digit * POWERS_OF_10[idx]) % MOD

          sum_contribution_from_current_digit = (term_value_for_this_digit * count) % MOD

          total_sum_from_this_state = (total_sum_from_this_state + sum_val + sum_contribution_from_current_digit) % MOD
        end
      end
    end

    MEMO[state] = [total_count_from_this_state, total_sum_from_this_state]
    return MEMO[state]
  end

  def solve
    # Initial call:
    # Start at digit index 0.
    # Current sum of squares is 0.
    # is_tight is true (we are bound by "99...9" initially, representing numbers < 10^20).
    # is_started is false (no non-zero digit placed yet).
    _count, final_sum = dp_solve(0, 0, true, false)
    final_sum
  end
end

# Main execution
if __FILE__ == $PROGRAM_NAME
  solver = Euler171.new
  result = solver.solve
  puts result
end
