# Function to check if a number is prime
def is_prime(num)
  return false if num < 2
  return true if num == 2 || num == 3
  return false if num % 2 == 0 || num % 3 == 0

  i = 5
  # Iterate up to sqrt(num)
  while i * i <= num
    return false if num % i == 0 || num % (i + 2) == 0
    i += 6
  end
  true
end

# Main logic
# Problem statement: Find how many primes p < 1,000,000 have the property that
# there exists a positive integer n such that n^3 + n^2*p = k^3 for some integer k.
# This simplifies to p = (a^3 - b^3) / b^2 where n = b^3 / (a^3 - b^3) * a
# and p = (x+1)^3/x^2 - (x+1)^2 = (x+1)^2 * ( (x+1)/x^2 - 1) ... no this is not right.
#
# The problem derivation states that p must be of the form 3*m^2 + 3*m + 1.
# Let n^3 + n^2*p = y^3
# n^2(n+p) = y^3
# For n^2(n+p) to be a cube, n+p must be a cube times some factor that makes n^2 a cube.
# Let n = a^3 * k and n+p = b^3 * k for some k. This is too complex.
#
# From the problem overview:
# n^3 + n^2*p = k^3
# n^2(n+p) = k^3
# Let n = x^3 and n+p = y^3 for some integers x, y. (This is not general enough)
#
# A common approach is to set n = a^2 * b and n+p = c^2 * d such that n^2(n+p) is a cube.
# Let n = x^2 * z and n+p = y^3 * z', which is not helping.
#
# If n^2(n+p) = k^3.
# Let n = a^3. Then a^6(a^3+p) = k^3. This implies a^3+p must be a cube.
# Let a^3+p = b^3. Then p = b^3 - a^3 = (b-a)(b^2+ab+a^2).
# Since p is prime, one factor must be 1.
# b-a = 1 => b = a+1.
# Then p = 1 * ((a+1)^2 + a(a+1) + a^2)
# p = a^2 + 2a + 1 + a^2 + a + a^2
# p = 3a^2 + 3a + 1.
# Here 'a' is our 'm'.
# We need to find primes p < 1,000,000 of this form.

prime_count = 0
m = 1

# puts "Searching for primes p < 1,000,000 of the form 3*m^2 + 3*m + 1..."

loop do
  p = 3 * m * m + 3 * m + 1

  if p >= 1_000_000
    break
  end

  if is_prime(p)
    prime_count += 1
    # puts "m = #{m}, p = #{p} (prime)" # For debugging and verification
  # else
    # puts "m = #{m}, p = #{p} (not prime)" # For debugging
  end

  m += 1
end

puts prime_count
