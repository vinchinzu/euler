# Problem 72
#
#    Consider the fraction, $\dfrac n d$, where $n$ and $d$ are positive
#    integers. If $n \lt d$ and $\operatorname{HCF}(n,d)=1$, it is called a
#    reduced proper fraction.
#
#    If we list the set of reduced proper fractions for $d \le 8$ in
#    ascending order of size, we get: $$\frac 1 8, \frac 1 7, \frac 1 6,
#    \frac 1 5, \frac 1 4, \frac 2 7, \frac 1 3, \frac 3 8, \frac 2 5, \frac
#    3 7, \frac 1 2, \frac 4 7, \frac 3 5, \frac 5 8, \frac 2 3, \frac 5 7,
#    \frac 3 4, \frac 4 5, \frac 5 6, \frac 6 7, \frac 7 8$$
#
#    It can be seen that there are $21$ elements in this set.
#
#    How many elements would be contained in the set of reduced proper
#    fractions for $d \le 1\,000\,000$?

# Solution for Project Euler Problem 72

# The number of reduced proper fractions n/d for a fixed denominator d
# (where 1 <= n < d and HCF(n,d) = 1) is given by Euler's totient function φ(d).
# The problem asks for the sum of φ(d) for all d from 2 to 1,000,000.
# Sum_{d=2 to N} φ(d)

LIMIT = 1_000_000

# Array to store phi values. Initialize phi[i] = i.
phi_values = Array.new(LIMIT + 1) { |i| i }

# phi[0] is not well-defined in this context, phi[1] = 1.
# The sieve starts from i = 2.
# For phi[1], if it were part of the sum, it would be 1.
# However, the problem implies d >= 2 for n < d (so n >= 1).
# If d=1, n cannot exist (n < 1, n positive).
# So we sum φ(d) for d from 2 to LIMIT.

(2..LIMIT).each do |i|
  if phi_values[i] == i # i is prime
    # For this prime i, iterate through its multiples j
    # and update their phi values.
    # phi(j) = phi(j) * (1 - 1/i) = phi(j) - phi(j)/i
    j = i
    while j <= LIMIT
      phi_values[j] -= phi_values[j] / i
      j += i
    end
  end
end

# Calculate the sum of phi_values[d] for d from 2 to LIMIT.
# Note: phi_values[0] and phi_values[1] are not included in the sum as per problem.
total_fractions = 0
(2..LIMIT).each do |d|
  total_fractions += phi_values[d]
end

puts "The number of elements in the set of reduced proper fractions for d <= #{LIMIT} is: #{total_fractions}"
