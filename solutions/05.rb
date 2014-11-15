
# Smallest multiple
# Problem 5
# 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

# What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

a = 17*13

m = []
set = (1..1200000).each do |x|
  if (x*a)%18==0 and (x*a) % 16 ==0 and (x*a) % 7 ==0 and (x*a)% 11 ==0 and (x*a)%19==0 and (x*a)%10==0 and (x*a)%14 ==0
    m << x*a
  end
 end
 
 puts m.first
 
 



# div = (1..10).to_a
# found = false
# n=1

# until found == true
  
  # if div.all? {|x| n%x == 0}
   # found = true
  # else
    # next
  # end
  # n+=2
  # break if found == true
  
# end

# puts n