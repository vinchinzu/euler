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

puts spiral_sum_formula(1001)

