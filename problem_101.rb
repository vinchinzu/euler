# Function to generate terms of the sequence u_n
# u_n = 1 - n + n^2 - n^3 + n^4 - n^5 + n^6 - n^7 + n^8 - n^9 + n^10
def u(n)
  sum = 0
  (0..10).each do |i|
    sum += ((-n)**i)
  end
  sum
end

# For testing purposes, let's print the first few terms
# (1..5).each do |i|
#   puts "u(#{i}) = #{u(i)}"
# end
#
# The problem statement implies u_n starts from n=1 for OP generation.
# u_1 = 1 - 1 + 1 - 1 + 1 - 1 + 1 - 1 + 1 - 1 + 1 = 1
# u_2 = 1 - 2 + 4 - 8 + 16 - 32 + 64 - 128 + 256 - 512 + 1024 = 683
# u_3 = 1 - 3 + 9 - 27 + 81 - 243 + 729 - 2187 + 6561 - 19683 + 59049 = 44287
# Let's verify u(1)
# (0..10).each { |i| p ((-1)**i) } => 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1. Sum = 1. Correct.
# Let's verify u(n) in the problem statement:
# u_n = 1 - n + n^2 - n^3 + n^4 - n^5 + n^6 - n^7 + n^8 - n^9 + n^10
# For n=1: 1-1+1-1+1-1+1-1+1-1+1 = 1
# For n=2: 1-2+4-8+16-32+64-128+256-512+1024 = 683
# For n=3: 1-3+9-27+81-243+729-2187+6561-19683+59049 = 44287
# The u(n) function seems correct.

# Function to calculate OP(k, n) using Lagrange Interpolation
# k_val: number of terms used to generate OP (e.g., k_val=1 means OP is degree 0, using u_1)
# n_val: the value at which to evaluate the polynomial OP(k, n_val)
# sequence_terms: an array containing the first k_val terms of the sequence [u_1, u_2, ..., u_k_val]
def op(k_val, n_val, sequence_terms)
  # Ensure we have enough terms in the sequence_terms
  raise "Not enough sequence terms provided for k_val=#{k_val}" if sequence_terms.length < k_val

  # The known points are (1, u_1), (2, u_2), ..., (k_val, u_k_val)
  # We want to evaluate the polynomial at n_val.

  sum = 0.0

  (0...k_val).each do |j| # j from 0 to k_val-1
    y_j = sequence_terms[j]
    x_j = j + 1 # x_j goes from 1 to k_val

    numerator = 1.0
    denominator = 1.0

    (0...k_val).each do |i| # i from 0 to k_val-1
      next if i == j
      x_i = i + 1 # x_i goes from 1 to k_val

      numerator *= (n_val - x_i)
      denominator *= (x_j - x_i)
    end

    # It's possible for denominator to be zero if x_j values are not distinct,
    # but in our case, x_j are 1, 2, ..., k_val, so they are distinct.
    raise "Denominator is zero in Lagrange basis polynomial calculation" if denominator == 0.0

    sum += y_j * (numerator / denominator)
  end

  # The problem expects integer FITs.
  # Lagrange polynomials with integer points (x_i, y_i) evaluated at an integer x
  # should result in a rational number. If the problem implies integer FITs,
  # we should be careful about precision.
  # Let's round to nearest integer, assuming FITs are integers.
  # Project Euler problems usually have integer answers.
  sum.round
end

# Example tests from the problem description (for a cubic: n^3)
# u_n = n^3.
# u1=1, u2=8, u3=27, u4=64.
# OP(1,n) = u1 = 1. FIT for k=1 is OP(1,2)=1. But u2=8. So FIT = 1.
# OP(2,n) uses (1,1) and (2,8).
#   y1=1, x1=1. y2=8, x2=2. Evaluate at n=3.
#   l1(3) = (3-x2)/(x1-x2) = (3-2)/(1-2) = 1/-1 = -1.
#   l2(3) = (3-x1)/(x2-x1) = (3-1)/(2-1) = 2/1 = 2.
#   OP(2,3) = y1*l1(3) + y2*l2(3) = 1*(-1) + 8*(2) = -1 + 16 = 15.
#   u3=27. FIT for k=2 is OP(2,3)=15.
# OP(3,n) uses (1,1), (2,8), (3,27). Evaluate at n=4.
#   y1=1,x1=1; y2=8,x2=2; y3=27,x3=3. Evaluate at n=4.
#   l1(4) = ((4-x2)(4-x3)) / ((x1-x2)(x1-x3)) = ((4-2)(4-3)) / ((1-2)(1-3)) = (2*1)/((-1)*(-2)) = 2/2 = 1.
#   l2(4) = ((4-x1)(4-x3)) / ((x2-x1)(x2-x3)) = ((4-1)(4-3)) / ((2-1)(2-3)) = (3*1)/(1*(-1)) = 3/-1 = -3.
#   l3(4) = ((4-x1)(4-x2)) / ((x3-x1)(x3-x2)) = ((4-1)(4-2)) / ((3-1)(3-2)) = (3*2)/(2*1) = 6/2 = 3.
#   OP(3,4) = y1*l1(4) + y2*l2(4) + y3*l3(4) = 1*(1) + 8*(-3) + 27*(3) = 1 - 24 + 81 = 58.
#   u4=64. FIT for k=3 is OP(3,4)=58.
# Sum of FITs for n^3: 1 + 15 + 58 = 74.

# The op function seems plausible.

# Main logic to calculate sum of FITs
sum_of_fits = 0
sequence_u_values = [] # Stores u_1, u_2, ...

# The generating polynomial is degree 10.
# We expect OP(k,n) to be a BOP for k = 1, 2, ..., 10.
# For k=11, OP(11,n) should be the original polynomial, and OP(11, 12) == u_12.
(1..10).each do |k|
  # Generate sequence terms up to u_k
  # The op function needs the first k terms: u_1, ..., u_k.
  # If sequence_u_values is [u_1, ..., u_{k-1}], we need to add u_k.
  # The u(n) function calculates u_n, so for u_k, n=k.
  if sequence_u_values.length < k
    sequence_u_values << u(k) # Add u_k to the list
  end
  
  # These are the terms [u_1, ..., u_k]
  current_sequence_terms = sequence_u_values

  # Calculate the predicted next term: OP(k, k+1)
  # This is the First Incorrect Term (FIT) if OP(k,n) is a BOP
  predicted_next_term = op(k, k + 1, current_sequence_terms)

  # Calculate the actual next term in the sequence: u_{k+1}
  actual_next_term = u(k + 1)

  # Check if OP(k,n) is a BOP
  if predicted_next_term != actual_next_term
    # puts "k=#{k}: BOP found."
    # puts "  Sequence terms used: #{current_sequence_terms.inspect}"
    # puts "  OP(#{k}, #{k+1}) = #{predicted_next_term}"
    # puts "  u(#{k+1})     = #{actual_next_term}"
    sum_of_fits += predicted_next_term
  else
    # This case should not happen for k <= 10 for a degree 10 polynomial
    # as OP(k,n) would be the true polynomial.
    # puts "k=#{k}: OP(#{k},n) is NOT a BOP (or we reached the polynomial's degree)."
    # puts "  OP(#{k}, #{k+1}) = #{predicted_next_term}"
    # puts "  u(#{k+1})     = #{actual_next_term}"
    # This would mean OP(k,n) perfectly predicts u_{k+1}.
    # For a polynomial of degree D, OP(D+1, n) is the polynomial itself.
    # Our polynomial is degree 10. So OP(11,n) is the polynomial.
    # Thus, for k=1 to 10, OP(k,n) must be a BOP.
  end
end

puts "Sum of FITs: #{sum_of_fits}"
