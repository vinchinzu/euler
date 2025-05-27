# Problem 110: Diophantine reciprocals II

# The number of distinct solutions to 1/x + 1/y = 1/n is (d(n^2) + 1) / 2.
# We want this to exceed 4,000,000.
# So, (d(n^2) + 1) / 2 > 4_000_000
# d(n^2) + 1 > 8_000_000
# d(n^2) > 7_999_999
# Let TARGET_NUM_DIVISORS_N_SQUARED be 8_000_000, as d(n^2) must be at least this value
# (i.e. the smallest integer strictly greater than 7_999_999).

TARGET_NUM_DIVISORS_N_SQUARED = 8_000_000

# If n = p_1^a_1 * p_2^a_2 * ... * p_k^a_k,
# then n^2 = p_1^(2a_1) * p_2^(2a_2) * ... * p_k^(2a_k).
# The number of divisors of n^2 is d(n^2) = (2a_1+1)(2a_2+1)...(2a_k+1).
# We need (2a_1+1)(2a_2+1)...(2a_k+1) >= TARGET_NUM_DIVISORS_N_SQUARED.

# To minimize n, we assign larger exponents to smaller primes.
# So, a_1 >= a_2 >= ... >= a_k >= 1.

# Primes to use. Max 15 primes should be more than enough.
# The smallest (2a+1) is 3 (for a=1).
# 3^k >= 8_000_000 => k log 3 >= log(8*10^6) => k * 0.477 >= 6.903 => k >= 14.47
# So, we might need up to 15 primes if all exponents are 1.
PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] # First 15 primes

# Global variable to store the minimum n found so far.
@min_n_found = Float::INFINITY

# Recursive search function
# prime_idx: current index in the PRIMES array
# current_prod_of_2a_plus_1: the product (2a_1+1)...(2a_{idx-1}+1) so far
# current_n_val: the value p_1^a_1 * ... * p_{idx-1}^a_{idx-1} so far
# max_exponent_for_this_prime: ensures a_idx <= a_{idx-1} (for current prime)
def find_min_n_recursive(prime_idx, current_prod_of_2a_plus_1, current_n_val, max_exponent_for_this_prime)
  # If current_n_val (even without adding current prime) is already greater than the best n found, prune.
  return if current_n_val >= @min_n_found

  # Base case for successful path: if the product of (2a+1) terms is sufficient
  if current_prod_of_2a_plus_1 >= TARGET_NUM_DIVISORS_N_SQUARED
    if current_n_val < @min_n_found
      @min_n_found = current_n_val
      # puts "New min_n_found: #{@min_n_found} (prod #{current_prod_of_2a_plus_1})" # For debugging
    end
    return
  end

  # Base case for unsuccessful path: if we run out of primes and target not met
  return if prime_idx >= PRIMES.length

  prime = PRIMES[prime_idx]
  
  (1..max_exponent_for_this_prime).each do |a|
    # Calculate new n based on current prime and exponent 'a'
    # Ruby handles large integers automatically.
    # Check against @min_n_found early.
    
    # Calculate prime**a safely
    begin
      term_val = prime**a
    rescue RangeError # prime**a too large
      term_val = Float::INFINITY
    end

    if term_val == Float::INFINITY || current_n_val > Float::INFINITY / term_val
        new_n_val = Float::INFINITY
    else
        new_n_val = current_n_val * term_val
    end

    break if new_n_val >= @min_n_found

    new_prod_of_2a_plus_1 = current_prod_of_2a_plus_1 * (2 * a + 1)
    
    # Recurse for the next prime, passing 'a' as the max_exponent for the next level (a_{idx+1} <= a_idx)
    find_min_n_recursive(prime_idx + 1, new_prod_of_2a_plus_1, new_n_val, a)
  end
end

# Initial call parameters
# prime_idx = 0 (for prime 2)
# current_prod_of_2a_plus_1 = 1
# current_n_val = 1
# max_exponent_for_this_prime (for prime 2, a_1):
# A reasonable upper limit for the first exponent (a_1 for prime 2) could be around 20-30.
# Let's use 25 as an initial estimate. The search prunes effectively.
initial_max_exponent = 25 
# If log2(@min_n_found_estimate) is lower, can use that.
# E.g. if we expect n around 10^12 to 10^15, log2(10^12) = 12 log2(10) ~ 12 * 3.32 ~ 39.
# log2(10^15) ~ 15 * 3.32 ~ 49.
# So 25 might be a bit low if solution has very high power of 2.
# However, solutions usually balance prime exponents.
# For d(n^2) ~ 8*10^6, if n = 2^a, 2a+1 ~ 8*10^6 => a ~ 4*10^6. 2^(4*10^6) is too big.
# Product (2a_i+1). If 10 primes, (2a+1)^10 ~ 8*10^6 => 2a+1 ~ (8*10^6)^0.1 ~ 4. So a=1 or 2.
# If 5 primes, (2a+1)^5 ~ 8*10^6 => 2a+1 ~ (8*10^6)^0.2 ~ 24. So a ~ 11.
# Max exponent for 2^a_1. If a_1=15, 2a_1+1=31. Need product 8*10^6/31 ~ 2.5*10^5 from rest.
# (2*15+1) * (2*10+1)^4 = 31 * 21^4 = 31 * 194481 = 6 * 10^6. This is close, with 5 primes.
# n = 2^15 * 3^10 * 5^10 * 7^10 * 11^10. This 'n' would be huge.
# Exponents must be decreasing: 2^15 * 3^10 * 5^8 * 7^7 * 11^6.
# So, an initial_max_exponent of around 15-20 seems more plausible than 25 for the first prime,
# if we are trying to keep n small. Let's try 20.
initial_max_exponent = 20

# puts "Starting search for n with initial_max_exponent=#{initial_max_exponent}..."
find_min_n_recursive(0, 1, 1, initial_max_exponent)

# The problem asks for the integer value of n.
puts @min_n_found.to_i
