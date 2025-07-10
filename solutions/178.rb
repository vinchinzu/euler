# Problem 178: Step Numbers

# dp[length][last_digit][mask]
# length: 1 to 40
# last_digit: 0 to 9
# mask: 0 to 1023 (2^10 - 1), bit i set if digit i is used.

# Initialize DP table with zeros
# Ruby's default value for hash entries can be used for sparse DP table
dp = Hash.new { |h, k| h[k] = Hash.new { |h2, k2| h2[k2] = Hash.new(0) } }

# Base cases for length = 1
# A number cannot start with 0.
(1..9).each do |d|
  dp[1][d][1 << d] = 1
end

# Fill DP table
# Iterate for current lengths from 1 up to 39 (to build numbers of length up to 40)
(1..39).each do |len|
  (0..9).each do |current_digit|
    (0...1024).each do |mask| # Iterate through all possible masks
      count = dp[len][current_digit][mask]
      next if count == 0

      # Try to append prev_digit = current_digit - 1
      if current_digit > 0
        prev_digit = current_digit - 1
        new_mask = mask | (1 << prev_digit)
        dp[len + 1][prev_digit][new_mask] += count
      end

      # Try to append next_digit = current_digit + 1
      if current_digit < 9
        next_digit = current_digit + 1
        new_mask = mask | (1 << next_digit)
        dp[len + 1][next_digit][new_mask] += count
      end
    end
  end
end

# Sum pandigital step numbers
# Pandigital means mask is (1<<10)-1 = 1023.
# Length must be at least 10 for a number to be pandigital (contain all 0-9).
# We are looking for numbers less than 10^40, so lengths up to 40.
full_mask = (1 << 10) - 1
total_pandigital_step_numbers = 0

(10..40).each do |len|
  (0..9).each do |d| # Any digit can be the last digit
    total_pandigital_step_numbers += dp[len][d][full_mask]
  end
end

puts total_pandigital_step_numbers
