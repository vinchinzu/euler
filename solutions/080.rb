#  <p>It is well known that if the square root of a natural number is not an intege
# r, then it is irrational. The decimal expansion of such square roots is infinite
#  without any repeating pattern at all.</p>
# <p>The square root of two is $1.41421356237309504880\\cdots$, and the digital sum
#  of the first one hundred decimal digits is $475$.</p>
# <p>For the first one hundred natural numbers, find the total of the digital sums
#  of the first one hundred decimal digits for all the irrational square roots.</p
# >

# Solution for Project Euler Problem 80
require 'bigdecimal'

# Initialize total sum
total_digital_sum = 0
NUM_DECIMAL_DIGITS = 100

# Set precision: 1 (integer) + 100 (decimals) + 19 guard digits
CALCULATION_PRECISION = NUM_DECIMAL_DIGITS + 20 # 120 significant digits
TEN_POW_NUM_DECIMAL_DIGITS = BigDecimal('10').power(NUM_DECIMAL_DIGITS)

(1..100).each do |num|
  # Check if √num is an integer (perfect square)
  root_test = Math.sqrt(num)
  next if root_test == root_test.floor

  # Compute square root with high precision
  sqrt_val = BigDecimal(num).sqrt(CALCULATION_PRECISION)

  # Scale by 10^100 and take floor
  scaled_sqrt = (sqrt_val * TEN_POW_NUM_DECIMAL_DIGITS).floor
  scaled_str = scaled_sqrt.to_i.to_s # Convert to string

  # Since √n < 10, scaled_sqrt has 100 or 101 digits
  # If 101 digits, first digit is integer part; take last 100
  decimal_digits = scaled_str.length > NUM_DECIMAL_DIGITS ? scaled_str[-NUM_DECIMAL_DIGITS..-1] : scaled_str.rjust(NUM_DECIMAL_DIGITS, '0')

  # Sum the digits
  current_sum = decimal_digits.chars.map(&:to_i).sum
  total_digital_sum += current_sum

  # Verification for √2
  puts "√#{num} digit sum: #{current_sum}" if num == 2
end

puts "Total sum: #{total_digital_sum}"