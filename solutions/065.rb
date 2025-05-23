# Project Euler Problem 65: Convergents of e
#
# The square root of 2 can be written as an infinite continued fraction.
# sqrt(2) = [1;(2)], (2) indicates that 2 repeats ad infinitum.
# In a similar way, sqrt(23) = [4;(1,3,1,8)].
#
# It turns out that the sequence of partial values of continued fractions for
# square roots provide the best rational approximations.
# Let us consider the convergents for sqrt(2).
#
# h_0 = 1
# h_1 = 1 + 1/2 = 3/2
# h_2 = 1 + 1/(2 + 1/2) = 7/5
# h_3 = 1 + 1/(2 + 1/(2 + 1/2)) = 17/12
# h_4 = 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29
#
# Hence the sequence of the first ten convergents for sqrt(2) are:
# 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, ...
#
# What is most surprising is that the important mathematical constant,
# e = [2; 1,2,1, 1,4,1, 1,6,1, ... , 1,2k,1, ...].
#
# The first ten terms in the sequence of convergents for e are:
# 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...
#
# The sum of digits in the numerator of the 10th convergent is 1+4+5+7=17.
#
# Find the sum of digits in the numerator of the 100th convergent of the
# continued fraction for e.

# Solution:
# The continued fraction for e is [a_0; a_1, a_2, ...].
# The terms a_k are given by:
# a_0 = 2
# a_k = 1, if (k+1) mod 3 != 0  (for k >= 1)
# a_k = 2*(k+1)/3, if (k+1) mod 3 == 0 (for k >= 1)
# Note: The problem statement uses 'k' in "1,2k,1" which can be confusing.
# Let's adjust index to be 1-based for the pattern part for clarity:
# The sequence is a_0, a_1, a_2, a_3, a_4, a_5, a_6, ...
# Values are:      2,   1,   2,   1,   1,   4,   1,   1,   6,   1, ...
# For a_i where i >= 1:
# if (i) % 3 == 1, then a_i = 1
# if (i) % 3 == 2, then a_i = 2 * (i+1)/3  (e.g., for i=2, a_2 = 2*(2+1)/3 = 2; for i=5, a_5 = 2*(5+1)/3 = 4)
# if (i) % 3 == 0, then a_i = 1

# We need to find the numerator of the 100th convergent.
# The convergents are h_k = p_k / q_k. We are looking for p_99.
# The recurrence relation for numerators is:
# p_k = a_k * p_{k-1} + p_{k-2}
# With initial conditions (standard for this recurrence):
# p_{-2} = 0
# p_{-1} = 1

# Number of terms for the 100th convergent (indexed from 0 to 99)
num_terms = 100

# Array to store the terms a_k of the continued fraction
a = Array.new(num_terms)

# Set a_0
a[0] = 2

# Generate a_1 to a_99
(1...num_terms).each do |k|
  # The pattern for a_k (k >= 1) depends on k itself, not (k+1) as in problem's "2k"
  # The sequence is 2; 1,2,1, 1,4,1, 1,6,1, ...
  # a_0 = 2
  # a_1 = 1
  # a_2 = 2
  # a_3 = 1
  # a_4 = 1
  # a_5 = 4
  # a_6 = 1
  #
  # k: 1  2  3  4  5  6  7  8  9
  # val:1  2  1  1  4  1  1  6  1
  # (k+1):2  3  4  5  6  7  8  9 10
  # If (k+1) % 3 == 0, then a_k = 2*(k+1)/3
  # Otherwise, a_k = 1.
  if (k + 1) % 3 == 0
    a[k] = 2 * (k + 1) / 3
  else
    a[k] = 1
  end
end

# Initialize numerators for the recurrence
# p_km2 corresponds to p_{k-2}
# p_km1 corresponds to p_{k-1}
p_km2 = 0 # p_{-2}
p_km1 = 1 # p_{-1}
current_p = 0

# Calculate p_0 through p_99
# For k = 0: p_0 = a_0 * p_{-1} + p_{-2} = a[0]*1 + 0
# For k = 1: p_1 = a_1 * p_0 + p_{-1}
# ...
# For k = 99: p_99 = a_99 * p_98 + p_97
(0...num_terms).each do |k|
  current_p = a[k] * p_km1 + p_km2
  # Update values for the next iteration
  p_km2 = p_km1
  p_km1 = current_p
end

# current_p now holds p_99, the numerator of the 100th convergent.
numerator_h99 = current_p

# Calculate the sum of the digits of the numerator.
sum_of_digits = numerator_h99.to_s.chars.map(&:to_i).sum

puts "The sum of digits in the numerator of the 100th convergent of e is: #{sum_of_digits}"
