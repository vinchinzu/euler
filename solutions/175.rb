# Euler Problem 175: Shortened Binary Expansion
# Find the SBE of the smallest n for which f(n)/f(n-1) = 123456789/987654321.
# f(0)=1, f(n) = number of ways to write n as sum of powers of 2 (power used at most twice).

class Euler175
  def solve
    p_val = 123456789
    q_val = 987654321

    # Step 1: Simplify the fraction P/Q
    # Integer#gcd is a built-in method in Ruby for integers.
    common_divisor = p_val.gcd(q_val)

    p_simple = p_val / common_divisor
    q_simple = q_val / common_divisor
    # After simplification, p_simple/q_simple = 1/8

    # Step 2: Determine the target ratio for f(n)/f(n-1)
    # The problem defines SBE(n) = (c_k, ...) where c_k is the count of leading 1s in binary n.
    # For n > 0, c_k must be >= 1.
    # The continued fraction for f(n)/f(n-1) is [c_k; c_{k-1}, ...].
    # If c_k >= 1, then the continued fraction value is >= 1.
    # So, f(n)/f(n-1) must be >= 1 for n > 0.
    # The problem asks for the "smallest n", implying such n exists and is likely positive.
    # If n=0, SBE(0)=(0,1), CF is [0;1]=1. f(0)/f(-1)=1. This is not 1/8.
    # Therefore, n must be > 0.
    # If the given (simplified) ratio p_simple/q_simple is < 1,
    # we must use its reciprocal q_simple/p_simple as the target for f(n)/f(n-1).

    target_p_num = p_simple
    target_q_den = q_simple

    if target_p_num < target_q_den # True for 1/8
      # Swap to make the ratio >= 1
      target_p_num, target_q_den = target_q_den, target_p_num
    end
    # Now target_p_num / target_q_den = 8/1 = 8.

    # Step 3: Compute the continued fraction of target_p_num / target_q_den.
    # This sequence of coefficients forms the SBE of n.
    sbe_terms = []

    num = target_p_num # Current numerator for CF algorithm
    den = target_q_den # Current denominator for CF algorithm

    while den > 0
      # Standard continued fraction algorithm:
      # current_fraction = num / den
      # term = floor(current_fraction)
      # next_fraction = 1 / (current_fraction - term)
      # which means next_fraction = den / (num % den)

      term = num / den # Integer part of num/den
      remainder = num % den

      sbe_terms << term # Add the integer part to SBE terms

      # Update num and den for the next iteration
      num = den
      den = remainder
    end

    # Step 4: Format the SBE terms as a comma-separated string without whitespace.
    sbe_terms.join(',')
  end
end

# Main execution block
if __FILE__ == $PROGRAM_NAME
  solver = Euler175.new
  result = solver.solve
  puts result # The script should only print the final answer
end
