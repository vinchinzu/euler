#  <p>A spider, S, sits in one corner of a cuboid room, measuring $6$ by $5$ by $3$
# , and a fly, F, sits in the opposite corner. By travelling on the surfaces of th
# e room the shortest "straight line" distance from S to F is $10$ and the path is
#  shown on the diagram.</p>
# <div class="center">
# <img src="resources/images/0086.png?1678992052" class="dark_img" alt=""><br></di
# v>
# <p>However, there are up to three "shortest" path candidates for any given cuboi
# d and the shortest route doesn't always have integer length.</p>
# <p>It can be shown that there are exactly $2060$ distinct cuboids, ignoring rota
# tions, with integer dimensions, up to a maximum size of $M$ by $M$ by $M$, for w
# hich the shortest route has integer length when $M = 100$. This is the least val
# ue of $M$ for which the number of solutions first exceeds two thousand; the numb
# er of solutions when $M = 99$ is $1975$.</p>
# <p>Find the least value of $M$ such that the number of solutions first exceeds o
# ne million.</p>

# Solution for Project Euler Problem 86

# For a cuboid with dimensions L, W, H, where we can assume 1 <= H <= W <= L without loss of generality for counting distinct cuboids.
# The shortest path on the surface is given by unfolding the cuboid.
# The candidates for the squared shortest path are (L+W)^2 + H^2, (L+H)^2 + W^2, (W+H)^2 + L^2.
# The smallest of these will be when the two smaller dimensions are summed, i.e., (W+H).
# So, shortest_path_squared = L^2 + (W+H)^2.
# We are looking for cases where sqrt(L^2 + (W+H)^2) is an integer.

TARGET_SOLUTIONS = 1_000_000
solutions_found_count = 0
m_val = 0 # This will be our M, the maximum dimension allowed for L, W, H.

loop do
  m_val += 1
  l_dim = m_val # L is the current M being tested. Cuboids are L x W x H where L=m_val.

  # Iterate through possible sums of W+H.
  # Let sum_wh = W+H.
  # Since 1 <= H <= W <= L_dim,
  # Min sum_wh = 1+1 = 2.
  # Max sum_wh = L_dim + L_dim = 2 * L_dim.
  (2..(2 * l_dim)).each do |sum_wh|
    path_length_squared = l_dim * l_dim + sum_wh * sum_wh
    path_length = Math.sqrt(path_length_squared)

    if path_length == path_length.floor # Integer path
      # Now we need to count valid pairs (W,H) that sum to sum_wh,
      # subject to 1 <= H <= W and W <= L_dim.

      # From H <= W and H+W = sum_wh:
      # H <= sum_wh - H  =>  2H <= sum_wh  =>  H <= sum_wh / 2.
      # So, upper_bound_for_h = sum_wh / 2 (integer division).
      upper_bound_for_h = sum_wh / 2

      # From W <= L_dim and H+W = sum_wh:
      # sum_wh - H <= L_dim  =>  H >= sum_wh - L_dim.
      # Also, H must be at least 1.
      # So, lower_bound_for_h = max(1, sum_wh - L_dim).
      # This can be written as: if sum_wh > L_dim, then H starts from sum_wh - L_dim.
      # Otherwise (if sum_wh <= L_dim), H can start from 1 (as W = sum_wh - H will be <= L_dim - 1 < L_dim).
      lower_bound_for_h = (sum_wh > l_dim) ? (sum_wh - l_dim) : 1
      
      if lower_bound_for_h <= upper_bound_for_h
        num_valid_pairs = upper_bound_for_h - lower_bound_for_h + 1
        solutions_found_count += num_valid_pairs
      end
    end
  end

  if solutions_found_count > TARGET_SOLUTIONS
    puts "The least value of M for which solutions exceed #{TARGET_SOLUTIONS} is: #{m_val}"
    break
  end
end
