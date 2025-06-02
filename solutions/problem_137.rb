# Problem 137: Fibonacci golden nuggets
#
# The problem asks for the 15th "golden nugget".
# A golden nugget is a positive integer N such that A_F(x) = N for some rational x.
# A_F(x) is the generating function for Fibonacci numbers, A_F(x) = x / (1 - x - x^2).
# Setting A_F(x) = N gives:
# N = x / (1 - x - x^2)
# N(1 - x - x^2) = x
# N - Nx - Nx^2 = x
# Nx^2 + (N+1)x - N = 0
#
# For x to be rational, the discriminant of this quadratic in x must be a perfect square (k^2).
# Discriminant D = (N+1)^2 - 4(N)(-N) = N^2 + 2N + 1 + 4N^2 = 5N^2 + 2N + 1.
# So, 5N^2 + 2N + 1 = k^2 for some integer k >= 0.
# Multiply by 5: 25N^2 + 10N + 5 = 5k^2
# (5N+1)^2 - 1 + 5 = 5k^2
# (5N+1)^2 + 4 = 5k^2
# (5N+1)^2 - 5k^2 = -4.
#
# This is a Pell-like equation of the form X^2 - 5Y^2 = -4.
# The fundamental solution to X^2 - 5Y^2 = 1 is (9,4).
# The solutions (X_j, Y_j) to X^2 - 5Y^2 = -4 are given by X_j = L_{2j+1} (Lucas numbers).
# (Actually, (L_n, F_n) are solutions to x^2 - 5y^2 = 4(-1)^n. We need n odd for -4 on RHS.)
# So, X = 5N+1 = L_{idx} where idx must be odd. Let idx = 2j+1.
#
# We need L_{2j+1} such that 5N+1 = L_{2j+1} gives an integer N.
# This means L_{2j+1} must be congruent to 1 (mod 5).
# The Lucas numbers sequence modulo 5 is:
# L_0 = 2 (mod 5)
# L_1 = 1 (mod 5)
# L_2 = 3 (mod 5)
# L_3 = 4 (mod 5)
# L_4 = 2 (mod 5) (since L_4 = L_3+L_2 = 4+3 = 7 = 2 mod 5)
# L_5 = 1 (mod 5) (since L_5 = L_4+L_3 = 2+4 = 6 = 1 mod 5)
# The sequence is periodic with period 4: (2, 1, 3, 4).
# L_idx === 1 (mod 5) when idx === 1 (mod 4).
# So, 2j+1 must be of the form 4m+1 for some integer m >= 0.
#
# For m=0: 2j+1 = 1. L_1 = 1. 5N+1 = 1 => 5N = 0 => N=0. Not a positive nugget.
# For m=1: 2j+1 = 5. L_5 = 11. 5N+1 = 11 => 5N = 10 => N=2. (1st positive nugget)
# For m=2: 2j+1 = 9. L_9 = 76. 5N+1 = 76 => 5N = 75 => N=15. (2nd positive nugget)
# ...
# The problem asks for the 15th golden nugget. This corresponds to m=15.
# (If m=0 is the "0th" nugget, then m=1 is the 1st, so m=k for k-th nugget).
# So for the 15th nugget, we use m=15.
# The index for the Lucas number is 4*15 + 1 = 60 + 1 = 61.
# The nugget N_15 is (L_61 - 1) / 5.

l_target_index = 61
l_val = 0

if l_target_index == 0
  l_val = 2
elsif l_target_index == 1
  l_val = 1
else
  l_prev = 2 # L_0
  l_curr = 1 # L_1
  # Iterate from 2 up to l_target_index
  (2..l_target_index).each do |_i|
    l_next = l_prev + l_curr
    l_prev = l_curr
    l_curr = l_next
  end
  l_val = l_curr
end

# Now calculate the nugget value
nugget_value = (l_val - 1) / 5
puts nugget_value
