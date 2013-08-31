# Goldbach's other conjecture
# Problem 46
# It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

# 9 = 7 + 2×12
# 15 = 7 + 2×22
# 21 = 3 + 2×32
# 25 = 7 + 2×32
# 27 = 19 + 2×22
# 33 = 31 + 2×12

# It turns out that the conjecture was false.

# What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

require 'Prime'

pri = Prime.first(750)
squares = (1..200).map{|x| x**2}

n=9

n.odd?

set = []
pri.each do |x|
      squares.each do |y|
	  set << (x+2*y)
	end
end

set.delete_if{|x| x.even?}.delete_if{|x| x.prime?}.uniq!.sort!


cc = (2..10000).to_a.delete_if{|x| x.even?}.delete_if{|x| x.prime?}


sol = cc - set
puts sol