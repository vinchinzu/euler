# Problem 104: Pandigital Fibonacci ends

# Function to check if a string 's' is 1-9 pandigital
# 's' must be exactly 9 characters long and contain digits '1' through '9' once.
def is_pandigital_1_9?(s)
  s.length == 9 && s.chars.sort.join == "123456789"
end

# Initialize Fibonacci sequence variables
# For full Fibonacci numbers
a = 1 # Represents F_1
b = 1 # Represents F_2

# For last 9 digits (modulo 10^9)
MOD = 1_000_000_000
a_last9 = 1 # Represents F_1 % MOD
b_last9 = 1 # Represents F_2 % MOD

# k is the index of the Fibonacci number.
# We start with F_1, F_2 already defined, so the loop calculates F_3, F_4, ...
k = 2 

loop do
  k += 1 # k is now the index of F_k being computed

  # Calculate next Fibonacci term for the full number
  c_val = a + b 
  a = b
  b = c_val

  # Calculate next Fibonacci term for the last 9 digits
  c_last9_val = (a_last9 + b_last9) % MOD
  a_last9 = b_last9
  b_last9 = c_last9_val
  
  # Check conditions for F_k (which is current 'b' and 'b_last9')
  
  # 1. Check pandigital property for the last nine digits
  # The number formed by the last 9 digits of F_k must be 1-9 pandigital.
  # For b_last9 to be 1-9 pandigital, its string representation must be 9 digits long.
  # This means b_last9 must be >= 100_000_000.
  # The is_pandigital_1_9? function's s.length == 9 check handles this implicitly.
  # If b_last9 < 100_000_000, b_last9.to_s will have < 9 digits.
  last_9_digits_str = b_last9.to_s
  
  if is_pandigital_1_9?(last_9_digits_str)
    # If last 9 digits are pandigital, then check the first 9 digits of the full F_k
    
    # 2. Check pandigital property for the first nine digits
    full_fib_str = b.to_s # Convert the full F_k to a string
    
    if full_fib_str.length >= 9 # F_k must have at least 9 digits
      first_9_digits_str = full_fib_str[0...9] # Extract the first 9 characters
      
      if is_pandigital_1_9?(first_9_digits_str)
        # Both conditions met
        puts k # Output the index k
        break  # Terminate the loop
      end
    end
  end
  
  # Optional: Print progress for very long runs (can slow down execution significantly)
  # if k % 10000 == 0
  #   puts "Checked k = #{k}"
  # end
end
```
