# Problem 28: Number spiral diagonals
# Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
#
#   21 22 23 24 25
#   20  7  8  9 10
#   19  6  1  2 11
#   18  5  4  3 12
#   17 16 15 14 13
#
# It can be verified that the sum of the numbers on the diagonals is 101.
#
# What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

# This script provides two solutions:
# 1. spiral_sum_formula(n): A direct mathematical formula.
# 2. spiral(n): An iterative approach that simulates the spiral generation.

require 'benchmark'

# Calculates the sum of the numbers on the diagonals of an n x n spiral
# using a direct mathematical formula.
# n must be an odd number.
def spiral_sum_formula(n)
  return 1 if n == 1 # Base case for a 1x1 spiral.

  # Let M be the number of layers around the central '1'.
  # For an n x n spiral, M = (n - 1) / 2.
  m = (n - 1) / 2

  # The four corners of each layer k (where k goes from 1 to M) are:
  # Top-right (TR): (2k+1)^2 = 4k^2 + 4k + 1
  # Top-left (TL): (2k+1)^2 - 2k = 4k^2 + 2k + 1          (difference of 2k from TR)
  # Bottom-left (BL): (2k+1)^2 - 4k = 4k^2 + 1           (difference of 2k from TL)
  # Bottom-right (BR): (2k+1)^2 - 6k = 4k^2 - 2k + 1      (difference of 2k from BL)
  # The sum of these four corners for layer k is:
  # Sum_corners(k) = (4k^2 + 4k + 1) + (4k^2 + 2k + 1) + (4k^2 + 1) + (4k^2 - 2k + 1)
  #                = 16k^2 + 4k + 4
  #
  # The total sum of diagonals is the central '1' plus the sum of corners for all layers from k=1 to M.
  # Total Sum = 1 + Σ_{k=1 to M} (16k^2 + 4k + 4)
  # This can be broken down using sum formulas:
  # Σ k^2 = M(M+1)(2M+1)/6
  # Σ k   = M(M+1)/2
  # Σ c   = cM
  # Total Sum = 1 + 16 * [M(M+1)(2M+1)/6] + 4 * [M(M+1)/2] + 4 * M
  
  sum_k_squared = m * (m + 1) * (2 * m + 1) / 6
  sum_k = m * (m + 1) / 2
  sum_constant_term = 4 * m
  
  total_sum = 1 + (16 * sum_k_squared) + (4 * sum_k) + sum_constant_term
  total_sum
end

# Iterative solution that simulates the spiral generation to sum diagonal numbers.
def spiral(n)
  i = 1     # Represents the current number being considered on a diagonal. Starts at the center '1'.
  sum = 1   # Accumulator for the sum of diagonal numbers. Initialized with the center '1'.
  j = 2     # Represents the increment to get to the next diagonal number in the current layer.
            # For the first layer (3x3 around the '1'), numbers are 3,5,7,9 (increment is 2).
            # For the second layer (5x5 around the 3x3), numbers are 13,17,21,25 (increment is 4).
            # So, j increases by 2 for each new layer.
 
  # Loop continues as long as 'i' (current diagonal number) is less than the maximum number in an n x n spiral (n^2).
  # Effectively, this means we are still generating layers.
  while i < n**2 
    # Each layer adds 4 diagonal numbers.
    1.upto(4) do |x|
      i += j    # Move to the next diagonal number.
      sum += i  # Add it to the sum.
    end
    j += 2 # For the next layer, the increment between diagonal numbers increases by 2.
  end
  sum
end

# Benchmark section to compare the performance and output of both methods.
Benchmark.bm do |x|
 x.report("Iterative: ") {
   puts spiral(1001)
 }
 x.report("Formula:   ") { # Added extra spaces for alignment if desired
   puts spiral_sum_formula(1001)
 }
end

