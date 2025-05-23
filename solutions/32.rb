# Pandigital products
# Problem 32
# We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

# The product 7254 is unusual, as the identity, 39  186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

# Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

# HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

require 'set'

s = Set.new
(1..9).to_a.permutation.each do |perm|
  # Split 1: a (1 digit), b (4 digits), c (4 digits)
  a = perm[0]
  b = perm[1..4].map(&:to_s).join.to_i
  c = perm[5..8].map(&:to_s).join.to_i
  s.add(c) if a * b == c

  # Split 2: a (2 digits), b (3 digits), c (4 digits)
  a = perm[0..1].map(&:to_s).join.to_i
  b = perm[2..4].map(&:to_s).join.to_i
  c = perm[5..8].map(&:to_s).join.to_i
  s.add(c) if a * b == c
end

puts s.inject(0, :+)