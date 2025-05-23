# Champernowne's constant
# Problem 40
# An irrational decimal fraction is created by concatenating the positive integers:

# 0.123456789101112131415161718192021...

# It can be seen that the 12th digit of the fractional part is 1.

# If dn represents the nth digit of the fractional part, find the value of the following expression.

# d1  d10  d100  d1000  d10000  d100000  d1000000

positions = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]
result_digits = []

current_length = 0
number = 1
next_pos_index = 0

while next_pos_index < positions.size
  num_str = number.to_s
  if current_length + num_str.length >= positions[next_pos_index]
    # Find the digit(s) needed in this number
    while next_pos_index < positions.size && current_length + num_str.length >= positions[next_pos_index]
      offset = positions[next_pos_index] - current_length - 1
      result_digits << num_str[offset].to_i
      next_pos_index += 1
    end
  end
  current_length += num_str.length
  number += 1
end

puts result_digits.inject(:*)
