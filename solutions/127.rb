# Problem 127: abc-hits
#
# Problem Statement:
# The radical of $n$, $\operatorname{rad}(n)$, is the product of distinct prime factors of $n$. For example, $504 = 2^3 	imes 3^2 	imes 7$, so $\operatorname{rad}(504) = 2 	imes 3 	imes 7 = 42$.
#
# We shall define the triplet of positive integers $(a, b, c)$ to be an abc-hit if:
# 1. $\gcd(a, b) = \gcd(a, c) = \gcd(b, c) = 1$
# 2. $a < b$
# 3. $a + b = c$
# 4. $\operatorname{rad}(abc) < c$
#
# For example, $(5, 27, 32)$ is an abc-hit, because:
# 1. $\gcd(5, 27) = \gcd(5, 32) = \gcd(27, 32) = 1$
# 2. $5 < 27$
# 3. $5 + 27 = 32$
# 4. $\operatorname{rad}(4320) = 30 < 32$
#
# It turns out that abc-hits are quite rare and there are only thirty-one abc-hits for $c < 1000$, with $\sum c = 12523$.
#
# Find $\sum c$ for $c < 120000$.
#
# Notes:
# An abc-hit (a,b,c) satisfies: a,b,c are positive, gcd(a,b)=1 (implies pairwise coprime for a+b=c), a<b, a+b=c, and rad(a)rad(b)rad(c) < c.
# The script calculates rad(n) for n up to 120000 using a sieve.
# It iterates c from 2 to 119999.
# Optimization: Skips c if rad(c) = c (c is square-free and c>1), as this would require rad(a)rad(b)<1, impossible.
# Inner loop iterates a from 1 to (c-1)/2. Let b = c-a.
# Optimizations for a: skip if rad(a)rad(c) >= c, or if rad(a)rad(b)rad(c) >= c.
# If gcd(a,b)=1 (and the product of radicals is < c), c is added to the sum.
# The solution found is 18407904.

# Full Ruby script content from temp_problem_127.rb:

class Problem127Solver
  LIMIT_C = 120_000

  # Pre-calculates rad(n) for n=1 to limit.
  # rad(n) is the product of distinct prime factors of n.
  def calculate_radicals(limit)
    rad = Array.new(limit + 1, 1)
    # rad[0] is not used for this problem's context (n > 0).
    # rad[1] is correctly 1 by initialization.

    (2..limit).each do |i|
      if rad[i] == 1 # 'i' is prime, as its radical is still 1.
        (i..limit).step(i) do |j| # Iterate through all multiples of i.
          rad[j] *= i # Multiply rad[j] by the distinct prime factor i.
        end
      end
    end
    rad
  end

  # Calculates Greatest Common Divisor using Euclidean algorithm.
  def gcd(x, y)
    while y != 0
      x, y = y, x % y
    end
    x
  end

  def solve
    # Step b: Pre-calculate rad(n) for n=1 to LIMIT_C
    radicals = calculate_radicals(LIMIT_C)
    
    # Step d: Initialize sum_of_c = 0
    sum_of_c = 0

    # Step e: Loop c from 2 up to LIMIT_C - 1
    (2...LIMIT_C).each do |c|
      # Step e.i: Get rad_c = rad[c]
      rad_c = radicals[c]

      # Step e.ii: Optimization
      # If rad_c == c (and c > 1), c is square-free.
      # The condition rad(a)rad(b)rad(c) < c becomes rad(a)rad(b)c < c,
      # which implies rad(a)rad(b) < 1. This is impossible as rad values are >= 1.
      # (c > 1 is implicit from loop start)
      next if rad_c == c 

      # Step f: Loop a from 1 up to (c-1) / 2
      # This ensures a < c/2, which implies a < b since b = c-a.
      limit_a = (c - 1) / 2 # Integer division
      (1..limit_a).each do |a|
        # Step f.i: Get rad_a = rad[a]
        rad_a = radicals[a]

        # Step f.ii: Optimization
        # If rad_a * rad_c >= c, then rad_a * rad_b * rad_c >= c (since rad_b >= 1).
        # So the condition rad(a)rad(b)rad(c) < c cannot be met.
        next if rad_a * rad_c >= c
        
        # Step f.iii: Let b = c-a
        b = c - a
        # (No need to check a < b, loop structure for 'a' ensures it)
        # (No need to check gcd(a,c)=1 or gcd(b,c)=1 yet, comes with gcd(a,b))

        rad_b = radicals[b] # Get rad[b]

        # Step f.iv: If rad_a * rad[b] * rad_c >= c, continue to next a.
        # This is the check for the core condition rad(abc) < c.
        # If this condition is false (i.e., product >= c), skip.
        next if rad_a * rad_b * rad_c >= c
        
        # Step f.v: If gcd(a,b) == 1:
        # (At this point, we've passed f.iv, so rad_a * rad_b * rad_c < c is true)
        if gcd(a, b) == 1
          # (a,b,c) is an abc-hit. Add c to sum.
          # Note: gcd(a,b)=1 and a+b=c implies a,b,c are pairwise coprime.
          sum_of_c += c
        end
      end
    end

    # Script must puts the final sum_of_c
    puts sum_of_c
  end
end

# Ensure the script is runnable when executed directly
if __FILE__ == $PROGRAM_NAME
  solver = Problem127Solver.new
  solver.solve
end

