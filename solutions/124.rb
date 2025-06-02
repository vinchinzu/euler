# Problem 124: Ordered Radicals
#
# Problem Statement:
# The radical of $n$, $\operatorname{rad}(n)$, is the product of the distinct prime factors of $n$. For example, $504 = 2^3 	imes 3^2 	imes 7$, so $\operatorname{rad}(504) = 2 	imes 3 	imes 7 = 42$.
#
# If we calculate $\operatorname{rad}(n)$ for $1 \le n \le 10$, then sort them on $\operatorname{rad}(n)$, and sorting on $n$ if the radical values are equal, we get:
#  Unsorted      Sorted
# n  rad(n)    n  rad(n) k
# 1  1         1  1      1
# 2  2         2  2      2
# 3  3         4  2      3
# 4  2         8  2      4
# 5  5         3  3      5
# 6  6         9  3      6
# 7  7         5  5      7
# 8  2         6  6      8
# 9  3         7  7      9
# 10 10        10 10     10
#
# Let $E(k)$ be the $k$-th element in the sorted $n$ column; for example, $E(4) = 8$ and $E(6) = 9$.
#
# If $\operatorname{rad}(n)$ is sorted for $1 \le n \le 100000$, find $E(10000)$.
#
# Notes:
# The problem asks for E(10000), the n-value of the 10000th term when numbers from 1 to 100000 are sorted by their radical, then by n itself.
# The script first calculates rad(n) for all n up to 100000 using a sieve-like method.
# (Initialize rad[j]=1; for each prime i, iterate multiples j=i,2i,... and multiply rad[j] by i).
# It then creates a list of numbers from 1 to 100000.
# This list is sorted: primary key is rad(n) (looked up from the precomputed array), secondary key is n.
# The n-value of the element at index 9999 (0-indexed for E(10000)) is selected.
# The solution found is 21417.

# Full Ruby script content from temp_problem_124.rb:

class Problem124Solver
  LIMIT = 100_000
  TARGET_K = 10_000 # We need E(10000)

  # Calculates rad(n) for all n from 1 to limit.
  # rad(n) is the product of distinct prime factors of n.
  # Uses a sieve-like method.
  def calculate_radicals(limit)
    # rads[i] will store rad(i). Initialize all to 1.
    # rads[0] is unused. rads[1] = 1 is correct by initialization.
    rads = Array.new(limit + 1, 1)

    (2..limit).each do |i|
      if rads[i] == 1 # This means 'i' is prime, as it hasn't been multiplied by smaller prime factors.
        # For this prime 'i', iterate through all its multiples j = i, 2i, 3i, ... up to limit.
        (i..limit).step(i) do |j|
          rads[j] *= i # Multiply rads[j] by the distinct prime factor i.
        end
      end
    end
    rads
  end

  def solve
    # Step a: Implement a method to calculate rad(n) for all n up to 100000.
    radicals = calculate_radicals(LIMIT)

    # Step b: Create a list of pairs [n, rad(n)] for n=1, ..., 100000.
    # Alternatively, we can create a list of n values and use the `radicals` array during sort.
    # Let's create an array of numbers [1, 2, ..., LIMIT] to be sorted.
    numbers_to_sort = (1..LIMIT).to_a

    # Step c: Sort this list according to the specified criteria.
    # Primary sort key: rad(n) value.
    # Secondary sort key: n value.
    # The `sort_by` method allows specifying keys for sorting.
    # For each number `n_val` in `numbers_to_sort`, its sort keys are `[radicals[n_val], n_val]`.
    sorted_n_values = numbers_to_sort.sort_by do |n_val|
      [radicals[n_val], n_val]
    end

    # Step d: Extract the n-value from the element at index 9999 (for E(10000)).
    # The sorted_n_values array is 0-indexed. So, the k-th element is at index k-1.
    result_n_value = sorted_n_values[TARGET_K - 1]

    # The script must puts this n-value.
    puts result_n_value
  end
end

# Create an instance of the solver and run the solve method.
solver = Problem124Solver.new
solver.solve

