# 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
# What is the sum of the digits of the number 2^1000?
puts (2**1000).to_s.split(//).to_a.map(&:to_i).inject(:+)


# x = 2**1000

# set = x.to_s.split(//).to_a

# set2 = set.map(&:to_i)


# puts set2.inject(:+)

# set = x.to_s.split(//).to_a.map(&:to_i).inject(:+)
