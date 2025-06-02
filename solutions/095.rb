#  <p>The proper divisors of a number are all the divisors excluding the number its
# elf. For example, the proper divisors of $28$ are $1$, $2$, $4$, $7$, and $14$.
# As the sum of these divisors is equal to $28$, we call it a perfect number.</p>
# <p>Interestingly the sum of the proper divisors of $220$ is $284$ and the sum of
#  the proper divisors of $284$ is $220$, forming a chain of two numbers. For this
#  reason, $220$ and $284$ are called an amicable pair.</p>
# <p>Perhaps less well known are longer chains. For example, starting with $12496$
# , we form a chain of five numbers:
# $$12496 \to 14288 \to 15472 \to 14536 \to 14264 (\to 12496 \to \cdots)$$</p>
# <p>Since this chain returns to its starting point, it is called an amicable chai
# n.</p>
# <p>Find the smallest member of the longest amicable chain with no element exceed
# ing one million.</p>

# Find the smallest member of the longest amicable chain with no element exceeding one million.

def solve
  limit = 1_000_000

  # Step 1: Precompute sum of proper divisors (sps) for numbers up to limit.
  # sps[n] will store the sum of proper divisors of n.
  # Initialize sps array with all zeros. sps[1] will correctly remain 0.
  sps = Array.new(limit + 1, 0)

  (1..limit).each do |i| # i is the potential divisor
    # Iterate through multiples of i, starting from 2*i, up to the limit.
    # Add i to the sum of proper divisors for each multiple j.
    (2 * i..limit).step(i).each do |j|
      sps[j] += i
    end
  end
  # Example: sps[6] will get 1 (from i=1), 2 (from i=2), 3 (from i=3) => sps[6]=6.
  # Example: sps[prime_p] will get 1 (from i=1) => sps[prime_p]=1.

  max_chain_len = 0
  smallest_member_of_longest_chain = nil # Use nil for uninitialized state

  # processed_info[k] stores { len: L, min_val: M } if k is part of a cycle,
  # or { len: 0 } if k leads to termination or an already processed non-maximal segment.
  processed_info = {}

  (1..limit).each do |start_node|
    next if processed_info.key?(start_node) # Skip if this number has already been processed

    current_path = [] # Stores numbers in the current chain being explored
    path_set = {}     # Stores num => index_in_current_path for quick cycle detection

    current_num = start_node

    loop do
      # Termination Condition 1: Number is out of bounds or is a known terminator.
      # sps[1] = 0, so a chain reaching 1 will go to 0 in the next step.
      # Numbers > limit break the chain.
      if current_num == 0 || current_num > limit
        current_path.each do |num_in_path|
          # Mark as terminated only if not already part of a valid processed chain.
          processed_info[num_in_path] = { len: 0 } unless processed_info.key?(num_in_path)
        end
        break # End current chain exploration
      end

      # Termination Condition 2: Number leads into an already analyzed segment.
      if processed_info.key?(current_num)
        # The current path leads to a segment whose fate is known.
        # These path elements do not form a new "best" cycle starting from start_node.
        current_path.each do |num_in_path|
          processed_info[num_in_path] = { len: 0 } unless processed_info.key?(num_in_path)
        end
        break # End current chain exploration
      end

      # Cycle Detection: Current number is already in the path being built.
      if path_set.key?(current_num)
        cycle_start_index = path_set[current_num]
        cycle_nodes = current_path[cycle_start_index..-1]
        cycle_len = cycle_nodes.length
        
        # All elements in cycle_nodes are guaranteed to be <= limit,
        # because a number is added to current_path only if it's within bounds.
        
        current_cycle_min_member = cycle_nodes.min

        if cycle_len > max_chain_len
          max_chain_len = cycle_len
          smallest_member_of_longest_chain = current_cycle_min_member
        elsif cycle_len == max_chain_len
          # If multiple chains have the same max length, choose the one with the smallest minimum member.
          if smallest_member_of_longest_chain.nil? || current_cycle_min_member < smallest_member_of_longest_chain
            smallest_member_of_longest_chain = current_cycle_min_member
          end
        end

        # Mark all members of this identified cycle in processed_info.
        # Store the length and the actual smallest member of *this* cycle.
        cycle_nodes.each do |node_in_cycle|
          processed_info[node_in_cycle] = { len: cycle_len, min_val: current_cycle_min_member }
        end
        
        # Mark numbers in current_path *before* this cycle as leading to it (not part of this cycle).
        current_path[0...cycle_start_index].each do |node_before_cycle|
          processed_info[node_before_cycle] = { len: 0 } unless processed_info.key?(node_before_cycle)
        end
        break # Cycle processed, end current chain exploration
      end

      # No termination or cycle yet, continue building the path.
      current_path.push(current_num)
      path_set[current_num] = current_path.length - 1 # Record index of current_num in path

      # Move to the next number in the chain.
      # current_num is guaranteed to be in [1, limit] at this point.
      next_num = sps[current_num]
      current_num = next_num
    end
  end

  smallest_member_of_longest_chain
end

# To execute the solution:
result = solve
puts "The smallest member of the longest amicable chain is: #{result}"