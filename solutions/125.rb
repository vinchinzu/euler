# Problem 125: Palindromic Sums
#
# Problem Statement:
# The palindromic number $595$ is interesting because it can be written as the
# sum of consecutive squares: $6^2 + 7^2 + 8^2 + 9^2 + 10^2 + 11^2 + 12^2$.
#
# There are exactly eleven palindromes below one-thousand that can be written
# as consecutive square sums, and the sum of these palindromes is $4164$. Note
# that $1 = 0^2 + 1^2$ has not been included as this problem is concerned with
# the squares of positive integers.
#
# Find the sum of all the numbers less than $10^8$ that are both palindromic
# and can be written as the sum of consecutive squares.
#
# Notes:
# The problem requires finding the sum of numbers < 10^8 that are palindromic and sums of at least two consecutive positive squares.
# The script iterates through possible starting squares (i*i).
# For each, it sums subsequent squares (j*j) until the sum exceeds 10^8.
# Each sum is checked for being a palindrome. A Set is used to store unique palindromic sums found.
# Finally, all numbers in the set are summed up.
# The solution found is 2906969179.

# Full Ruby script content from temp_problem_125.rb:

require 'set' # Required for using Set data structure

class Problem125Solver
  MAX_SUM_LIMIT = 10**8 # Numbers must be less than 10^8

  # Helper function to check if a number is a palindrome
  def is_palindrome?(num)
    num_str = num.to_s
    num_str == num_str.reverse
  end

  def solve
    # Use a Set to store unique palindromic sums found
    palindromic_sums_found = Set.new

    # Outer loop for the starting number 'i' of the sequence of squares.
    # The loop for 'i' can stop when i*i + (i+1)*(i+1) >= MAX_SUM_LIMIT,
    # as this is the smallest possible sum of at least two consecutive squares starting with i.
    # 2*i^2 + 2*i + 1 >= MAX_SUM_LIMIT. Approx. i_limit = sqrt(MAX_SUM_LIMIT / 2).
    i_upper_bound = Math.sqrt(MAX_SUM_LIMIT / 2.0).to_i
    # If i_upper_bound is such that i_upper_bound^2 + (i_upper_bound+1)^2 >= MAX_SUM_LIMIT,
    # then i_upper_bound might be one too high or just right.
    # For i = 1 up to this calculated bound.
    (1..i_upper_bound).each do |i|
      current_sum_of_squares = i * i # Start sum with the first square i^2

      # Inner loop for the subsequent number 'j' in the sequence of squares.
      # The sum must include at least two squares, so j starts from i+1.
      (i + 1..MAX_SUM_LIMIT).each do |j| # Upper bound for j is loose; break condition is key
        current_sum_of_squares += j * j # Add the next square j^2

        # If the current sum exceeds or equals MAX_SUM_LIMIT, break this inner loop.
        # No further sums starting with i^2 and ending beyond j^2 will be valid.
        if current_sum_of_squares >= MAX_SUM_LIMIT
          break
        end

        # Check if the current sum is a palindrome.
        if is_palindrome?(current_sum_of_squares)
          palindromic_sums_found.add(current_sum_of_squares)
        end
      end
    end

    # Calculate the sum of all unique palindromic numbers found.
    total_sum = 0
    palindromic_sums_found.each { |pal_sum| total_sum += pal_sum }
    # Or simply: total_sum = palindromic_sums_found.sum

    # The script must puts this sum.
    puts total_sum
  end
end

# Create an instance of the solver and run the solve method.
solver = Problem125Solver.new
solver.solve

