# Function to find the least k such that R(k) is divisible by n
# R(k) is a repunit of length k (e.g., R(3) = 111)
# Assumes gcd(n, 10) == 1
def a_n(n)
  if n % 2 == 0 || n % 5 == 0
    raise ArgumentError, "n must not be divisible by 2 or 5"
  end

  k = 1
  repunit_mod_n = 1 % n

  while repunit_mod_n != 0
    k += 1
    repunit_mod_n = (repunit_mod_n * 10 + 1) % n
    # Safety break for very large k, though problem constraints shouldn't hit this
    # if k > n * 2 # A(n) <= n for n > 1
    #   raise "k is getting too large, possible issue or very large A(n) for n=#{n}"
    # end
  end
  k
end

# Function to check if a number is prime
def is_prime(num)
  return false if num < 2
  return true if num == 2 || num == 3
  return false if num % 2 == 0 || num % 3 == 0

  i = 5
  while i * i <= num
    return false if num % i == 0 || num % (i + 2) == 0
    i += 6
  end
  true
end

# Main logic
found_count = 0
total_sum = 0
n = 1 # Start from 1, will increment to 2, then 3, etc.

puts "Searching for the first 25 composite values n such that gcd(n, 10) = 1 and (n-1) is divisible by A(n)..."

loop do
  n += 1

  # Condition: gcd(n, 10) = 1
  next if n % 2 == 0 || n % 5 == 0

  # Condition: n is composite
  next if is_prime(n)

  # Calculate A(n)
  # puts "Checking n = #{n}" # For debugging
  begin
    k = a_n(n)
  rescue ArgumentError => e
    # This should not happen due to the gcd(n, 10) check above
    puts "Error for n=#{n}: #{e.message}"
    next
  end
  # puts "A(#{n}) = #{k}" # For debugging

  # Condition: (n - 1) is divisible by A(n)
  if (n - 1) % k == 0
    total_sum += n
    found_count += 1
    puts "Found ##{found_count}: n = #{n}, A(n) = #{k}. (n-1)/A(n) = #{(n-1)/k}. Cumulative sum: #{total_sum}"
  end

  break if found_count == 25
end

puts "The sum of the first twenty-five such composite numbers is: #{total_sum}"
