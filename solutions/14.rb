# Longest Collatz sequence
# Problem 14
# The following iterative sequence is defined for the set of positive integers:

# n → n/2 (n is even)
# n → 3n + 1 (n is odd)

# Using the rule above and starting with 13, we generate the following sequence:

# 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
# It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. 
#Although it has not been proved yet (Collatz Problem), 
#it is thought that all starting numbers finish at 1.

# Which starting number, under one million, produces the longest chain?

# NOTE: Once the chain starts the terms are allowed to go above one million.

#gensequence


def col(test)
  if test%2==0 
  out = test/2
  else 
 out = test*3 + 1
 end
out
end

max = 1
top = 1
start  = 750000

while (start < 1_000_000)
 set = []
 set << start
 nex = col (start)
  while (nex != 1) 
   nex = col(nex)
   set << nex
  end
 
if set.count > max then 
  max = set.count
  top = start
end

start +=1


#puts start,max, set.count
end

puts top


