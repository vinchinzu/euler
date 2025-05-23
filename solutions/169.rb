# Sums of Powers of Two
# Problem 169

# <p>Define $f(0)=1$ and $f(n)$ to be the number of different ways $n$ can be expressed as a sum of integer powers of $2$ using each power no more than twice.</p>
# <p>For example, $f(10)=5$ since there are five different ways to express $10$:</p>
# \begin{align}
# &amp; 1 + 1 + 8\\
# &amp; 1 + 1 + 4 + 4\\
# &amp; 1 + 1 + 2 + 2 + 4\\
# &amp; 2 + 4 + 4\\
# &amp; 2 + 8
# \end{align}
# <p>What is $f(10^{25})$?</p>

class SumsOfPowersOfTwo
  def initialize
    @memo = {}
  end

  def count_ways(n)
    bits = []
    x = n
    while x > 0
      bits << x % 2
      x /= 2
    end
    dp(0, 0, bits)
  end

  private

  def dp(pos, carry, bits)
    return 1 if pos == bits.size && carry == 0
    return 0 if pos == bits.size
    key = [pos, carry]
    return @memo[key] if @memo.key?(key)
    total = 0
    3.times do |use|
      val = use + carry
      if val % 2 == bits[pos]
        total += dp(pos + 1, val / 2, bits)
      end
    end
    @memo[key] = total
    total
  end
end

if __FILE__ == $0
  n = 10**25
  solver = SumsOfPowersOfTwo.new
  puts solver.count_ways(n)
end

