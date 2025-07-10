# Problem 180: Golden Triplets
require 'set' # For Set data structure
require 'rational' # For Rational numbers

K = 35

# 1. Generate set Q of unique reduced rationals a/b with 0 < a < b <= K
# These are x, y, z in the problem statement, belonging to Q_k.
# "a rational number x = a/b with 0 < a < b <= k and gcd(a,b) = 1"
rationals_q = Set.new
1.upto(K) do |b_val| # Denominator b can range from 1 to K
  1.upto(b_val - 1) do |a_val| # Numerator a must be < b
    # Rational(a,b) automatically reduces to simplest form, ensuring gcd(a,b)=1
    # and stores it uniquely in the set.
    # Condition 0 < a is met by 1.upto.
    # Condition b <= K is met by outer loop.
    # Condition a < b is met by inner loop.
    # Example: Rational(2,4) becomes Rational(1,2)
    r = Rational(a_val, b_val)
    # We need 0 < r < 1. Rational(a,b) with a < b guarantees r < 1.
    # If a_val is 0, Rational(0,b) is 0. We need a > 0.
    # The problem states 0 < a, so a_val starts from 1.
    # Denominators can be from 1 up to K.
    # If b_val = 1, then 1.upto(0) does nothing, so no rationals with den=1. Correct.
    # Smallest denominator for a/b < 1 is b=2 (e.g. 1/2).
    rationals_q.add(r)
  end
end
# The problem statement implies b starts from 2 for a/b to be < 1 and a > 0.
# If K=1, rationals_q is empty.
# If K=2, b_val=2, a_val=1 -> Rational(1,2). Correct.

# 2. Initialize distinct_sums Set to store unique s(x,y,z) values.
# These are of the form x+y+z.
distinct_sums = Set.new

# 3. Iterate through pairs (x,y) from Q
rationals_q.each do |x|
  rationals_q.each do |y|
    # For a golden triple (x,y,z) of order k, x,y,z must be in Q_k.
    # Q_k means denominator <= k and 0 < number < 1.

    # Case 1: x + y = z (implies s(x,y,z) = x+y+z = z+z = 2z)
    # Here, z is denoted z1 in the provided solution sketch.
    z1 = x + y
    # Check if z1 qualifies: 0 < z1.numerator < z1.denominator AND z1.denominator <= K
    # z1 > 0 is guaranteed as x,y > 0.
    # z1.numerator < z1.denominator ensures z1 < 1.
    if z1.denominator <= K && z1.numerator > 0 && z1.numerator < z1.denominator
      # The sum s(x,y,z1) = x+y+z1. Since x+y=z1, this is z1+z1 = 2*z1.
      s1 = 2 * z1
      distinct_sums.add(s1)
    end

    # Case 2: 1/x + 1/y = 1/z  (implies z = xy / (x+y))
    # Here, z is denoted z2 in the provided solution sketch.
    # s(x,y,z) = x+y+z
    z2 = (x * y) / (x + y) # This is the solution for z from 1/x + 1/y = 1/z
    # Check if z2 qualifies: 0 < z2.numerator < z2.denominator AND z2.denominator <= K
    # z2 > 0 is guaranteed.
    # z2 < min(x,y) so z2 < 1 is guaranteed as x,y < 1.
    # So z2.numerator < z2.denominator is guaranteed if z2.numerator > 0.
    if z2.denominator <= K && z2.numerator > 0 # Denominator can't be 0. Numerator > 0 for valid rational in Q_k.
      # For this case, s(x,y,z2) = x + y + z2.
      s2 = x + y + z2
      distinct_sums.add(s2)
    end
  end
end

# 4. Calculate total sum t = u/v by summing all unique s values
total_sum_t = Rational(0, 1) # Initialize sum as Rational(0,1)
distinct_sums.each do |s_val|
  total_sum_t += s_val
end

# 5. Output u+v from the final sum t = u/v
puts total_sum_t.numerator + total_sum_t.denominator
