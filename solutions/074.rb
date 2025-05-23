#  <p>The number $145$ is well known for the property that the sum of the factorial
#  of its digits is equal to $145$:
# $$1! + 4! + 5! = 1 + 24 + 120 = 145.$$</p>
# <p>Perhaps less well known is $169$, in that it produces the longest chain of nu
# mbers that link back to $169$; it turns out that there are only three such loops
#  that exist:</p>
# \begin{align}
# &amp;169 \to 363601 \to 1454 \to 169\\
# &amp;871 \to 45361 \to 871\\
# &amp;872 \to 45362 \to 872
# \end{align}
# <p>It is not difficult to prove that EVERY starting number will eventually get s
# tuck in a loop. For example,</p>
# \begin{align}
# &amp;69 \to 363600 \to 1454 \to 169 \to 363601 (\to 1454)\\
# &amp;78 \to 45360 \to 871 \to 45361 (\to 871)\\
# &amp;540 \to 145 (\to 145)
# \end{align}
# <p>Starting with $69$ produces a chain of five non-repeating terms, but the long
# est non-repeating chain with a starting number below one million is sixty terms.
# </p>
# <p>How many chains, with a starting number below one million, contain exactly si
# xty non-repeating terms?</p>

# Solution for Project Euler Problem 74

# Precompute factorials for digits 0-9
FACTORIALS = {
  0 => 1, 1 => 1, 2 => 2, 3 => 6, 4 => 24,
  5 => 120, 6 => 720, 7 => 5040, 8 => 40320, 9 => 362_880
}.freeze

# Function to calculate the sum of the factorials of the digits of n
def sum_digit_factorials(n, factorials_map)
  sum = 0
  # Efficiently get digits:
  # Special case for 0, though problem context implies n > 0 for chains.
  return factorials_map[0] if n == 0 
  temp_n = n
  while temp_n > 0
    sum += factorials_map[temp_n % 10]
    temp_n /= 10
  end
  sum
end

# Function to get the length of the digit factorial chain starting from start_node
# Uses memoization via chain_length_cache
def get_chain_length(start_node, factorials_map, chain_length_cache)
  # If start_node's chain length is already known, return it
  return chain_length_cache[start_node] if chain_length_cache.key?(start_node)

  current_path = [] # Stores nodes in the current exploration path
  current_num = start_node

  loop do
    if chain_length_cache.key?(current_num)
      # Case 1: Hit a node whose full chain length is already known
      length_from_current_num = chain_length_cache[current_num]
      current_path.reverse_each.with_index do |path_node, j|
        # j is 0 for the last element added to current_path, 1 for second last, etc.
        # distance from path_node to current_num is j + 1
        # So length of chain from path_node is (j + 1) + length_from_current_num
        chain_length_cache[path_node] = (j + 1) + length_from_current_num
      end
      break # Chain lengths for all nodes in current_path are now cached
    elsif current_path.include?(current_num)
      # Case 2: Detected a loop within the current exploration path
      loop_start_index = current_path.index(current_num)
      cycle_len = current_path.length - loop_start_index

      # Nodes within the cycle all have length 'cycle_len'
      (loop_start_index...current_path.length).each do |j|
        chain_length_cache[current_path[j]] = cycle_len
      end

      # Nodes in current_path leading into this cycle
      (0...loop_start_index).each do |j|
        # distance from current_path[j] to start of cycle is (loop_start_index - j)
        chain_length_cache[current_path[j]] = (loop_start_index - j) + cycle_len
      end
      break # Chain lengths for all nodes in current_path are now cached
    else
      # Case 3: Continue tracing the chain
      current_path << current_num
      current_num = sum_digit_factorials(current_num, factorials_map)
    end
  end
  
  # The cache for start_node must have been populated by one of the break conditions
  chain_length_cache[start_node]
end

# Main logic
LIMIT = 1_000_000
TARGET_LENGTH = 60
count_of_sixty_length_chains = 0
chain_length_cache = {} # Memoization cache for chain lengths

# Specific known loops can be pre-populated if desired, though the general logic handles them.
# chain_length_cache[169] = 3
# chain_length_cache[363601] = 3
# chain_length_cache[1454] = 3
# chain_length_cache[871] = 2
# chain_length_cache[45361] = 2
# chain_length_cache[872] = 2
# chain_length_cache[45362] = 2
# chain_length_cache[145] = 1
# chain_length_cache[1] = 1 # 1! = 1
# chain_length_cache[2] = 1 # 2! = 2
# chain_length_cache[40585] = 1 # 4!+0!+5!+8!+5! = 24+1+120+40320+120 = 40585

(1...LIMIT).each do |i|
  # The problem asks for starting numbers *below* one million.
  chain_len = get_chain_length(i, FACTORIALS, chain_length_cache)
  if chain_len == TARGET_LENGTH
    count_of_sixty_length_chains += 1
  end
end

puts "Number of chains with exactly sixty non-repeating terms: #{count_of_sixty_length_chains}"
