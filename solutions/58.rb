require 'prime'

# Efficiently computes the side length of the square spiral for which
# the ratio of primes along both diagonals first falls below 10%.
def solve_problem_58
  prime_count = 0
  total_diagonals = 1
  side_length = 1

  loop do
    side_length += 2
    # The four corners for the current layer
    corners = [
      side_length**2 - 0 * (side_length - 1),
      side_length**2 - 1 * (side_length - 1),
      side_length**2 - 2 * (side_length - 1),
      side_length**2 - 3 * (side_length - 1)
    ]
    # Exclude the bottom-right corner (side_length**2), already counted in previous layer
    corners.shift
    prime_count += corners.count { |n| Prime.prime?(n) }
    total_diagonals += 4
    ratio = prime_count.to_f / total_diagonals
    return side_length if ratio < 0.10
  end
end

puts solve_problem_58