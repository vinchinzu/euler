#  <p>It is well known that if the square root of a natural number is not an intege
# r, then it is irrational. The decimal expansion of such square roots is infinite
#  without any repeating pattern at all.</p>
# <p>The square root of two is $1.41421356237309504880\cdots$, and the digital sum
#  of the first one hundred decimal digits is $475$.</p>
# <p>For the first one hundred natural numbers, find the total of the digital sums
#  of the first one hundred decimal digits for all the irrational square roots.</p
# >

# Solution for Project Euler Problem 80

require 'bigdecimal'
# require 'bigdecimal/util' # Not strictly necessary for to_s("F")

total_digital_sum_of_decimals = 0
NUM_DECIMAL_DIGITS = 100

# Precision for BigDecimal.sqrt:
# The result of sqrt(n) will have some integer digits and some decimal digits.
# For n <= 100, the integer part of sqrt(n) is at most 1 digit (e.g., sqrt(99) is 9.xxxx).
# (sqrt(100)=10, but 100 is a perfect square and will be skipped).
# We need the first 100 decimal digits.
# So, total significant digits needed for the value itself = 1 (for integer part) + 100 (decimal digits) = 101.
# It's common to add a few guard digits for precision during the sqrt calculation.
CALCULATION_PRECISION = 101 + 5 # = 106. This many significant digits will be computed by sqrt.

(1..100).each do |num|
  # Check if num is a perfect square
  root_test = Math.sqrt(num)
  if root_test == root_test.floor # Or use root_test.to_i
    # num is a perfect square, its square root is rational. Skip it.
    next
  end

  # num is not a perfect square, so its square root is irrational.
  # Calculate the square root using BigDecimal for high precision.
  sqrt_val = BigDecimal(num).sqrt(CALCULATION_PRECISION)

  # Convert the BigDecimal to a string in plain floating-point format.
  # The to_s("F") method ensures enough digits are represented.
  # The precision of sqrt_val (106 sig-figs) should be reflected in this string.
  sqrt_str = sqrt_val.to_s("F")
  
  # Extract the decimal part of the string.
  # Example: if sqrt_str is "9.12345...", parts will be ["9", "12345..."].
  # We are interested in parts[1].
  parts = sqrt_str.split('.')
  
  if parts.length < 2
    # This case should not be reached for irrational square roots of n >= 1,
    # as they will always have a non-empty decimal part.
    # If it did, it means no decimal digits to sum for this number.
    decimal_part_str = "" 
  else
    decimal_part_str = parts[1]
  end

  # Take the first NUM_DECIMAL_DIGITS from the decimal part string.
  # The CALCULATION_PRECISION ensures decimal_part_str should be long enough.
  # (e.g., for sqrt(2) ~ "1.414...", decimal_part_str is "414...". We need 100 chars from this.)
  first_100_decimals = decimal_part_str[0...NUM_DECIMAL_DIGITS]
  
  # Sum these digits
  current_sum = 0
  first_100_decimals.each_char do |char_digit|
    current_sum += char_digit.to_i
  end
  # More idiomatic sum: current_sum = first_100_decimals.chars.map(&:to_i).sum

  total_digital_sum_of_decimals += current_sum
end

puts "The total of the digital sums of the first one hundred decimal digits for all irrational square roots is: #{total_digital_sum_of_decimals}"
