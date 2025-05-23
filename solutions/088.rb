#  <p>A natural number, $N$, that can be written as the sum and product of a given
# set of at least two natural numbers, $\{a_1, a_2, \dots, a_k\}$ is called a prod
# uct-sum number: $N = a_1 + a_2 + \cdots + a_k = a_1 \times a_2 \times \cdots \ti
# mes a_k$.</p>
# <p>For example, $6 = 1 + 2 + 3 = 1 \times 2 \times 3$.</p>
# <p>For a given set of size, $k$, we shall call the smallest $N$ with this proper
# ty a minimal product-sum number. The minimal product-sum numbers for sets of siz
# e, $k = 2, 3, 4, 5$, and $6$ are as follows.</p>
# <ul style="list-style-type:none;">
# <li>$k=2$: $4 = 2 \times 2 = 2 + 2$</li>
# <li>$k=3$: $6 = 1 \times 2 \times 3 = 1 + 2 + 3$</li>
# <li>$k=4$: $8 = 1 \times 1 \times 2 \times 4 = 1 + 1 + 2 + 4$</li>
# <li>$k=5$: $8 = 1 \times 1 \times 2 \times 2 \times 2 = 1 + 1 + 2 + 2 + 2$</li><
# li>$k=6$: $12 = 1 \times 1 \times 1 \times 1 \times 2 \times 6 = 1 + 1 + 1 + 1 +
#  2 + 6$</li></ul>
# <p>Hence for $2 \le k \le 6$, the sum of all the minimal product-sum numbers is
# $4+6+8+12 = 30$; note that $8$ is only counted once in the sum.</p>
# <p>In fact, as the complete set of minimal product-sum numbers for $2 \le k \le
# 12$ is $\{4, 6, 8, 12, 15, 16\}$, the sum is $61$.</p>
# <p>What is the sum of all the minimal product-sum numbers for $2 \le k \le 12000
# $?</p>

# Solution for Project Euler Problem 88
require 'set'

K_MAX = 12000
# N_LIMIT is the maximum product value we will explore in the DFS.
# For a given k, the minimal N is often <= 2k. So for k=12000, N could be around 24000.
# We need a buffer for intermediate products in DFS.
# If k consists of j non-1 factors and (k-j) ones, then Product = P_j and Sum = S_j + (k-j)*1.
# P_j = S_j + k - j => k = P_j - S_j + j.
# Max j (number of non-1 factors) for P_j <= N_LIMIT (e.g. 26000) is log2(N_LIMIT) ~ 14-15.
# If k > N_LIMIT (e.g. k=25000, N_LIMIT=26000), this is impossible if all factors are >=2.
# But k can be large due to many 1s.
# The problem asks for minimal N for k. k itself can be up to K_MAX.
# The product N (current_P in DFS) can be bounded.
# N_LIMIT should be at least K_MAX because if k=K_MAX, N=K_MAX for {K_MAX, 1, 1, ...} (not prod-sum)
# Example given: k=6, N=12. {1,1,1,1,2,6}. P=12, S=12.
# Example: k=12000, N=24000. {2, 12000, 1, ..., 1 (11998 ones)}. Prod=24000, Sum=2+12000+11998=23990+10=24000.
# The values stored in min_N_for_k[k] will not exceed 2*k typically. So max N stored is ~24000.
# The DFS search for products can go slightly higher.
N_LIMIT = 2 * K_MAX + 4000 # e.g., 24000 + 4000 = 28000. Max product to explore.

# min_N_for_k[k_val] stores the minimal product-sum number N for a set of size k_val.
# Initialize with infinity. Indices 0 and 1 are not used for k.
min_N_for_k = Array.new(K_MAX + 1, Float::INFINITY)

# Recursive function to find product-sums
# product: current product of non-1 factors
# sum_of_factors: current sum of these non-1 factors
# num_non_one_factors: count of these non-1 factors
# min_factor_to_use: ensures factors are chosen in non-decreasing order
def find_product_sums(product, sum_of_factors, num_non_one_factors, min_factor_to_use, limit_k, limit_n, solutions_array)
  # The loop for f (next factor)
  # The smallest next factor is min_factor_to_use.
  # The largest next factor f: if product * f > limit_n, we stop. So f <= limit_n / product.
  # Also, num_non_one_factors + 1 (for f) + (product*f - (sum_of_factors+f)) ones must be <= limit_k.
  # This simplifies to product*f - sum_of_factors - f + num_non_one_factors + 1 <= limit_k
  # product*f - f <= limit_k + sum_of_factors - num_non_one_factors - 1
  # f * (product - 1) <= ...
  # This is a complex upper bound for f. Simpler to check k inside.
  
  f = min_factor_to_use
  while true
    current_P = product * f
    break if current_P > limit_n # Product exceeds search limit

    current_S_f = sum_of_factors + f
    current_j = num_non_one_factors + 1

    # Number of ones needed to make product = sum
    # P = S_f + num_ones * 1
    # Product = (Sum of non-1 factors) + (Number of 1s)
    num_ones = current_P - current_S_f
    
    if num_ones >= 0 # Product must be >= sum of non-1 factors
      k = current_j + num_ones # Total number of terms in the set {a_i}

      if k <= limit_k
        if current_P < solutions_array[k]
          solutions_array[k] = current_P
        end
      end
    end
    
    # Recursive call: try adding another factor f' >= f
    # Optimization: current_j + 1 (for next factor) must not make k (even with 0 ones) exceed K_MAX too much.
    # Smallest k possible for current_P, current_S_f, current_j is when num_ones=0 => k=current_j.
    # If current_j (number of non-1 factors) itself is > K_MAX, we can potentially stop early.
    # However, k can be smaller than current_j if num_ones is negative, but we filter num_ones >=0.
    # The condition current_P - current_S_f (num_ones) + current_j (num non-1s) gives k.
    # If current_j alone (minimum possible k for this path if num_ones=0) > K_MAX, and we require num_ones >=0,
    # then k will always be > K_MAX. But this is already handled by `if k <= limit_k`.
    # The critical path length constraint is for the number of non-1 factors.
    # Maximum number of non-1 factors is log2(N_LIMIT), e.g., log2(28000) ~ 14.7.
    # So, if current_j >= 15 (approx), further recursion is not useful.
    # The problem specifies "at least two natural numbers", so k >= 2 always.
    # Our loop for k starts from 2.
    if current_j < 2 * K_MAX.to_s(2).length # Heuristic limit on number of factors, approx 2*log2(K_MAX)
        find_product_sums(current_P, current_S_f, current_j, f, limit_k, limit_n, solutions_array)
    end

    f += 1
  end
end

# Initial call to the recursive function.
# We start with a product of 1, sum of 0, 0 non-1 factors, and the smallest factor to use is 2.
# The problem constraints imply k >= 2. The find_product_sums will calculate k values.
# The first factor 'f' taken will make num_non_one_factors = 1.
# e.g. f=4: P=4, S=4, j=1. num_ones = 4-4=0. k = 1+0=1. Not stored.
# Recurse: find_product_sums(4,4,1,4).
#   Next f=?: if f=1 (not allowed by min_factor=4), if f=2 (not allowed).
# If initial call is find_product_sums(f, f, 1, f) for f in 2..N_LIMIT/2?
# The prompt's structure `find_product_sums(1,0,0,2)` is common.
# It means the "set of factors" starts empty, and we add factors >= 2.
# Example: Call(1,0,0,2)
#   f=2: P=2, S=2, j=1. num_ones=0. k=1. Recurse(2,2,1,2)
#     f=2: P=4, S=4, j=2. num_ones=0. k=2. min_N_for_k[2]=min(inf,4)=4. Recurse(4,4,2,2)
#       f=?: P*f > N_LIMIT or other conditions.
#     f=3: P=6, S=5, j=2. num_ones=1. k=2+1=3. min_N_for_k[3]=min(inf,6)=6. Recurse(6,5,2,3)
# This seems correct.

find_product_sums(1, 0, 0, 2, K_MAX, N_LIMIT, min_N_for_k)

# Collect unique values from min_N_for_k[2] through min_N_for_k[K_MAX]
# and sum them.
# Values that remained Float::INFINITY mean no product-sum number was found for that k
# within the N_LIMIT for products. This should not happen for k <= K_MAX based on problem structure.
unique_minimal_Ns = Set.new
(2..K_MAX).each do |k_val|
  if min_N_for_k[k_val] != Float::INFINITY
    unique_minimal_Ns.add(min_N_for_k[k_val])
  end
end

total_sum = unique_minimal_Ns.sum

puts "The sum of all minimal product-sum numbers for 2 <= k <= #{K_MAX} is: #{total_sum}"
