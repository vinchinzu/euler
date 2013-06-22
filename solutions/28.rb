#ruby 27
#Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
#
#  21 22 23 24 25
#  20  7  8  9 10
#  19  6  1  2 11
#  18  5  4  3 12
#  17 16 15 14 13
#
#It can be verified that the sum of the numbers on the diagonals is 101.
#
# What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

#idea 1  map arr within array and use positioning.... stupid idea

# after mapping a few times in excel noticed that only aodds are added, each round 4 corners are added up to the square
#each time it is increased by the sum. 

#add odd numbers
require 'benchmark'

def spiral(n)

i = 1 
sum = 1
j = 2
 
while i <  n**2 
  1.upto(4) do |x|
   i += j
   sum += i
  end
 j +=2 
end
sum
end

Benchmark.bm do |x|
 x.report("Solution: ") {
 puts spiral(1001)
 }
 end

