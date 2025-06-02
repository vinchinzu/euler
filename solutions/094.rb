#  <p>It is easily proved that no equilateral triangle exists with integral length
# sides and integral area. However, the <dfn>almost equilateral triangle</dfn> $5$
# -$5$-$6$ has an area of $12$ square units.</p>
# <p>We shall define an <dfn>almost equilateral triangle</dfn> to be a triangle fo
# r which two sides are equal and the third differs by no more than one unit.</p>
# <p>Find the sum of the perimeters of all <dfn>almost equilateral triangles</dfn>
#  with integral side lengths and area and whose perimeters do not exceed one bill
# ion ($1\,000\,000\,000$).</p>

sum_of_perimeters = 0
limit = 1_000_000_000

# X_k sequence for X^2 - 3Y^2 = 1, derived from (2+sqrt(3))^n
# Recurrence for Xk: X_{n+1} = 4*X_n - X_{n-1}
# X0 = 1
# X1 = 2

# Base cases for the recurrence
# xk_minus_1 corresponds to X_{k-1}
# xk corresponds to X_k
xk_minus_1 = 1 # This is X0
xk = 2         # This is X1
current_k = 1  # This is the index of xk (X1)

loop do
  # Calculate X_{current_k+1}
  # This will be X2, then X3, then X4, ...
  xk_plus_1 = 4 * xk - xk_minus_1
  index_of_xk_plus_1 = current_k + 1

  perimeter = 0
  if index_of_xk_plus_1 % 2 == 0
    # Index is Even (e.g., 2, 4, ... for X2, X4, ...)
    # This corresponds to case (a,a,a+1). Perimeter P = 2*X_k + 2.
    # Smallest k here is 2 (X2=7). a = (2*7+1)/3 = 5. P = 16. Valid.
    perimeter = 2 * xk_plus_1 + 2
  else
    # Index is Odd (e.g., 3, 5, ... for X3, X5, ...)
    # This corresponds to case (a,a,a-1). Perimeter P = 2*X_k - 2.
    # Smallest k here is 3 (X3=26). a = (2*26-1)/3 = 17. P = 50. Valid.
    # X1=2 (index 1) would give a=1, P=2 (area 0), which is correctly skipped
    # because this branch is for X3, X5, etc.
    perimeter = 2 * xk_plus_1 - 2
  end

  if perimeter > limit
    break
  end

  sum_of_perimeters += perimeter

  # Update for next iteration
  xk_minus_1 = xk
  xk = xk_plus_1
  current_k = index_of_xk_plus_1
end

puts "Sum of perimeters: #{sum_of_perimeters}"
# Expected Answer: 518408346
