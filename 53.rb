

# Combinatoric selections
# Problem 53
# There are exactly ten ways of selecting three from five, 12345:

# 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

# In combinatorics, we use the notation, 5C3 = 10.

# In general,

# nCr =	
# n!
# r!(n−r)!
# ,where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
# It is not until n = 23, that a value exceeds one-million: 23C10 = 1144066.

# How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than one-million?


# only go up to 1_000_000 then use triangle

limit = 1_000_000


how_many = 0

(2..30).each do |j|
set=(1..j).to_a
  i = 1
    until  set.combination(i).to_a.count > 1_000_000
	  i +=1
	  break if i > a.count
	end
  how_many += j - i * 2
end

puts how_many	
		


# first attempts
# max = 0
# limit = 10000

# (1..100).each do |j|
   # a=(1..j).to_a
      # (1..j-1).each do |i|
       # if  a.combination(i).to_a.count > limit 
	     # max += 1
	   # end
     # end
# end

# limit = 1_000_000

# how_many = 0
# a=(1..27).to_a
      # (1..a.count-1).each do |i|
	   # set = a.combination(i).to_a.count
         # puts set
		 # if  set > limit 
	       # how_many += 1
	     # end
       # end
# how_many
