# solutions/184.rb

# Problem 184: Triangles Containing the Origin
# r = 105

def combinations(n, k)
  return 0 if k < 0 || k > n
  return 1 if k == 0 || k == n
  # Reduce k to simpler case by symmetry C(n,k) = C(n, n-k)
  k = n - k if k > n / 2

  # Calculate C(n,k)
  res = 1
  (1..k).each do |i|
    # Ensure division is exact by ordering terms carefully if n is large,
    # but for C(n,2) and C(n,3) direct formula is fine.
    # Ruby handles large integers, so direct computation is okay.
    res = res * (n - i + 1) / i
  end
  res
end

Point = Struct.new(:x, :y, :angle)

radius = 105
r_squared = radius * radius
raw_points = []

# Step 1: Generate points
(-radius + 1...radius).each do |x|
  (-radius + 1...radius).each do |y|
    next if x == 0 && y == 0 # Exclude origin
    if x * x + y * y < r_squared
      raw_points << [x, y]
    end
  end
end

# Step 2: Create Point objects and sort by angle
points = raw_points.map { |p| Point.new(p[0], p[1], Math.atan2(p[1], p[0])) }
points.sort_by!(&:angle)

n_total = points.length
n_bad_sum = 0 # Use standard integer, Ruby handles large numbers

# Step 3 & 4: Initialize sum and j_ptr for sweep
j_ptr = 1 # This pointer will sweep CCW around the circle

# Step 5: Iterate through each point pi
(0...n_total).each do |i|
  pi = points[i]

  # 5.b: Ensure j_ptr is ahead of i (circularly).
  # This condition means j_ptr must have completed a full circle if it catches up to i this way.
  # Or, more simply, j_ptr should be at least (i+1)%n_total.
  # If i "overtakes" j_ptr due to circular array logic, reset j_ptr.
  # This check is subtle. If points[j_ptr] == pi, it means j_ptr has wrapped AND caught up.
  # A robust way: check if angle of points[j_ptr] is "less than or equal" to angle of pi considering wrap.
  # Simpler: if pi is points[0], j_ptr starts at 1. If pi is points[1], j_ptr continues from where it was.
  # The crucial part is that j_ptr only advances.
  # If j_ptr == i due to wrap around, advance it.
  if j_ptr == i
    j_ptr = (j_ptr + 1) % n_total
  end

  # 5.c: Advance j_ptr while points[j_ptr] is strictly in CCW half-plane of pi
  # The loop must also ensure j_ptr doesn't cross pi again from the "wrong" side.
  # points[j_ptr] != pi handles the case where all points are in one half-plane from pi.
  while points[j_ptr] != pi && (pi.x * points[j_ptr].y - pi.y * points[j_ptr].x) > 0
    j_ptr = (j_ptr + 1) % n_total
  end
  # points[j_ptr] is now the first point that is NOT strictly CCW of pi.

  # 5.d: Calculate n_half_pi
  n_half_pi = (j_ptr - i - 1 + n_total) % n_total

  # 5.e: Count n_line_pi
  n_line_pi = 0
  temp_ptr = j_ptr # Start checking from where j_ptr stopped
  # Loop while points[temp_ptr] is collinear with pi AND not pi itself
  while points[temp_ptr] != pi && (pi.x * points[temp_ptr].y - pi.y * points[temp_ptr].x) == 0
    # Check if it's on the opposite side of origin
    if (pi.x * points[temp_ptr].x + pi.y * points[temp_ptr].y) < 0
      n_line_pi += 1
    end
    temp_ptr = (temp_ptr + 1) % n_total
  end

  # 5.f: Add to n_bad_sum
  n_bad_sum += combinations(n_half_pi, 2)
  n_bad_sum += n_half_pi * n_line_pi
end

# Step 6 & 7: Calculate final result
total_triangles = combinations(n_total, 3)
result = total_triangles - n_bad_sum

# Step 8: Print result
puts result
