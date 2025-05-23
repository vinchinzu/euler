# Summation of primes
# Problem 10
# The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

# Find the sum of all the primes below two million.

require 'prime'

i = 148933
max = Prime.first(i).last
puts  Prime.first(i).inject(:+)

# while set.last <= 2000000
# i +=1
# set = Prime.first(i)
# end





