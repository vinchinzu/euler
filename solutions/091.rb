#  <p>The points $P(x_1, y_1)$ and $Q(x_2, y_2)$ are plotted at integer co-ordinate
# s and are joined to the origin, $O(0,0)$, to form $\triangle OPQ$.</p>
# 
# <div class="center">
# <img src="resources/images/0091_1.png?1678992052" class="dark_img" alt=""><br></
# div>
# 
# <p>There are exactly fourteen triangles containing a right angle that can be for
# med when each co-ordinate lies between $0$ and $2$ inclusive; that is, $0 \le x_
# 1, y_1, x_2, y_2 \le 2$.</p>
# 
# <div class="center">
# <img src="resources/images/0091_2.png?1678992052" alt=""><br></div>
# 
# <p>Given that $0 \le x_1, y_1, x_2, y_2 \le 50$, how many right triangles can be
#  formed?</p>

# Solution for Project Euler Problem 91

GRID_MAX = 50
right_triangle_count = 0

# Iterate through all possible coordinates for point P(x1, y1)
(0..GRID_MAX).each do |x1|
  (0..GRID_MAX).each do |y1|
    # Iterate through all possible coordinates for point Q(x2, y2)
    (0..GRID_MAX).each do |x2|
      (0..GRID_MAX).each do |y2|
        
        # Validity Checks for P and Q:
        # 1. P must not be the origin O(0,0)
        next if x1 == 0 && y1 == 0
        # 2. Q must not be the origin O(0,0)
        next if x2 == 0 && y2 == 0
        # 3. P and Q must not be the same point
        next if x1 == x2 && y1 == y2

        # To count each triangle {O,P,Q} only once, we enforce an order on P and Q.
        # For example, process only if P is lexicographically smaller than Q.
        # If (x1,y1) is "greater than" (x2,y2), we would have already processed
        # this triangle when P was (x2,y2) and Q was (x1,y1).
        # This specific condition skips if P > Q, processing only P < Q.
        # (The problem example implies this ordering for distinct P, Q)
        # A simple way to do this is to iterate x2 from 0, but y2 from 0 if x2 > x1,
        # or y2 from y1+1 if x2 == x1.
        # However, the provided condition (x1 > x2) || (x1 == x2 && y1 > y2) effectively means
        # we only consider pairs where (x1,y1) < (x2,y2) in lexicographical order.
        # This means we are choosing Q such that it is "after" P.
        # This is a common way to iterate over unique pairs of points.
        if (x1 > x2) || (x1 == x2 && y1 > y2)
          next # Skip this pair to avoid double counting triangles (O,P,Q) and (O,Q,P)
        end

        # Calculate squared lengths of the sides of triangle OPQ
        # O is (0,0), P is (x1,y1), Q is (x2,y2)
        d_op_sq = x1*x1 + y1*y1       # Distance squared from O to P
        d_oq_sq = x2*x2 + y2*y2       # Distance squared from O to Q
        d_pq_sq = (x1-x2)**2 + (y1-y2)**2 # Distance squared from P to Q

        # These squared lengths must be non-zero because P, Q are distinct from O and each other.
        
        # Check for right angle using Pythagorean theorem
        # The sum of the squares of two sides must equal the square of the third side.
        # We need to check all three possibilities for the right angle (at O, P, or Q).
        
        is_right_triangle = false
        # Case 1: Right angle at O (OP is perpendicular to OQ)
        if d_op_sq + d_oq_sq == d_pq_sq
          is_right_triangle = true
        # Case 2: Right angle at P (OP is perpendicular to PQ)
        elsif d_op_sq + d_pq_sq == d_oq_sq
          is_right_triangle = true
        # Case 3: Right angle at Q (OQ is perpendicular to PQ)
        elsif d_oq_sq + d_pq_sq == d_op_sq
          is_right_triangle = true
        end

        if is_right_triangle
          right_triangle_count += 1
        end
      end
    end
  end
end

puts "Number of right triangles that can be formed: #{right_triangle_count}"
