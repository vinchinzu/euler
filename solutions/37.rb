# Truncatable primes
# Problem 37
# The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

# Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

# NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

require 'prime'

right_truncatable = []
queue = [2, 3, 5, 7]

while !queue.empty?
  new_queue = []
  queue.each do |num|
    [1, 3, 5, 7, 9].each do |digit|
      new_num = num * 10 + digit
      if Prime.prime?(new_num)
        new_queue << new_num
        right_truncatable << new_num  # All new_num have ≥ 2 digits
      end
    end
  end
  queue = new_queue
end