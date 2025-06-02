# Distinct primes factors
# Problem 47
# The first two consecutive numbers to have two distinct prime factors are:

# 14 = 2 × 7
# 15 = 3 × 5

# The first three consecutive numbers to have three distinct prime factors are:

# 644 = 2² × 7 × 23
# 645 = 3 × 5 × 43
# 646 = 2 × 17 × 19.

# Find the first four consecutive integers to have four distinct prime factors. What is the first of these numbers?


require 'prime'


list = []

# set list as array of all factors
(1..200000).each do |x|
list << Prime.prime_division(x)
end
#count the number of factors
arr = list.map{|x| x.count}

#select the array locatiosn where equal to 4
check = arr.each_index.select{|i| arr[i] == 4} # These are 0-based indices

# find the sequential values 
# check contains indices i such that number (i+1) has 4 distinct prime factors.
# We need to find four consecutive numbers, so their indices in 'arr' must also be consecutive.
first_num_in_sequence = nil
check.each_cons(4) do |idx1, idx2, idx3, idx4|
  if idx1 + 1 == idx2 && idx1 + 2 == idx3 && idx1 + 3 == idx4
    # Found four consecutive indices in 'check', which means
    # idx1, idx1+1, idx1+2, idx1+3 are consecutive indices in 'arr'
    # for numbers that each have 4 distinct prime factors.
    # The first number of this sequence is (idx1 + 1).
    first_num_in_sequence = idx1 + 1 
    break # Found the first sequence, so we can stop.
  end
end

puts first_num_in_sequence