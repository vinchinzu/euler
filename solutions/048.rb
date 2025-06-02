# Self powers
# Problem 48
# The series, 11 + 22 + 33 + ... + 1010 = 10405071317.

# Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

# To handle large numbers and keep only the last 10 digits,
# all additions and exponentiations should be done modulo 10^10.
# However, Ruby's bignum arithmetic allows direct computation,
# though it might be slower than necessary. The task is to ensure
# correct single integer output.

mod_value = 10**10
sum_last_digits = 0

(1..1000).each do |x|
  # Calculate x**x modulo mod_value
  # pow(base, exp, mod) is efficient for this.
  # Ruby's ** operator can be slow for huge x**x before taking modulo.
  # For x**x, we need modular exponentiation.
  term_last_digits = x.pow(x, mod_value)
  sum_last_digits = (sum_last_digits + term_last_digits) % mod_value
end

# The result sum_last_digits is already an integer.
# If we were converting from a full string sum:
# full_sum_str = (1..1000).map {|x| x**x}.inject(:+).to_s
# last_ten_digits_str = full_sum_str[-10..-1]
# puts last_ten_digits_str.to_i

puts sum_last_digits