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
check = arr.each_index.select{|i| arr[i] == 4}

# find the sequential values 

solutions = check.each_cons(4){|i,j,k,l| p i if i==( j-1) and i == (k-2) and i == (l-3)}

# find fol

puts solutions[0]