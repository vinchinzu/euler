# <p>Consider quadratic Diophantine equations of the form:
# $$x^2 - Dy^2 = 1$$</p>
# <p>For example, when $D=13$, the minimal solution in $x$ is $649^2 - 13 \times 1
# 80^2 = 1$.</p>
# <p>It can be assumed that there are no solutions in positive integers when $D$ i
# s square.</p>
# <p>By finding minimal solutions in $x$ for $D = \{2, 3, 5, 6, 7\}$, we obtain th
# e following:</p>
# \begin{align}
# 3^2 - 2 \times 2^2 &amp;= 1\\
# 2^2 - 3 \times 1^2 &amp;= 1\\
# {\color{red}{\mathbf 9}}^2 - 5 \times 4^2 &amp;= 1\\
# 5^2 - 6 \times 2^2 &amp;= 1\\
# 8^2 - 7 \times 3^2 &amp;= 1
# \end{align}
# <p>Hence, by considering minimal solutions in $x$ for $D \le 7$, the largest $x$
#  is obtained when $D=5$.</p>
# <p>Find the value of $D \le 1000$ in minimal solutions of $x$ for which the larg
# est value of $x$ is obtained.</p>

# Solution for Project Euler Problem 66

max_x = 0
result_d = 0
limit = 1000

(2..limit).each do |d|
  # Skip if D is a perfect square
  sqrt_d_int = Math.sqrt(d).to_i
  next if sqrt_d_int * sqrt_d_int == d

  # Continued fraction algorithm for sqrt(D)
  # and calculation of convergents p_k/q_k to find the minimal x
  # such that x^2 - D*y^2 = 1.

  m_i = 0
  d_i = 1
  a_0 = Math.sqrt(d).to_i
  a_i = a_0 # Current a_i term

  # Initial values for convergents p_k, q_k
  # p_km2 is p_{k-2}, p_km1 is p_{k-1}
  # q_km2 is q_{k-2}, q_km1 is q_{k-1}
  p_km2 = 0
  p_km1 = 1
  q_km2 = 1
  q_km1 = 0

  # Loop to find convergents until x^2 - D*y^2 = 1
  loop do
    # Calculate current convergent (p_k, q_k) using a_i
    # p_k = a_i * p_{k-1} + p_{k-2}
    # q_k = a_i * q_{k-1} + q_{k-2}
    current_p = a_i * p_km1 + p_km2
    current_q = a_i * q_km1 + q_km2

    # Check if this convergent is the solution to Pell's equation
    # current_p will be x, current_q will be y
    if current_p * current_p - d * current_q * current_q == 1
      if current_p > max_x
        max_x = current_p
        result_d = d
      end
      break # Found minimal solution for this D
    end

    # Update p's and q's for next iteration
    p_km2 = p_km1
    p_km1 = current_p
    q_km2 = q_km1
    q_km1 = current_q

    # Calculate next a_i for the continued fraction
    m_next = d_i * a_i - m_i
    d_next = (d - m_next * m_next) / d_i
    a_next = (a_0 + m_next).div(d_next)

    # Update m_i, d_i, a_i for next iteration
    m_i = m_next
    d_i = d_next
    a_i = a_next
  end
end

puts "The value of D <= #{limit} for which the largest x is obtained is: #{result_d}"
#puts "This largest x is: #{max_x}" # For verification, not required by problem
