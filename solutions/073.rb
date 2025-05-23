# Problem 73
#
#    Consider the fraction, $\dfrac n d$, where $n$ and $d$ are positive
#    integers. If $n \lt d$ and $\operatorname{HCF}(n, d)=1$, it is called a
#    reduced proper fraction.
#
#    If we list the set of reduced proper fractions for $d \le 8$ in
#    ascending order of size, we get: $$\frac 1 8, \frac 1 7, \frac 1 6,
#    \frac 1 5, \frac 1 4, \frac 2 7, \frac 1 3, \mathbf{\frac 3 8, \frac 2
#    5, \frac 3 7}, \frac 1 2, \frac 4 7, \frac 3 5, \frac 5 8, \frac 2 3,
#    \frac 5 7, \frac 3 4, \frac 4 5, \frac 5 6, \frac 6 7, \frac 7 8$$
#
#    It can be seen that there are $3$ fractions between $\dfrac 1 3$ and
#    $\dfrac 1 2$.
#
#    How many fractions lie between $\dfrac 1 3$ and $\dfrac 1 2$ in the
#    sorted set of reduced proper fractions for $d \le 12\,000$?

# Solution for Project Euler Problem 73

# We need to count reduced proper fractions n/d such that 1/3 < n/d < 1/2
# for d <= 12,000.
# A reduced proper fraction means HCF(n,d) = 1 and n < d.
# (The n < d condition is implicitly handled by n/d < 1/2).

LIMIT_D = 12_000
count = 0

# Iterate through all possible denominators d
(1..LIMIT_D).each do |d|
  # Determine the range for n:
  # 1/3 < n/d  =>  d < 3n  =>  n > d/3
  # n_min is the smallest integer n satisfying this.
  # Using integer division: n_min = (d / 3) + 1
  n_min = (d / 3) + 1

  # n/d < 1/2  =>  2n < d
  # n_max is the largest integer n satisfying this.
  # Using integer division: n_max = (d - 1) / 2
  # (If d is even, d-1 is odd, (d-1)/2 ensures 2n <= d-1 < d.
  #  If d is odd, d-1 is even, (d-1)/2 ensures 2n <= d-1 < d.)
  n_max = (d - 1) / 2

  # Iterate through possible numerators n in the calculated range
  if n_min <= n_max # Ensure the range is valid
    (n_min..n_max).each do |n|
      # Check if the fraction is reduced (HCF(n,d) == 1)
      if n.gcd(d) == 1
        count += 1
      end
    end
  end
end

puts "The number of fractions between 1/3 and 1/2 for d <= #{LIMIT_D} is: #{count}"
