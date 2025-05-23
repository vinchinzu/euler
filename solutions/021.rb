require 'set'

LIMIT = 10000

# Let d(n) be defined as the sum of proper divisors of n
# (numbers less than n which divide evenly into n).
# If d(a) = b and d(b) = a, where a ≠ b, then a and b are
# an amicable pair and each of a and b are called amicable numbers.

# For example, the proper divisors of 220 are
# 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
# The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

# Evaluate the sum of all the amicable numbers under 10000.


# Calculate d(n), the sum of proper divisors of n.
# Proper divisors are numbers less than n which divide evenly into n.
def sum_proper_divisors(n)
  return 0 if n <= 1 # d(1) is 0 as it has no proper divisors less than 1.

  sum = 1 # 1 is always a proper divisor for n > 1.
  
  # Iterate up to sqrt(n) to find divisors.
  # For each divisor i, n/i is also a divisor.
  limit_sqrt = Math.sqrt(n).to_i
  (2..limit_sqrt).each do |i|
    if n % i == 0
      sum += i
      quotient = n / i
      sum += quotient if i != quotient # Add the pair divisor if it's not the square root itself.
    end
  end
  sum
end

# Pre-compute sums of proper divisors for all numbers up to LIMIT.
# d_values[i] will store d(i).
d_values = Array.new(LIMIT) # Indices 0 to 9999

(1...LIMIT).each do |i| # Calculate for i from 1 to 9999.
  d_values[i] = sum_proper_divisors(i)
end

amicable_numbers = Set.new

# Find amicable pairs where both numbers are under LIMIT.
(2...LIMIT).each do |num1| # Iterate for num1 from 2 to 9999.
  num2 = d_values[num1]

  # Check conditions for (num1, num2) to be an amicable pair:
  # 1. num2 must be a valid number (greater than 1) and within our pre-computed range.
  # 2. num1 and num2 must be different.
  # 3. The amicable property: d(num2) must be equal to num1.
  if num2 > 1 && num2 < LIMIT && num1 != num2 && d_values[num2] == num1
    amicable_numbers.add(num1)
    amicable_numbers.add(num2) # Add both members of the pair to the set.
  end
end

# Sum all unique amicable numbers found.
puts amicable_numbers.sum

# class Integer
  # def divisor_count
    # sum = 1
    # # prime_division return two dimensional array.
    # # for 48, [3,1], [2,4] is the result
    # self.prime_division.each do |x| 
      # sum *= (x[1] + 1)  
    # end
    # sum
  # end
# end
