#  <p>Let $p(n)$ represent the number of different ways in which $n$ coins can be s
# eparated into piles. For example, five coins can be separated into piles in exac
# tly seven different ways, so $p(5)=7$.</p>
# <div class="margin_left">
# OOOOO<br>
# OOOO   O<br>
# OOO   OO<br>
# OOO   O   O<br>
# OO   OO   O<br>
# OO   O   O   O<br>
# O   O   O   O   O
# </div>
# <p>Find the least value of $n$ for which $p(n)$ is divisible by one million.</p>

# Solution for Project Euler Problem 78

# We use Euler's Pentagonal Number Theorem to calculate p(n), the number of partitions of n.
# The recurrence relation is:
# p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + p(n-15) - ...
# The numbers 1, 2, 5, 7, 12, 15, ... are generalized pentagonal numbers g_k = k(3k-1)/2 for k = 1, -1, 2, -2, ...
# Or, for k = 1, 2, 3, ... :
#   g_k_pos = k(3k-1)/2  (terms for k=1, 2, 3, ...)
#   g_k_neg = k(3k+1)/2  (terms for k=-1, -2, -3, ... by substituting k with -k in k(3k-1)/2)
# The signs of the terms p(n-g_k) in the sum alternate ++--++-- corresponding to k=1, k=2, k=3, ...

TARGET_DIVISOR = 1_000_000

# partitions_mod_divisor[i] will store p(i) % TARGET_DIVISOR
# Initialize with p(0) = 1. We can use an array and dynamically extend if needed,
# or pre-size if we have a good estimate (e.g., 70,000).
# Ruby arrays grow dynamically, so we can just append or assign by index.
partitions_mod_divisor = [1] # p(0) = 1

n = 0
loop do
  n += 1
  current_P_n = 0
  k = 1 # k for generalized pentagonal numbers

  loop do
    # Pentagonal number for k
    pentagonal1 = k * (3 * k - 1) / 2
    # Pentagonal number for -k (substituting -k into k(3k-1)/2 gives k(3k+1)/2)
    pentagonal2 = k * (3 * k + 1) / 2

    # Determine the sign based on k. For k=1,2,3,4,... signs are +1,+1,-1,-1,...
    # Sign is (-1)^(k-1). If k is odd, sign is +1. If k is even, sign is -1.
    sign = (k.odd? ? 1 : -1)

    # Term from pentagonal1 = k(3k-1)/2
    if n - pentagonal1 >= 0
      term1_val = partitions_mod_divisor[n - pentagonal1]
      current_P_n += sign * term1_val
    else
      # If n - pentagonal1 < 0, this and all subsequent pentagonal numbers are too large.
      # This is the primary break condition for the inner k-loop.
      break
    end

    # Term from pentagonal2 = k(3k+1)/2
    if n - pentagonal2 >= 0
      term2_val = partitions_mod_divisor[n - pentagonal2]
      current_P_n += sign * term2_val
    end
    # No explicit break here, as pentagonal1 <= n might still allow next k's pentagonal1.
    # The break from pentagonal1 > n handles termination correctly.

    k += 1
  end

  # Ensure the result is a non-negative value modulo TARGET_DIVISOR
  # Ruby's %: -5 % 100 = 95 if 100 is positive.
  # However, if current_P_n becomes very negative, e.g. -1_000_005 % 1_000_000 = -5
  # So, (val % D + D) % D is the most robust way to ensure positive modulo.
  pn_mod = (current_P_n % TARGET_DIVISOR + TARGET_DIVISOR) % TARGET_DIVISOR
  partitions_mod_divisor[n] = pn_mod
  
  # Check if p(n) is divisible by TARGET_DIVISOR
  if pn_mod == 0
    puts "The least value of n for which p(n) is divisible by #{TARGET_DIVISOR} is: #{n}"
    break
  end

  # Safety break for very long runs, though problem implies a solution will be found.
  # if n > 70000 # A reasonable upper bound if solution wasn't found quickly
  #   puts "Reached n=#{n}, p(n) mod #{TARGET_DIVISOR} = #{pn_mod}. Consider increasing search limit."
  #   break
  # end
end
