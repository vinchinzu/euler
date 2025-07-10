# Project Euler Problem 163: Cross-hatched triangles

class TriangleGridCounter
  def initialize(n_value)
    @n = n_value
  end

  def count_triangles
    # Part A(n): Triangles with sides parallel to the main triangle's sides
    # Formula: n * (n + 2) * (2*n + 1) / 8
    a_n = @n * (@n + 2) * (2 * @n + 1) / 8

    # Part B(n): Tilted triangles, formed by median lines and their parallels
    # Formula depends on parity of n
    b_n = 0
    if @n.even?
      # Formula for even n: n * (n + 2) * (2*n - 1) / 8
      b_n = @n * (@n + 2) * (2 * @n - 1) / 8
    else # n is odd
      # Formula for odd n: (n*n - 1) * (2*n + 3) / 8
      b_n = (@n * @n - 1) * (2 * @n + 3) / 8
    end

    # Total count is the sum of these two parts
    a_n + b_n
  end
end

if __FILE__ == $PROGRAM_NAME
  # The problem asks for N = 36
  n_value = 36

  counter = TriangleGridCounter.new(n_value)
  result = counter.count_triangles

  # The script should print only the final answer
  puts result
end
```
