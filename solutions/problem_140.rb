# Problem 140: Modified Fibonacci golden nuggets
#
# The problem statement gives AG(x) = x(1+2x)/(1-x-3x^2) = N.
# For x to be rational, the discriminant of (3N+2)x^2 + (N+1)x - N = 0 must be a square.
# D = (N+1)^2 - 4(3N+2)(-N) = (N+1)^2 + 4N(3N+2)
#   = N^2+2N+1 + 12N^2+8N = 13N^2+10N+1 = k^2
# Multiplying by 13: (13N+5)^2 - 25 + 13 = 13k^2
# (13N+5)^2 - 12 = 13k^2
# (13N+5)^2 - 13k^2 = 12. This is a Pell-type equation.
#
# The problem text indicates that solutions (N) are found from X values
# satisfying X^2 - 5Y^2 = 44, where N = (X-7)/5.
# We need X > 7 and X === 2 (mod 5) for N to be a positive integer.
#
# The fundamental solution to u^2 - 5v^2 = 1 is (u1,v1) = (9,4).
# Solutions (X_k, Y_k) to X^2 - 5Y^2 = 44 can be generated from fundamental solutions
# to X^2 - 5Y^2 = 44 using transformations involving (u1,v1).
# If (X_0, Y_0) is a base solution to X^2 - 5Y^2 = 44, then further solutions are:
# X_{k+1} + Y_{k+1}sqrt(5) = (X_k + Y_k sqrt(5))(u_1 + v_1 sqrt(5))^n
# Using (u_1,v_1)=(9,4) for n=1:
# X_1 = 9X_0 + 20Y_0
# Y_1 = 4X_0 + 9Y_0
#
# The recurrence for X values is X_{k+1} = 2*u1*X_k - X_{k-1} = 18*X_k - X_{k-1}.
# (This means X_{idx} = 18 * X_{idx-1} - X_{idx-2})
#
# The problem identifies two series of solutions for X based on initial (X,Y):
# 1. Starts with (X0,Y0) = (17,7).
#    X = 17 => 17 === 2 (mod 5). N = (17-7)/5 = 2. This is a nugget.
# 2. Starts with (X0,Y0) = (32,14).
#    X = 32 => 32 === 2 (mod 5). N = (32-7)/5 = 5. This is a nugget.
#
# It's mentioned that X_k gives a nugget if k is even (X_0, X_2, X_4, ...).
# We need to find the sum of the first 30 such nuggets.
# This means we'll likely take 15 from each series.

nuggets = []

# Series A
# Initial solution X_a0 = 17, Y_a0 = 7
xa_0 = 17
nuggets << (xa_0 - 7) / 5 # Nugget from X_a0

# To start the recurrence X_k = 18*X_{k-1} - X_{k-2}, we need X_{-1} and X_0.
# Let's calculate X_1 first using the (9,4) transformation:
# X_a1 = 9*X_a0 + 20*Y_a0 (here Y_a0 = 7)
#      = 9*17 + 20*7 = 153 + 140 = 293
# Now, X_{a,-1} = 18*X_a0 - X_a1 = 18*17 - 293 = 306 - 293 = 13.
xa_k_minus_2 = 13 # This is X_{a,-1}
xa_k_minus_1 = 17 # This is X_{a,0}

# We need 14 more nuggets from this series (from X_a2, X_a4, ..., X_a28)
# Loop idx from 1 up to 28.
(1..28).each do |idx|
  xa_k = 18 * xa_k_minus_1 - xa_k_minus_2
  if idx.even? # Corresponds to X_a2, X_a4, ...
    nuggets << (xa_k - 7) / 5
  end
  xa_k_minus_2 = xa_k_minus_1
  xa_k_minus_1 = xa_k
end

# Series B
# Initial solution X_b0 = 32, Y_b0 = 14
xb_0 = 32
nuggets << (xb_0 - 7) / 5 # Nugget from X_b0

# Calculate X_1 for series B:
# X_b1 = 9*X_b0 + 20*Y_b0 (here Y_b0 = 14)
#      = 9*32 + 20*14 = 288 + 280 = 568
# Now, X_{b,-1} = 18*X_b0 - X_b1 = 18*32 - 568 = 576 - 568 = 8.
xb_k_minus_2 = 8  # This is X_{b,-1}
xb_k_minus_1 = 32 # This is X_{b,0}

# We need 14 more nuggets from this series (from X_b2, X_b4, ..., X_b28)
(1..28).each do |idx|
  xb_k = 18 * xb_k_minus_1 - xb_k_minus_2
  if idx.even? # Corresponds to X_b2, X_b4, ...
    nuggets << (xb_k - 7) / 5
  end
  xb_k_minus_2 = xb_k_minus_1
  xb_k_minus_1 = xb_k
end

# We have collected 1 (initial) + 14 (from loop) = 15 nuggets from Series A.
# And 1 (initial) + 14 (from loop) = 15 nuggets from Series B.
# Total 30 nuggets.

nuggets.sort!

# Sum the first 30. If more than 30 were generated due to some miscalculation,
# this ensures we only sum 30. If exactly 30, it sums all of them.
sum_of_first_30_nuggets = nuggets.first(30).sum

puts sum_of_first_30_nuggets
