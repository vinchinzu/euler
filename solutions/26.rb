# Solution for Project Euler Problem 26: Reciprocal cycles
# Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

def get_cycle_length(n)
  # Step 2: Remove factors of 2 and 5 from n, as they don't contribute to the recurring cycle length.
  # The cycle length of 1/n is determined by the part of n that is coprime to 10.
  while n % 2 == 0
    n /= 2
  end
  while n % 5 == 0
    n /= 5
  end

  # Step 3: If n is 1 after removing factors of 2 and 5, it means the original number
  # was of the form 2^a * 5^b. Such fractions terminate.
  # A terminating decimal has a recurring cycle of length 0.
  return 0 if n == 1

  # Step 4: If n > 1 and coprime to 10, calculate the cycle length.
  # We are looking for the smallest positive integer k (the order) such that 10^k % n == 1.
  # This is found by simulating the long division process for 1/n.
  
  # Initialize remainder for the first step of the long division after the decimal point.
  # (e.g., for 1/7: 10 divided by 7 is 1 with remainder 3. So, remainder is 3.)
  # We are essentially tracking the sequence 10^1 % n, 10^2 % n, ... until it is 1.
  remainder = 10 % n 
  length = 1

  # Continue the long division process. In each step, we effectively calculate (current_remainder * 10) % n.
  # The cycle completes when the remainder sequence repeats, which for 10^k % n means returning to 1.
  while remainder != 1
    remainder = (remainder * 10) % n
    length += 1
  end

  length
end

# Main loop to find the value of d < 1000 that produces the longest recurring cycle.
max_length = 0 # Stores the maximum cycle length found so far.
result_d = 0   # Stores the value of d corresponding to max_length.

# Iterate through all possible values of d from 2 up to (but not including) 1000.
# d=1 would result in 1/1 = 1.0, which has a cycle length of 0.
(2...1000).each do |d|
  current_length = get_cycle_length(d)
  if current_length > max_length
    max_length = current_length
    result_d = d
  end
end

puts result_d