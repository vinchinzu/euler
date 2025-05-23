#  <p>A common security method used for online banking is to ask the user for three
#  random characters from a passcode. For example, if the passcode was 531278, the
# y may ask for the 2nd, 3rd, and 5th characters; the expected reply would be: 317
# .</p>
# <p>The text file, <a href="resources/documents/0079_keylog.txt">keylog.txt</a>,
# contains fifty successful login attempts.</p>
# <p>Given that the three characters are always asked for in order, analyse the fi
# le so as to determine the shortest possible secret passcode of unknown length.</
# p>

# Solution for Project Euler Problem 79
require 'set'

KEYLOG_DATA = [
  "319", "680", "180", "690", "129", "620", "762", "689", "762", "318",
  "368", "710", "720", "710", "629", "168", "160", "689", "716", "731",
  "736", "729", "316", "729", "729", "710", "769", "290", "719", "680",
  "318", "389", "162", "289", "162", "718", "729", "319", "790", "680",
  "890", "362", "319", "760", "316", "729", "380", "319", "728", "716"
].freeze

# Graph representation
nodes = Set.new # Set of all unique characters (digits)
adj = Hash.new { |h, k| h[k] = Set.new } # Adjacency list: char -> Set of chars it precedes
in_degree = Hash.new(0) # In-degree of each character

# Step 1: Construct the graph
KEYLOG_DATA.each do |attempt|
  d1, d2, d3 = attempt.chars

  # Add all characters to the set of nodes
  nodes.add(d1)
  nodes.add(d2)
  nodes.add(d3)

  # Add edges and update in-degrees as per problem statement's interpretation
  # Edge d1 -> d2
  if adj[d1].add?(d2) # If edge is new
    in_degree[d2] += 1
  end
  
  # Edge d1 -> d3
  if adj[d1].add?(d3) # If edge is new
    in_degree[d3] += 1
  end

  # Edge d2 -> d3
  if adj[d2].add?(d3) # If edge is new
    in_degree[d3] += 1
  end
end

# Ensure all nodes are in in_degree hash (Hash.new(0) handles this for reads,
# but explicit iteration is good for clarity if needed for queue initialization)
# nodes.each { |node| in_degree[node] ||= 0 } # Not strictly needed with Hash.new(0) if only reading

# Step 2: Topological Sort (Kahn's Algorithm)
queue = []
nodes.each do |node|
  if in_degree[node] == 0
    queue << node
  end
end

# Sort initial queue to ensure deterministic output if multiple starting nodes exist.
# For this problem, it's likely there's only one or a few, and order might not matter
# if the graph structure is simple enough. However, sorting is good practice.
queue.sort! 

passcode_chars = []
while !queue.empty?
  u = queue.shift # Dequeue
  passcode_chars << u

  # Sort neighbors to process them in a deterministic order.
  # This can affect the specific topological sort output if multiple valid sorts exist,
  # but for a unique shortest passcode, this should converge.
  # For this problem, the order of processing neighbors of u (if u has multiple)
  # doesn't change the final unique passcode if one exists.
  sorted_neighbors = adj[u].to_a.sort 
  
  sorted_neighbors.each do |v|
    in_degree[v] -= 1
    if in_degree[v] == 0
      queue << v
      # If maintaining a specific order in queue is important (e.g. for multiple solutions)
      # one might sort the queue here. For Kahn's, just adding is fine.
      # For this problem, sorting the initial queue is more impactful for determinism.
    end
  end
  queue.sort! # Re-sort queue if new elements added, to maintain deterministic processing
end

# Step 3: Check and Print Result
if passcode_chars.length == nodes.size
  passcode = passcode_chars.join
  puts "The shortest possible secret passcode is: #{passcode}"
else
  # This case implies a cycle in the graph, which shouldn't happen for this problem.
  puts "Error: Could not determine passcode (possible cycle in graph or missing data)."
  puts "Processed nodes: #{passcode_chars.join}"
  puts "Total unique nodes: #{nodes.size}"
end
