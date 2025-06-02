# Problem 133: Repunit non-factors
# Find the sum of all primes below one-hundred thousand that will never be a factor of R(10^n).
# A prime p is never a factor of R(10^n) if its order modulo p, ord_p(10),
# is not of the form 2^a * 5^b.

require 'prime'

# Calculates (base^exp) % mod
def power(base, exp, mod)
  res = 1
  base %= mod
  while exp > 0
    res = (res * base) % mod if exp.odd?
    base = (base * base) % mod
    exp /= 2
  end
  res
end

# Calculates the multiplicative order of n modulo prime_mod
# Assumes prime_mod is prime and n is not a multiple of prime_mod
def multiplicative_order(n, prime_mod)
  # For p > 5, 10 is not a multiple of p, so gcd(10, p) = 1
  # Thus, an order should always exist.
  
  phi = prime_mod - 1 # By Fermat's Little Theorem, since prime_mod is prime
  
  # Get prime factors of phi
  # prime_division returns pairs like [[factor1, power1], [factor2, power2], ...]
  # We only need the factors themselves.
  factors = phi.prime_division.map(&:first)
  
  order = phi
  factors.each do |factor|
    # Repeatedly divide order by factor as long as the congruence holds
    while (order % factor).zero? && power(n, order / factor, prime_mod) == 1
      order /= factor
    end
  end
  order
end

# Checks if the order contains any prime factors other than 2 or 5.
# Returns true if the order IS of the form 2^a * 5^b, false otherwise.
def order_is_power_of_2_or_5(order)
  # This case should ideally not be hit if multiplicative_order is called correctly
  # for p > 5 and n=10, as an order must exist.
  return false if order == 0 

  temp_order = order
  # Remove all factors of 2
  temp_order /= 2 while (temp_order % 2).zero?
  # Remove all factors of 5
  temp_order /= 5 while (temp_order % 5).zero?
  
  # If temp_order is 1, it means all prime factors were either 2 or 5
  temp_order == 1
end

sum_of_non_factors = 0

# Primes 2, 3, 5 are never factors of R(10^n) for any n >= 1.
# R(10^n) = (10^(10^n) - 1) / 9.
# R(10^n) is always an integer ending in 1, so it's not divisible by 2 or 5.
# Sum of digits of 10^(10^n) is 1. (10^(10^n)-1) is a sequence of 10^n nines.
# (10^(10^n)-1)/9 is a sequence of 10^n ones (a repunit R(10^n)).
# The sum of digits of R(10^n) is 10^n.
# If 10^n is a multiple of 3, R(10^n) is a multiple of 3.
# However, the problem asks for primes p such that p is *never* a factor of R(10^n).
# $10^n \pmod 3 = 1^n \pmod 3 = 1 \pmod 3$.
# $R(k) = (10^k-1)/9$. The sum of digits of $R(k)$ is $k$.
# $R(k) \pmod 3 = k \pmod 3$.
# So $R(10^n) \pmod 3 = 10^n \pmod 3 = 1 \pmod 3$.
# Thus, R(10^n) is never divisible by 3.
sum_of_non_factors += 2 + 3 + 5

# Iterate through primes p such that 5 < p < 100,000
# Prime.each(N) generates primes up to N, inclusive.
# So, we need primes up to 99,999.
Prime.each(100_000 - 1) do |p|
  next if p <= 5 # Already handled and ensures p-1 is not too small for prime_division

  # Calculate ord_p(10)
  order = multiplicative_order(10, p)
  
  # If the order is NOT of the form 2^a * 5^b, then p is a non-factor.
  unless order_is_power_of_2_or_5(order)
    sum_of_non_factors += p
  end
end

puts sum_of_non_factors
