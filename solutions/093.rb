#  <p>By using each of the digits from the set, $\{1, 2, 3, 4\}$, exactly once, and
#  making use of the four arithmetic operations ($+, -, \times, /$) and brackets/p
# arentheses, it is possible to form different positive integer targets.</p>
# <p>For example,</p>
# \begin{align}
# 8 &amp;= (4 \times (1 + 3)) / 2\\
# 14 &amp;= 4 \times (3 + 1 / 2)\\
# 19 &amp;= 4 \times (2 + 3) - 1\\
# 36 &amp;= 3 \times 4 \times (2 + 1)
# \end{align}
# <p>Note that concatenations of the digits, like $12 + 34$, are not allowed.</p>
# <p>Using the set, $\{1, 2, 3, 4\}$, it is possible to obtain thirty-one differen
# t target numbers of which $36$ is the maximum, and each of the numbers $1$ to $2
# 8$ can be obtained before encountering the first non-expressible number.</p>
# <p>Find the set of four distinct digits, $a \lt b \lt c \lt d$, for which the lo
# ngest set of consecutive positive integers, $1$ to $n$, can be obtained, giving
# your answer as a string: <i>abcd</i>.</p>

require 'set'

# Helper function to apply an operation
def apply_op(a, op, b)
  a = a.to_f
  b = b.to_f

  return nil if op == :/ && b == 0.0 # Division by zero

  case op
  when :+
    a + b
  when :-
    a - b
  when :*
    a * b
  when :/
    a / b # Result can be a float
  end
end

max_n_found = 0
best_digits_string = ""
operators = [:+, :-, :*, :/]
epsilon = 1e-7 # For comparing float to integer, e.g. 23.999999999 -> 24

# Iterate through all combinations of 4 distinct digits from 0-9
(0..9).to_a.combination(4).each do |digits| # digits is sorted, e.g., [1, 2, 3, 4]
  
  achieved_targets = Set.new

  # Iterate through all permutations of these 4 digits
  digits.permutation.each do |p| # p is a specific ordering, e.g., [4, 1, 3, 2]
    
    # Iterate through all combinations of 3 operators (with repetition)
    # ops = [op_for_first_eval, op_for_second_eval, op_for_third_eval]
    # The meaning of ops[0], ops[1], ops[2] depends on the parenthesization scheme.
    operators.repeated_permutation(3).each do |ops_list|
      
      # Parenthesization Scheme 1: ((p[0] ops_list[0] p[1]) ops_list[1] p[2]) ops_list[2] p[3]
      val_s1_1 = apply_op(p[0], ops_list[0], p[1])
      if !val_s1_1.nil?
        val_s1_2 = apply_op(val_s1_1, ops_list[1], p[2])
        if !val_s1_2.nil?
          final_res_s1 = apply_op(val_s1_2, ops_list[2], p[3])
          if !final_res_s1.nil? && final_res_s1.finite? && final_res_s1 > 0 && (final_res_s1 - final_res_s1.round).abs < epsilon
            achieved_targets.add(final_res_s1.round)
          end
        end
      end

      # Parenthesization Scheme 2: (p[0] ops_list[0] (p[1] ops_list[1] p[2])) ops_list[2] p[3]
      val_s2_1 = apply_op(p[1], ops_list[1], p[2]) # Inner: p[1] op1 p[2]
      if !val_s2_1.nil?
        val_s2_2 = apply_op(p[0], ops_list[0], val_s2_1) # Mid: p[0] op0 (result of inner)
        if !val_s2_2.nil?
          final_res_s2 = apply_op(val_s2_2, ops_list[2], p[3]) # Outer: (result of mid) op2 p[3]
          if !final_res_s2.nil? && final_res_s2.finite? && final_res_s2 > 0 && (final_res_s2 - final_res_s2.round).abs < epsilon
            achieved_targets.add(final_res_s2.round)
          end
        end
      end
      
      # Parenthesization Scheme 3: p[0] ops_list[0] ((p[1] ops_list[1] p[2]) ops_list[2] p[3])
      val_s3_1 = apply_op(p[1], ops_list[1], p[2]) # Inner: p[1] op1 p[2]
      if !val_s3_1.nil?
        val_s3_2 = apply_op(val_s3_1, ops_list[2], p[3]) # Mid: (result of inner) op2 p[3]
        if !val_s3_2.nil?
          final_res_s3 = apply_op(p[0], ops_list[0], val_s3_2) # Outer: p[0] op0 (result of mid)
          if !final_res_s3.nil? && final_res_s3.finite? && final_res_s3 > 0 && (final_res_s3 - final_res_s3.round).abs < epsilon
            achieved_targets.add(final_res_s3.round)
          end
        end
      end

      # Parenthesization Scheme 4: p[0] ops_list[0] (p[1] ops_list[1] (p[2] ops_list[2] p[3]))
      val_s4_1 = apply_op(p[2], ops_list[2], p[3]) # Innermost: p[2] op2 p[3]
      if !val_s4_1.nil?
        val_s4_2 = apply_op(p[1], ops_list[1], val_s4_1) # Mid: p[1] op1 (result of innermost)
        if !val_s4_2.nil?
          final_res_s4 = apply_op(p[0], ops_list[0], val_s4_2) # Outer: p[0] op0 (result of mid)
          if !final_res_s4.nil? && final_res_s4.finite? && final_res_s4 > 0 && (final_res_s4 - final_res_s4.round).abs < epsilon
            achieved_targets.add(final_res_s4.round)
          end
        end
      end

      # Parenthesization Scheme 5: (p[0] ops_list[0] p[1]) ops_list[1] (p[2] ops_list[2] p[3])
      val_s5_1 = apply_op(p[0], ops_list[0], p[1]) # Left pair: p[0] op0 p[1]
      if !val_s5_1.nil?
        val_s5_2 = apply_op(p[2], ops_list[2], p[3]) # Right pair: p[2] op2 p[3]
        if !val_s5_2.nil?
          final_res_s5 = apply_op(val_s5_1, ops_list[1], val_s5_2) # Combine: (left_res) op1 (right_res)
          if !final_res_s5.nil? && final_res_s5.finite? && final_res_s5 > 0 && (final_res_s5 - final_res_s5.round).abs < epsilon
            achieved_targets.add(final_res_s5.round)
          end
        end
      end
    end
  end

  # Check for the longest consecutive sequence of positive integers (1 to n)
  current_n = 0
  while achieved_targets.include?(current_n + 1)
    current_n += 1
  end

  if current_n > max_n_found
    max_n_found = current_n
    best_digits_string = digits.join # digits is already sorted, e.g. [1,2,5,8] -> "1258"
  end
end

puts best_digits_string