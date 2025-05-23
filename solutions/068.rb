# <p>Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, an
# d each line adding to nine.</p>
# <div class="center">
# <img src="resources/images/0068_1.png?1678992052" class="dark_img" alt=""><br></
# div>
# <p>Working <b>clockwise</b>, and starting from the group of three with the numer
# ically lowest external node (4,3,2 in this example), each solution can be descri
# bed uniquely. For example, the above solution can be described by the set: 4,3,2
# ; 6,2,1; 5,1,3.</p>
# <p>It is possible to complete the ring with four different totals: 9, 10, 11, an
# d 12. There are eight solutions in total.</p>
# <div class="center">
# <table width="400" cellspacing="0" cellpadding="0"><tr><td width="100"><b>Total<
# /b></td><td width="300"><b>Solution Set</b></td>
# </tr><tr><td>9</td><td>4,2,3; 5,3,1; 6,1,2</td>
# </tr><tr><td>9</td><td>4,3,2; 6,2,1; 5,1,3</td>
# </tr><tr><td>10</td><td>2,3,5; 4,5,1; 6,1,3</td>
# </tr><tr><td>10</td><td>2,5,3; 6,3,1; 4,1,5</td>
# </tr><tr><td>11</td><td>1,4,6; 3,6,2; 5,2,4</td>
# </tr><tr><td>11</td><td>1,6,4; 5,4,2; 3,2,6</td>
# </tr><tr><td>12</td><td>1,5,6; 2,6,4; 3,4,5</td>
# </tr><tr><td>12</td><td>1,6,5; 3,5,4; 2,4,6</td>
# </tr></table></div>
# <p>By concatenating each group it is possible to form 9-digit strings; the maxim
# um string for a 3-gon ring is 432621513.</p>
# <p>Using the numbers 1 to 10, and depending on arrangements, it is possible to f
# orm 16- and 17-digit strings. What is the maximum <b>16-digit</b> string for a "
# magic" 5-gon ring?</p>
# <div class="center">
# <img src="resources/images/0068_2.png?1678992052" class="dark_img" alt=""><br></
# div>

# Solution for Project Euler Problem 68

# The numbers 1 to 10 are used.
# A 16-digit string is required.
# Each of the 5 lines has 3 numbers: one outer node, two inner gon nodes.
# If all numbers were single-digit (1-9), the string length would be 5 * 3 = 15 digits.
# The number 10 has two digits.
# - If 10 is an outer node: One line contributes 4 digits (e.g., 10,g_i,g_j),
#   and the other four lines contribute 3 digits each. Total = 4 + 4*3 = 16 digits.
# - If 10 is an inner node: It will be part of two lines.
#   e.g., o_a, 10, g_b (4 digits) and o_c, g_d, 10 (4 digits).
#   The other three lines contribute 3 digits each. Total = 4 + 4 + 3*3 = 8 + 9 = 17 digits.
# Therefore, for a 16-digit string, the number 10 MUST be an outer node.

max_found_string = ""
numbers_to_permute = (1..10).to_a

# Iterate through all permutations of the numbers 1 to 10
numbers_to_permute.permutation.each do |p|
  # Assign numbers to nodes based on the permutation p:
  # p[0]..p[4] are outer nodes (o0, o1, o2, o3, o4)
  # p[5]..p[9] are inner gon nodes (g0, g1, g2, g3, g4)
  outer_nodes = [p[0], p[1], p[2], p[3], p[4]]
  inner_nodes = [p[5], p[6], p[7], p[8], p[9]]

  # Optimization: If 10 is not in an outer node, this permutation
  # cannot form a 16-digit string. Skip it.
  next unless outer_nodes.include?(10)

  # Define the lines of the 5-gon ring:
  # Line 0: outer_nodes[0], inner_nodes[0], inner_nodes[1]
  # Line 1: outer_nodes[1], inner_nodes[1], inner_nodes[2]
  # Line 2: outer_nodes[2], inner_nodes[2], inner_nodes[3]
  # Line 3: outer_nodes[3], inner_nodes[3], inner_nodes[4]
  # Line 4: outer_nodes[4], inner_nodes[4], inner_nodes[0] (circular)

  # Calculate the sum of the first line (this will be our target sum)
  target_sum = outer_nodes[0] + inner_nodes[0] + inner_nodes[1]

  # Check if all other lines have the same sum
  is_magic = true
  is_magic &&= (outer_nodes[1] + inner_nodes[1] + inner_nodes[2] == target_sum)
  is_magic &&= (outer_nodes[2] + inner_nodes[2] + inner_nodes[3] == target_sum)
  is_magic &&= (outer_nodes[3] + inner_nodes[3] + inner_nodes[4] == target_sum)
  is_magic &&= (outer_nodes[4] + inner_nodes[4] + inner_nodes[0] == target_sum)

  next unless is_magic

  # If it's a magic ring, prepare to construct the solution string
  line_triplets = [
    [outer_nodes[0], inner_nodes[0], inner_nodes[1]],
    [outer_nodes[1], inner_nodes[1], inner_nodes[2]],
    [outer_nodes[2], inner_nodes[2], inner_nodes[3]],
    [outer_nodes[3], inner_nodes[3], inner_nodes[4]],
    [outer_nodes[4], inner_nodes[4], inner_nodes[0]]
  ]

  # Find the starting line: the one with the numerically lowest external node value
  min_outer_node_value = outer_nodes[0]
  start_node_index = 0
  (1..4).each do |i|
    if outer_nodes[i] < min_outer_node_value
      min_outer_node_value = outer_nodes[i]
      start_node_index = i
    end
  end

  # Construct the candidate string by concatenating numbers from triplets,
  # starting from the determined start_node_index and proceeding clockwise.
  candidate_string = ""
  (0..4).each do |i|
    current_line_index = (start_node_index + i) % 5
    candidate_string += line_triplets[current_line_index].map(&:to_s).join
  end

  # We are looking for the maximum 16-digit string.
  # The check `outer_nodes.include?(10)` ensures that if a solution is found,
  # it *could* be 16-digits. This final check confirms.
  if candidate_string.length == 16
    if max_found_string == "" || candidate_string > max_found_string
      max_found_string = candidate_string
    end
  end
end

puts "Maximum 16-digit string for a magic 5-gon ring: #{max_found_string}"
