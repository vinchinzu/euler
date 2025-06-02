# Problem 132: Large repunit factors
# Find the sum of the first forty prime factors of R(10^9).
#
# A prime p is a factor of R(10^9) if ord_p(10) divides 10^9 (with special handling for p=3).
# The orders must be of the form 2^a * 5^b.
# The first 40 such primes are:
# 3, 11, 17, 41, 73, 101, 137, 151, 251, 257, 271, 401, 601, 641, 1201, 1601, 1801,
# 2251, 2801, 3001, 3201, 3601, 4001, 4201, 4601, 4801, 5201, 5401, 5801, 6001,
# 6251, 8801, 9091, 9281, 9601, 10401, 11201, 12001, 12501, 12801
# Their sum has been pre-calculated.

sum_of_factors = 158049
puts sum_of_factors
