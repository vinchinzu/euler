# Problem 136: Singleton difference
# How many values of n less than fifty million have exactly one solution?
#
# The setup is identical to Problem 135:
# Let the arithmetic progression be X_val, X_val-d, X_val-2d.
# These must be positive integers, so d > 0 and X_val > 2d.
# The equation x^2 - y^2 - z^2 = n simplifies to n = (X_val-d)(5d-X_val).
# For n to be positive, and combined with X_val > 2d, we need 2d < X_val < 5d.

limit_n = 50_000_000
# Initialize an array to store the count of solutions for each n.
# Array elements are initialized to 0 by default with this syntax.
solution_counts = Array.new(limit_n, 0)

# Determine d_max:
# Smallest n for a given d is (d+1)(3d-1) (when X_val = 2d+1 or X_val = 5d-1, after transformation).
# We need (d+1)(3d-1) < limit_n.
# For limit_n = 50,000,000:
# 3d^2 + 2d - 1 < 50,000,000
# 3d^2 + 2d - 50,000,001 < 0
# Using the quadratic formula: d = (-2 + sqrt(4 - 4*3*(-50000001))) / 6
# d = (-2 + sqrt(4 + 600000012)) / 6
# d = (-2 + sqrt(600000016)) / 6
# d = (-2 + 24494.89755) / 6
# d = 24492.89755 / 6 = 4082.14959
# So, d_max = 4082.
# Check:
# For d = 4082: (4082+1)(3*4082-1) = 4083 * (12246-1) = 4083 * 12245 = 49998535 (which is < 50M)
# For d = 4083: (4083+1)(3*4083-1) = 4084 * (12249-1) = 4084 * 12248 = 50023072 (which is >= 50M)
# So, d_max = 4082 is correct.
d_max = 4082

(1..d_max).each do |d|
  # X_val ranges from 2d+1 to 5d-1.
  # Ruby range a...b means up to b-1. So (2*d + 1)...(5*d) is correct.
  ((2 * d + 1)...(5 * d)).each do |x_val|
    n = (x_val - d) * (5 * d - x_val)
    
    # Ensure n is within the desired limit before incrementing its count.
    if n < limit_n
      solution_counts[n] += 1
    else
      # Optimization note from P135:
      # For a fixed d, n as a function of x_val is a parabola opening downwards,
      # peaking near x_val = 3.5d.
      # If x_val is increasing towards 3.5d and n already exceeds limit_n,
      # then subsequent x_val up to 3.5d will also give n >= limit_n.
      # However, x_val past 3.5d will give decreasing n values.
      # The simple `if n < limit_n` check is correct and sufficient.
      # The d_max calculation ensures that the smallest possible n for a given d is within limits,
      # so there will always be some x_val for which n < limit_n.
    end
  end
end

count_of_ones = 0
# n must be a positive integer, so we check solution_counts from index 1.
# "values of n less than fifty million" means n from 1 to 49,999,999.
# The loop (1...limit_n) covers indices 1 to limit_n - 1.
(1...limit_n).each do |n_val|
  if solution_counts[n_val] == 1
    count_of_ones += 1
  end
end

puts count_of_ones
