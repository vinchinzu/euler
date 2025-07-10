require 'set'

class TrueIntersectionsCounter
  # Point struct can hold Integer coordinates (for segment endpoints)
  # or Rational coordinates (for intersection points).
  # Struct provides value-based equality and hashing needed for Set storage.
  Point = Struct.new(:x, :y)
  Segment = Struct.new(:p1, :p2)

  def initialize(num_segments)
    @num_segments = num_segments
    @segments = []
    @intersection_points_set = Set.new
  end

  def generate_segments_and_coords
    s = 290797
    mod_s = 50515093
    mod_t = 500

    coords_t = Array.new(4 * @num_segments)

    # Generate t_1, t_2, ..., t_{4*num_segments}
    # s_0 is the seed. s_1 = s_0^2 mod M. t_1 = s_1 mod T.
    # So the loop should start with s, then update s to s^2 for next term.
    current_s = s # This is s_0
    (0...(4 * @num_segments)).each do |i|
      current_s = (current_s * current_s) % mod_s # This is s_{i+1}
      coords_t[i] = current_s % mod_t             # This is t_{i+1}
    end

    (0...@num_segments).each do |i|
      idx = i * 4
      # Segment endpoints are Points with Integer coordinates
      # L_k uses t_{4k-3}, t_{4k-2}, t_{4k-1}, t_{4k}
      # For L_1 (i=0), uses t_1, t_2, t_3, t_4. Indices are coords_t[0]..coords_t[3]
      p1 = Point.new(coords_t[idx + 0], coords_t[idx + 1])
      p2 = Point.new(coords_t[idx + 2], coords_t[idx + 3])
      @segments << Segment.new(p1, p2)
    end
  end

  def count_distinct_true_intersections
    generate_segments_and_coords

    (0...@num_segments).each do |i|
      ((i + 1)...@num_segments).each do |j|
        s1 = @segments[i]
        s2 = @segments[j]

        intersect_pt = calculate_true_intersection(s1, s2)
        @intersection_points_set.add(intersect_pt) if intersect_pt
      end
    end
    @intersection_points_set.size
  end

  private

  def calculate_true_intersection(s1, s2)
    p1 = s1.p1; p2 = s1.p2 # Points with Integer x, y
    p3 = s2.p1; p4 = s2.p2

    x1, y1 = p1.x, p1.y
    x2, y2 = p2.x, p2.y
    x3, y3 = p3.x, p3.y
    x4, y4 = p4.x, p4.y

    # Calculate components of vectors r = P2-P1 and s = P4-P3 (all Integers)
    rx = x2 - x1; ry = y2 - y1
    sx = x4 - x3; sy = y4 - y3

    # Denominator for parameters t and u
    den = rx * sy - ry * sx # Integer
    return nil if den == 0 # Lines are parallel or collinear

    # Numerator for t parameter (P = P1 + t*r)
    # t = ((x3 - x1) * sy - (y3 - y1) * sx) / den
    t_num = (x3 - x1) * sy - (y3 - y1) * sx # Integer

    # Numerator for u parameter (P = P3 + u*s)
    # u = ((x3 - x1) * ry - (y3 - y1) * rx) / den
    u_num = (x3 - x1) * ry - (y3 - y1) * rx # Integer

    # Check for true intersection: 0 < t < 1 and 0 < u < 1
    # This means t_num and den must have same sign, and |t_num| < |den|. Same for u.
    valid_t = false
    if den > 0
      valid_t = (t_num > 0 && t_num < den)
    else # den < 0
      valid_t = (t_num < 0 && t_num > den) # e.g. t_num=-1, den=-2 (t=0.5). (-1<0 && -1>-2) is true.
    end
    return nil unless valid_t

    valid_u = false
    if den > 0
      valid_u = (u_num > 0 && u_num < den)
    else # den < 0
      valid_u = (u_num < 0 && u_num > den)
    end
    return nil unless valid_u

    # Calculate intersection point coordinates. These will be Rational.
    # Px = x1 + t_num/den * rx = (x1*den + t_num*rx) / den
    # Py = y1 + t_num/den * ry = (y1*den + t_num*ry) / den
    # All calculations on RHS are integer arithmetic. Final result is Rational.
    intersect_x = Rational(x1 * den + t_num * rx, den)
    intersect_y = Rational(y1 * den + t_num * ry, den)

    # Return a Point with Rational coordinates
    Point.new(intersect_x, intersect_y)
  end
end

if __FILE__ == $PROGRAM_NAME
  counter = TrueIntersectionsCounter.new(5000)
  result = counter.count_distinct_true_intersections
  # The script should print only the final answer
  puts result
end
```
