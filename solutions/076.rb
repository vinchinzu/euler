#  <p>It is possible to write five as a sum in exactly six different ways:</p>
# \begin{align}
# &amp;4 + 1\\
# &amp;3 + 2\\
# &amp;3 + 1 + 1\\
# &amp;2 + 2 + 1\\
# &amp;2 + 1 + 1 + 1\\
# &amp;1 + 1 + 1 + 1 + 1
# \end{align}
# <p>How many different ways can one hundred be written as a sum of at least two p
# ositive integers?</p>

# Solution for Project Euler Problem 76

# This is a problem of finding the number of partitions of 100 into at least two parts.
# We can use dynamic programming. Let ways[i] be the number of ways to make sum 'i'.
# The parts we can use are integers from 1 up to TARGET_SUM - 1 (i.e., 1 to 99).
# If we used parts up to TARGET_SUM, ways[TARGET_SUM] would be the total number of
# partitions of TARGET_SUM, including the partition {TARGET_SUM} itself.
# By restricting the parts to be less than TARGET_SUM, we inherently exclude the
# partition {TARGET_SUM}, thus directly calculating partitions into at least two parts.

TARGET_SUM = 100

# ways[i] will store the number of ways to make sum 'i' using the allowed parts.
# Initialize ways[0] = 1 (one way to make sum 0: use no parts).
# All other ways[j] are initialized to 0.
ways = Array.new(TARGET_SUM + 1, 0)
ways[0] = 1

# Loop through each part (coin) that can be used in the sum.
# Since the sum must be of AT LEAST TWO positive integers, the largest single
# part effectively allowed is TARGET_SUM - 1.
(1..(TARGET_SUM - 1)).each do |part|
  # For each part, update the ways to make sums from 'part' up to TARGET_SUM.
  (part..TARGET_SUM).each do |current_sum|
    # The number of ways to make 'current_sum' can be increased by using the current 'part'.
    # This is done by adding the number of ways to make 'current_sum - part' (the remaining sum).
    ways[current_sum] += ways[current_sum - part]
  end
end

# The final answer is the number of ways to make TARGET_SUM using parts up to TARGET_SUM - 1.
result = ways[TARGET_SUM]

puts "Number of different ways one hundred can be written as a sum of at least two positive integers: #{result}"
