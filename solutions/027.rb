# Quadratic primes
# Problem 27
# Published on Friday, 27th September 2002, 06:00 pm; Solved by 36147
# Euler discovered the remarkable quadratic formula:

# n² + n + 41

# It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39. However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41² + 41 + 41 is clearly divisible by 41.

# The incredible formula  n²  79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0 to 79. The product of the coefficients, 79 and 1601, is 126479.

# Considering quadratics of the form:

# n² + an + b, where |a|  1000 and |b|  1000

# where |n| is the modulus/absolute value of n
# e.g. |11| = 11 and |4| = 4
# Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.

require 'prime' # Changed from 'mathn' to 'prime' for compatibility with modern Ruby versions

# Function to check primality with caching.
# Uses memoization (the prime_cache) to store results of primality tests
# and avoid re-computing them, significantly speeding up repeated checks for the same number.
def is_prime_cached(num, prime_cache)
  return false if num < 2 # Numbers less than 2 are not prime
  if prime_cache.key?(num)
    return prime_cache[num]
  end
  # Calculate primality and cache it
  is_prime = num.prime?
  prime_cache[num] = is_prime
  is_prime
end

# For the quadratic n² + an + b:
# When n = 0, the expression becomes 'b'. For 'b' to be prime itself (as it's the first term in the sequence of primes),
# 'b' must be a prime number.
# Also, 'b' must be positive since n=0 must yield a prime. Problem statement implies |b| <= 1000.
# We pre-compute primes for 'b' here to reduce the search space in the main loop.
primes_b = []
(1..1000).each do |y| # As per problem statement |b| <= 1000.
  primes_b << y if y.prime?
end

# Function to count consecutive primes for a given a, b, using the cache
def count_consecutive_primes(a, b, prime_cache)
  n = 0
  loop do
    val = n**2 + a * n + b
    break unless is_prime_cached(val, prime_cache)
    n += 1
  end
  n
end

m = 0 # max primes found
p = 0 # product of coefficients a*b for max primes

# Global cache to store primality test results. This cache is populated by is_prime_cached
# and used across all calls to count_consecutive_primes, avoiding redundant calculations.
prime_cache = {}

# Iterate over possible values of a and pre-computed primes for b.
# 'a' ranges from -999 to 999 as per the problem statement |a| < 1000.
# 'b' iterates through primes_b (primes from 1 to 1000).
(-999..999).each do |x|
  primes_b.each do |y|
    # For each (a,b) pair, count how many consecutive primes are generated.
    current_primes_count = count_consecutive_primes(x, y, prime_cache)
    if current_primes_count > m
      m = current_primes_count
      p = x * y
    end
  end
end

puts p