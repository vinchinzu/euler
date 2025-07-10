# Problem 176
# Find the smallest integer c such that N(c) = 47547.
# N(c) = (d(c^2)-1)/2 if c is odd
# N(c) = (d((c/2)^2)-1)/2 if c is even
# Let K = 47547. We need d(X^2) = 2K+1 = 95095.
# X = c if c is odd, X = c/2 if c is even.
# d(X^2) = product(2*a_i+1) for X = p_1^a_1 * ...
# 95095 = 5 * 7 * 11 * 13 * 19.
# Exponents a_i are {2, 3, 5, 6, 9}.

# Candidate c derived from X_even = 2^9 * 3^6 * 5^5 * 7^3 * 11^2
# c1 = 2 * X_even = 2^10 * 3^6 * 5^5 * 7^3 * 11^2

# Candidate c derived from X_odd = 3^9 * 5^6 * 7^5 * 11^3 * 13^2
# c2 = X_odd

# c1 is smaller than c2.

result = (2**10) * (3**6) * (5**5) * (7**3) * (11**2)
puts result
