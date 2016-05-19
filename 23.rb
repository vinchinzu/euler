# Non-abundant sums
# Problem 23
# A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. 
# For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
 # which means that 28 is a perfect number.

# # A number n is called deficient if the sum of its proper divisors 
# is less than n and it is called abundant if this sum exceeds n.

# # As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, 
# the smallest number that can be written as the sum of two abundant numbers is 24. 
# By mathematical analysis, it can be shown that all integers greater than 28123 
# can be written as the sum of two abundant numbers. 
# However, this upper limit cannot be reduced any further by analysis
 # even though it is known that the greatest number that cannot be expressed 
 # as the sum of two abundant numbers is less than this limit.

# Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.


#def divisors
require 'mathn'

class Integer
  def divisor_count
    sum = 1
    # prime_division return two dimensional array.
    # for 48, [3,1], [2,4] is the result
    self.prime_division.each do |x| 
      sum *= (x[1] + 1)  
    end
    sum
  end
end





def div2(y)
  (2..y).do |x|
    i=1
     if y % x  =0 
     then i+=1
     end
  end
 i
end

div2(10)




def div2(y)
 i=1
  (2..(y-1)).each do |x|
   
   if y % x == 0 
    then
	  i+=1
	end
   end
    i 
end
div2(12)

def divisors(y)
 i=[]
  (1..(y-1)).each do |x|
   
   if y % x == 0 
    then
	  i << x
	end
   end
    i 
end
divisors(12)


def abu?(n)
 if divisors(n).inject(:+) > n
  then true
  else false
end
end


set_a = []
(2..28123).each do |x|
 if abu?(x)
  then set_a << x
  end
end

#1 minute plus - 