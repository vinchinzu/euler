# Problem 135: Same differences
# How many values of n less than one million have exactly ten distinct solutions?
#
# Let the arithmetic progression be X, X-d, X-2d.
# For these to be positive integers:
# X > 0, d > 0 (for distinct terms, if d=0 then x=y=z, n = X^2 - X^2 - X^2 = -X^2, not positive)
# X-2d > 0  => X > 2d
#
# The equation x^2 - y^2 - z^2 = n becomes:
# X^2 - (X-d)^2 - (X-2d)^2 = n
# X^2 - (X^2 - 2Xd + d^2) - (X^2 - 4Xd + 4d^2) = n
# X^2 - X^2 + 2Xd - d^2 - X^2 + 4Xd - 4d^2 = n
# -X^2 + 6Xd - 5d^2 = n
# n = -X^2 + 6Xd - 5d^2
#
# This can be factored. Consider it as a quadratic in X: X^2 - 6dX + (n + 5d^2) = 0.
# Alternatively, the problem statement notes n = (X-d)(5d-X). Let's check:
# (X-d)(5d-X) = 5dX - X^2 - 5d^2 + dX = -X^2 + 6dX - 5d^2. This is correct.
#
# For n > 0, we need (X-d)(5d-X) > 0.
# Since d > 0:
# Case 1: X-d > 0 AND 5d-X > 0  => X > d AND X < 5d.
# Case 2: X-d < 0 AND 5d-X < 0  => X < d AND X > 5d. (Impossible if d>0)
# So we need X > d and X < 5d.
# Combined with X > 2d from the positive integer condition, the overall condition for X is:
# 2d < X < 5d.

limit_n = 1_000_000
solution_counts = Array.new(limit_n, 0)

# Determine the maximum value for d.
# n = (X-d)(5d-X). Let k = X-d. Then X = k+d.
# n = k(5d - (k+d)) = k(4d-k).
# Condition 2d < X < 5d translates to:
# 2d < k+d < 5d  =>  d < k < 4d.
# For a given d, k can range from d+1 to 4d-1.
# n = k(4d-k).
# Smallest n for a given d occurs when k is minimal (d+1) or maximal (4d-1).
# If k = d+1, n = (d+1)(4d-(d+1)) = (d+1)(3d-1).
# If k = 4d-1, n = (4d-1)(4d-(4d-1)) = (4d-1)(1).
# The expression (d+1)(3d-1) grows faster with d.
# So, we need (d+1)(3d-1) < limit_n.
# 3d^2 + 2d - 1 < limit_n
# 3d^2 + 2d - (limit_n + 1) < 0. (Using limit_n, not limit_n - 1 as in draft)
# For limit_n = 1_000_000: 3d^2 + 2d - 1_000_001 < 0.
# Roots of 3d^2 + 2d - 1000001 = 0 are d = (-2 +- sqrt(4 - 4*3*(-1000001))) / 6
# d = (-2 +- sqrt(4 + 12000012)) / 6
# d = (-2 +- sqrt(12000016)) / 6
# d = (-2 +- 3464.1039) / 6
# Positive root: d = (3462.1039) / 6 = 577.0173
# So, d_max = 577.
d_max = 577

(1..d_max).each do |d|
  # Iterate X from 2d+1 up to 5d-1
  # The loop ((2 * d + 1)...(5 * d)) means X from 2d+1 up to 5d-1.
  ((2 * d + 1)...(5 * d)).each do |x_val|
    n = (x_val - d) * (5 * d - x_val)
    if n < limit_n
      solution_counts[n] += 1
    else
      # Optimization: If n is derived from k(4d-k), for a fixed d, n is a parabola in k.
      # It peaks at k=2d (i.e., X=3d).
      # If X = x_val, then k = x_val - d.
      # As x_val goes from 2d+1 to 5d-1, k goes from d+1 to 4d-1.
      # The values of n will be symmetric around X=3.5d (or k=2.5d, but k is integer).
      # If n calculated for current x_val is >= limit_n,
      # and x_val is increasing towards 3.5d, then subsequent n might also be >= limit_n.
      # And once it starts decreasing past 3.5d, if n was already too large, it might become smaller again.
      # Example: d=1. X ranges from 3 to 4.
      # X=3: k=2. n = 2(4-2) = 4.
      # X=4: k=3. n = 3(4-3) = 3.
      # The current simple check `if n < limit_n` is correct and sufficient.
      # The d_max calculation ensures that at least the smallest n for that d is within limits.
    end
  end
end

count_of_tens = 0
# solution_counts[0] is for n=0, which is not considered by problem (positive integers for x,y,z implies n>0)
# The problem asks for "values of n". The array is indexed 0 to limit_n-1.
# n = (X-d)(5d-X). Smallest possible X-d is 1 (when X=d+1, but X must be >2d).
# Smallest X-d is (2d+1)-d = d+1.
# Smallest 5d-X is 1 (when X=5d-1).
# So n is always positive. solution_counts[0] will remain 0.
(1...limit_n).each do |i| # Iterate from n=1 up to limit_n-1
  if solution_counts[i] == 10
    count_of_tens += 1
  end
end

puts count_of_tens
