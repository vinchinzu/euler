# Sub-string divisibility
# Problem 43
# The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.

# Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

# d2d3d4=406 is divisible by 2
# d3d4d5=063 is divisible by 3
# d4d5d6=635 is divisible by 5
# d5d6d7=357 is divisible by 7
# d6d7d8=572 is divisible by 11
# d7d8d9=728 is divisible by 13
# d8d9d10=289 is divisible by 17
# Find the sum of all 0 to 9 pandigital numbers with this property.

require 'set'

DIVISORS = [2, 3, 5, 7, 11, 13, 17]

sum = 0
(0..9).to_a.permutation(10) do |perm|
  next if perm[0] == 0 # skip numbers starting with 0
  valid = true
  DIVISORS.each_with_index do |div, i|
    # d2d3d4 is perm[1..3], d3d4d5 is perm[2..4], etc.
    num = perm[i+1]*100 + perm[i+2]*10 + perm[i+3]
    unless num % div == 0
      valid = false
      break
    end
  end
  sum += perm.join.to_i if valid
end

puts sum




