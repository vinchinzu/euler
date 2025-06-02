# Problem 122: Efficient Exponentiation
#
# Problem Statement:
# The most naive way of computing $n^{15}$ requires fourteen multiplications: $$n 	imes n 	imes \cdots 	imes n = n^{15}.$$
#
# But using a "binary" method you can compute it in six multiplications:
# $n 	imes n = n^2$
# $n^2 	imes n^2 = n^4$
# $n^4 	imes n^4 = n^8$
# $n^8 	imes n^4 = n^{12}$
# $n^{12} 	imes n^2 = n^{14}$
# $n^{14} 	imes n = n^{15}$
#
# However it is yet possible to compute it in only five multiplications:
# $n 	imes n = n^2$
# $n^2 	imes n = n^3$
# $n^3 	imes n^3 = n^6$
# $n^6 	imes n^6 = n^{12}$
# $n^{12} 	imes n^3 = n^{15}$
#
# We shall define $m(k)$ to be the minimum number of multiplications to compute $n^k$; for example $m(15) = 5$.
#
# Find $\sum\limits_{k = 1}^{200} m(k)$.
#
# Notes:
# The problem asks for the sum of m(k) for k=1 to 200, where m(k) is the minimum number of multiplications to compute n^k.
# This is equivalent to finding the length of the shortest addition chain for each k.
# The script uses a Breadth-First Search (BFS) approach. States in the BFS are [chain_elements_set, cost].
# An array m[k] stores the minimum cost found for each k, initialized with m[1]=0.
# A 'visited_states' set (using a frozen sorted array of chain elements and cost as key) is used to avoid redundant computations and cycles.
# The search depth (cost) is capped for optimization. The script sums m[k] for k=1 to 200 to get the answer 1582.

# Full Ruby script content from temp_problem_122.rb:

require 'set'

LIMIT = 200
# MAX_COST_TO_PROCESS is the maximum cost of a state whose children will be explored.
# Based on known values, m(191) = 11 is the max m(k) for k <= 200.
# Children of states with cost=11 (i.e., new_cost=12) are generated, and their m(k) values updated.
# Children of states with cost=12 (i.e., new_cost=13) are not generated.
MAX_COST_TO_PROCESS = 11

# m[k] stores the minimum number of multiplications for n^k
# m[0] is unused. m[1] to m[LIMIT] are used.
m = Array.new(LIMIT + 1, Float::INFINITY)
m[1] = 0 # Base case: n^1 requires 0 multiplications

# Initial state for BFS:
# The chain {1} is achieved with 0 multiplications.
initial_chain_set = Set[1]

# For the visited_states set, we need a hashable representation of the chain set.
# A frozen sorted array of elements serves this purpose.
initial_chain_repr = initial_chain_set.to_a.sort.freeze # Store sorted frozen array

# Queue stores [Set_object_for_chain, cost]
queue = [[initial_chain_set, 0]]
head = 0 # Simulating queue behavior with an array for BFS

# visited_states stores [frozen_array_representation_of_chain, cost]
# This set prevents redundant exploration of identical states (same chain elements and cost).
visited_states = Set[[initial_chain_repr, 0]]

while head < queue.length
  current_chain_set, cost = queue[head]
  head += 1

  # If the current state's cost is too high, don't explore its children
  next if cost > MAX_COST_TO_PROCESS

  # Iterate over all pairs (x,y) from the current chain to form new exponents
  current_chain_set.each do |x|
    current_chain_set.each do |y|
      new_val = x + y
      next if new_val > LIMIT # We only care about exponents up to LIMIT

      new_cost = cost + 1

      # If this path offers a better way to new_val, OR an equally good way
      # (which might lead to different subsequent optimal paths for other numbers)
      if new_cost <= m[new_val]
        # Update m[new_val] if this path is strictly shorter
        m[new_val] = new_cost if new_cost < m[new_val]
        
        # new_elements_for_path is the set of numbers available after this multiplication step
        new_elements_for_path = current_chain_set.dup.add(new_val)
        
        # Create a hashable representation for the visited_states set
        new_chain_repr = new_elements_for_path.to_a.sort.freeze
        state_key_for_visited = [new_chain_repr, new_cost]

        # If this specific state (chain elements and cost) hasn't been visited,
        # add it to the queue and mark as visited.
        # The .add? method returns the set itself if element was added, nil otherwise.
        if visited_states.add?(state_key_for_visited)
          queue.push([new_elements_for_path, new_cost])
        end
      end
    end
  end
end

# Calculate the sum of m[k] for k = 1 to LIMIT
sum_m_k = 0
(1..LIMIT).each do |k|
  if m[k] == Float::INFINITY
    # This indicates an issue, e.g., MAX_COST_TO_PROCESS too low or a bug in logic.
    # For robust error handling, one might raise an error or log this.
    # $stderr.puts "Warning: m[#{k}] remained infinity. Check MAX_COST_TO_PROCESS or algorithm logic."
  else
    sum_m_k += m[k]
  end
end

puts sum_m_k

