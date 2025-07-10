# Problem 179: Consecutive Positive Divisors

LIMIT = 10**7

# Initialize num_divisors array
# num_divisors[k] will store d(k), the number of divisors of k.
# Array is 0-indexed, so size is LIMIT + 1 to access num_divisors[LIMIT].
num_divisors = Array.new(LIMIT + 1, 0)

# Sieve method to calculate number of divisors for all k up to LIMIT
# For each number i from 1 to LIMIT, i is a divisor of i, 2i, 3i, ...
# So we increment the divisor count for all multiples of i.
1.upto(LIMIT) do |i|
  j = i
  while j <= LIMIT
    num_divisors[j] += 1
    j += i
  end
end

# Count integers n such that d(n) = d(n+1)
# The problem states 1 < n < 10^7.
# This means n ranges from 2 up to 10^7 - 1 (i.e., LIMIT - 1).
count = 0
2.upto(LIMIT - 1) do |n|
  if num_divisors[n] == num_divisors[n + 1]
    count += 1
  end
end

puts count
