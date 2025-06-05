# Helper to count even and odd numbers in a given range (inclusive)
def count_even_odd_in_range(range_start, range_end)
  even_count = 0
  odd_count = 0
  (range_start..range_end).each do |val|
    if val.even?
      even_count += 1
    else
      odd_count += 1
    end
  end
  [even_count, odd_count]
end

# Calculates the number of "diagonal" rectangles in a single m x n grid
def count_diagonal_rectangles_single_grid(m, n)
  # u = x+y ranges from 0 to m+n. Number of points is m+n+1.
  # x and y here are 0-indexed vertex coordinates.
  # m and n are cell counts (width and height).
  u_range_size = m + n + 1

  # Number of even/odd u-coordinates
  # If u_range_size is 5 (e.g., 0,1,2,3,4), u_even is 3 (0,2,4), u_odd is 2 (1,3)
  # (5+1)/2 = 3 for even; 5/2 = 2 for odd.
  u_even_count = (u_range_size + 1) / 2
  u_odd_count = u_range_size / 2

  # v = x-y ranges from -n to m. Number of points is m-(-n)+1 = m+n+1.
  v_even_count, v_odd_count = count_even_odd_in_range(-n, m)

  # Number of ways to choose 2 distinct coordinates C(N,2) = N*(N-1)/2
  rects_from_even_uv = 0
  if u_even_count >= 2 && v_even_count >= 2
    rects_from_even_uv = (u_even_count * (u_even_count - 1) / 2) *                          (v_even_count * (v_even_count - 1) / 2)
  end

  rects_from_odd_uv = 0
  if u_odd_count >= 2 && v_odd_count >= 2
    rects_from_odd_uv = (u_odd_count * (u_odd_count - 1) / 2) *                         (v_odd_count * (v_odd_count - 1) / 2)
  end

  rects_from_even_uv + rects_from_odd_uv
end

def problem_147(max_m, max_n)
  total_rectangles = 0

  # Iterate through all possible grid dimensions (width m, height n)
  (1..max_m).each do |m|
    (1..max_n).each do |n|
      # Count grid-aligned rectangles
      grid_aligned_rects = (m * (m + 1) / 2) * (n * (n + 1) / 2)
      total_rectangles += grid_aligned_rects

      # Count diagonal rectangles
      diagonal_rects = count_diagonal_rectangles_single_grid(m, n)
      total_rectangles += diagonal_rects
    end
  end

  total_rectangles
end
