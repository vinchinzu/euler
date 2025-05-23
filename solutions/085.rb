#  <p>By counting carefully it can be seen that a rectangular grid measuring $3$ by
#  $2$ contains eighteen rectangles:</p>
# <div class="center">
# <img src="resources/images/0085.png?1678992052" class="dark_img" alt=""></div>
# <p>Although there exists no rectangular grid that contains exactly two million r
# ectangles, find the area of the grid with the nearest solution.</p>

# Solution for Project Euler Problem 85

# The number of rectangles in an m x n grid is given by the formula:
# Count(m,n) = (m * (m+1) / 2) * (n * (n+1) / 2)
# Let T(x) = x * (x+1) / 2. Then Count(m,n) = T(m) * T(n).
# We need to find m and n such that Count(m,n) is closest to 2,000,000.
# The area of this grid is m * n.

TARGET_RECTS = 2_000_000
min_difference_found = Float::INFINITY
area_for_min_difference = 0

# Determine a search limit for m (and n, since we'll use n <= m).
# If n=1, T(m) * T(1) = T(m) * 1 should be around TARGET_RECTS.
# m*(m+1)/2 approx TARGET_RECTS => m^2 approx 2 * TARGET_RECTS
# m approx sqrt(2 * 2_000_000) = sqrt(4_000_000) = 2000.
# So, m (and n) will not exceed 2000 by much.
M_SEARCH_LIMIT = 2000 # Max value for m

(1..M_SEARCH_LIMIT).each do |m|
  m_term = m * (m + 1) / 2

  # Optimization for outer m loop:
  # If T(m)*T(1) (i.e., m_term * 1, for the smallest n=1) is already
  # producing a number of rectangles that is further from TARGET_RECTS
  # than the best solution found so far, and we are on the "overshot" side,
  # then increasing m further will only result in even larger rectangle counts
  # and thus larger differences.
  # A simpler check: if m_term itself is already > TARGET_RECTS, then
  # m_term * n_term will be even larger.
  if m_term > TARGET_RECTS && m_term - TARGET_RECTS > min_difference_found && min_difference_found != Float::INFINITY
     # This condition means that for n=1, we are already overshooting TARGET_RECTS,
     # and the difference is already worse than the best difference we've found.
     # Any further increase in 'm' will make m_term larger, pushing results further away.
     break
  end

  (1..m).each do |n| # Loop n up to m to cover unique grid geometries (m x n)
    n_term = n * (n + 1) / 2
    current_num_rectangles = m_term * n_term
    current_difference = (current_num_rectangles - TARGET_RECTS).abs

    if current_difference < min_difference_found
      min_difference_found = current_difference
      area_for_min_difference = m * n
    elsif current_difference == min_difference_found
      # The problem asks for "the area", implying one such area or any if tied.
      # If multiple areas give the same minimal difference, we could, for example,
      # choose the one with the largest area or smallest area.
      # For now, sticking to the first one found by strictly smaller difference.
      # Or, if criteria was e.g. largest area for same diff:
      # if m * n > area_for_min_difference
      #   area_for_min_difference = m * n
      # end
    end

    # Optimization for inner n loop:
    # If current_num_rectangles has overshot TARGET_RECTS, then increasing n
    # (for the current m) will only result in a larger current_num_rectangles,
    # and thus a potentially larger difference. So, we can break.
    if current_num_rectangles > TARGET_RECTS
      # We also need to consider if this overshot value is better than previous.
      # The update for min_difference_found already handled this.
      # If we overshot, the next n will only be further overshot.
      break
    end
  end
end

puts "The area of the grid with the nearest number of rectangles to #{TARGET_RECTS} is: #{area_for_min_difference}"
puts "This grid has a difference of #{min_difference_found} from the target." # For verification.
