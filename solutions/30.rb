# Digit fifth powers
# Problem 30
# Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

# 1634 = 14 + 64 + 34 + 44
# 8208 = 84 + 24 + 04 + 84
# 9474 = 94 + 44 + 74 + 44
# As 1 = 14 is not a sum it is not included.

# The sum of these numbers is 1634 + 8208 + 9474 = 19316.

# Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.




def test(i)
i == i.to_s.split(//).map{|i| i.to_i**5}.inject(:+) ? true : false
end
 
 d = []
(2..200000).each do |x|
  d << x if test(x)
end
 
puts d.inject(:+)

#1523 ms

