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

def collatz_length(n, memo)
  # If already computed, return the stored length
  return memo[n] if memo[n] != 0
  
  # Base case
  if n == 1
    memo[n] = 1
  # Even case
  elsif n.even?
    memo[n] = 1 + collatz_length(n / 2, memo)
  # Odd case
  else
    memo[n] = 1 + collatz_length(3 * n + 1, memo)
  end
  memo[n]
end

def longest_collatz_under(limit)
  memo = Hash.new(0)  # Use a hash for memoization to handle large n
  max_length = 0
  starting_number = 1

  (1...limit).each do |i|
    length = collatz_length(i, memo)
    if length > max_length
      max_length = length
      starting_number = i
    end
  end

  starting_number
end

limit = 1_000_000
puts longest_collatz_under(limit)