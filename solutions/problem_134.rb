# Problem 134: Prime pair connection
# For every pair of consecutive primes (p1, p2) with 5 <= p1 <= 1,000,000:
# 1. Determine k, the number of digits in p1. Let M = 10^k.
# 2. Find the smallest positive integer S such that S ends with digits of p1 (S === p1 mod M)
#    and S is divisible by p2 (S === 0 mod p2).
# 3. Sum these values of S.

require 'prime'

# Extended Euclidean Algorithm
# Returns [gcd, x, y] such that a*x + b*y = gcd(a,b)
def extended_gcd(a, b)
  return [b, 0, 1] if a == 0
  gcd, x1, y1 = extended_gcd(b % a, a)
  x = y1 - (b / a) * x1
  y = x1
  [gcd, x, y]
end

# Modular inverse
# Returns x such that (a * x) % m == 1
def mod_inverse(a, m)
  gcd, x, _y = extended_gcd(a, m)
  # p2 can be larger than M, but gcd(p2, M) must be 1.
  # M = 10^k = 2^k * 5^k. So p2 cannot be 2 or 5.
  # Since p1 >= 5, p2 will be > 5, so p2 is not 2 or 5.
  # Thus, gcd(p2, M) will be 1.
  raise "Modular inverse does not exist if gcd(a, m) != 1, gcd was #{gcd}" if gcd != 1
  (x % m + m) % m
end

limit_p1 = 1_000_000
# We need p2 for p1 up to limit_p1.
# If p1 is the largest prime <= 1,000,000 (which is 999983),
# p2 is the next prime (1000003).
# So, generating primes up to limit_p1 + a small margin (e.g., 200, or just find next prime after limit_p1)
# Prime.each(N) goes up to N inclusive.
# Max p1 = 999983. Next prime is 1000003. So Prime.each(1000003) is sufficient.
# A slightly larger upper bound for safety is fine.
primes = Prime.each(limit_p1 + 200).to_a # Using a buffer like in the draft

total_s_sum = 0

(0...primes.length - 1).each do |i|
  p1 = primes[i]
  
  next if p1 < 5 # Start from p1 = 5
  break if p1 > limit_p1 # Process p1 up to the limit
  
  p2 = primes[i+1]
  
  # Determine k, the number of digits in p1
  # k = p1.to_s.length # String conversion is okay for numbers up to 10^6
  # More arithmetically:
  k = 0
  temp_p1 = p1
  if temp_p1 == 0 # Should not happen for p1 >= 5
    k = 1
  else
    k = Math.log10(temp_p1).floor + 1
  end

  m_mod = 10**k
  
  # We need to solve S = x * p2
  # And S = p1 (mod m_mod)
  # So, x * p2 = p1 (mod m_mod)
  # x = p1 * mod_inverse(p2, m_mod) (mod m_mod)
  # Let this x be m_val in the problem description's context
  
  inv_p2_mod_m = mod_inverse(p2, m_mod)
  m_val = (p1 * inv_p2_mod_m) % m_mod
  
  # S is m_val * p2.
  # The problem asks for the smallest *positive* integer S.
  # m_val from % m_mod will be in [0, m_mod - 1].
  # If p1 * inv_p2_mod_m is a multiple of m_mod, then m_val = 0.
  # This implies p1 % m_mod = 0 (since inv_p2_mod_m is coprime to m_mod).
  # But p1 < m_mod (e.g. if p1=99, k=2, m_mod=100. p1=7, k=1, m_mod=10).
  # So p1 % m_mod = p1, which is not 0 for p1 >= 5.
  # Therefore, m_val will be in [1, m_mod - 1].
  # The smallest positive S is obtained with this m_val.
  
  s_value = m_val * p2
  
  total_s_sum += s_value
end

puts total_s_sum
