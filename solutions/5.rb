# Smallest multiple
# Problem 5
# 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

# What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

div = (1..10).to_a
found = false
n=1

until found == true
  
  if div.all? {|x| n%x == 0}
   found = true
  else
    next
  end
  n+=2
  break if found == true
  
end

puts n