# Non-abundant sums
# Problem 23
# Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

# Calculates the sum of proper divisors of n efficiently.
# Proper divisors are numbers less than n which divide evenly into n.
def sum_of_proper_divisors(n)
  return 0 if n <= 1 # 1 has no proper divisors sum, other numbers <=1 not relevant here

  sum = 1 # 1 is a proper divisor for all n > 1
  sqrt_n = Math.sqrt(n).to_i

  (2..sqrt_n).each do |i|
    if n % i == 0
      sum += i
      quotient = n / i
      sum += quotient if i != quotient # Add the quotient only if it's not the square root
    end
  end
  sum
end

# Checks if a number is abundant.
# An abundant number is a number for which the sum of its proper divisors is greater than the number itself.
def abundant?(n)
  return false if n < 12 # Smallest abundant number is 12
  sum_of_proper_divisors(n) > n
end

LIMIT = 28123 # All integers greater than 28123 can be written as the sum of two abundant numbers.

# Generate a list of abundant numbers up to the limit.
# We start from 12 as it's the smallest abundant number.
abundants = (12..LIMIT).select { |n| abundant?(n) }

# Create a boolean array to mark numbers that can be written as the sum of two abundant numbers.
# Initialize all to false. The array size is LIMIT + 1 to use direct indexing.
can_be_written_as_sum = Array.new(LIMIT + 1, false)

# Populate the can_be_written_as_sum array.
# Iterate through pairs of abundant numbers.
abundants.each_with_index do |a1, index|
  # Start the inner loop from the current index to avoid duplicate pairs (a1+a2 vs a2+a1)
  # and to ensure a2 >= a1.
  abundants[index..-1].each do |a2|
    sum = a1 + a2
    if sum <= LIMIT
      can_be_written_as_sum[sum] = true
    else
      # If sum exceeds LIMIT, further elements in the inner loop (which are larger)
      # will also exceed LIMIT, so we can break early.
      break
    end
  end
end

# Sum all positive integers up to LIMIT that cannot be written as the sum of two abundant numbers.
result = 0
(1..LIMIT).each do |n|
  result += n unless can_be_written_as_sum[n]
end

puts result 