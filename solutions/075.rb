#  <p>It turns out that $\pu{12 cm}$ is the smallest length of wire that can be ben
# t to form an integer sided right angle triangle in exactly one way, but there ar
# e many more examples.</p>
# <ul style="list-style-type:none;">
# <li>$\pu{\mathbf{12} \mathbf{cm}}$: $(3,4,5)$</li>
# <li>$\pu{\mathbf{24} \mathbf{cm}}$: $(6,8,10)$</li>
# <li>$\pu{\mathbf{30} \mathbf{cm}}$: $(5,12,13)$</li>
# <li>$\pu{\mathbf{36} \mathbf{cm}}$: $(9,12,15)$</li>
# <li>$\pu{\mathbf{40} \mathbf{cm}}$: $(8,15,17)$</li>
# <li>$\pu{\mathbf{48} \mathbf{cm}}$: $(12,16,20)$</li></ul>
# <p>In contrast, some lengths of wire, like $\pu{20 cm}$, cannot be bent to form
# an integer sided right angle triangle, and other lengths allow more than one sol
# ution to be found; for example, using $\pu{120 cm}$ it is possible to form exact
# ly three different integer sided right angle triangles.</p>
# <ul style="list-style-type:none;">
# <li>$\pu{\mathbf{120} \mathbf{cm}}$: $(30,40,50)$, $(20,48,52)$, $(24,45,51)$</l
# i></ul>
# 
# <p>Given that $L$ is the length of the wire, for how many values of $L \le 1\,50
# 0\,000$ can exactly one integer sided right angle triangle be formed?</p>

# Solution for Project Euler Problem 75

# We use Euclid's formula for Pythagorean triples:
# a = k * (m^2 - n^2)
# b = k * (2mn)
# c = k * (m^2 + n^2)
# where m > n > 0, m and n are coprime, and one of m, n is even (i.e., m-n is odd).
# The perimeter L = a + b + c = k * (m^2 - n^2 + 2mn + m^2 + n^2)
# L = k * (2m^2 + 2mn) = 2km(m+n).

LIMIT = 1_500_000

# perimeter_counts[l] will store how many ways a perimeter l can be formed.
perimeter_counts = Array.new(LIMIT + 1, 0)

# Determine the upper bound for m.
# Since L = 2km(m+n), and k >= 1, n >= 1:
# L >= 2m(m+1) > 2m^2.
# So, 2m^2 < LIMIT  => m^2 < LIMIT/2 => m < sqrt(LIMIT/2).
m_limit = Math.sqrt(LIMIT / 2.0).floor

# Loop for m
(2..m_limit).each do |m|
  # Loop for n
  (1...m).each do |n|
    # Conditions for (m,n) to form a primitive triple:
    # 1. m and n are coprime
    # 2. m and n have different parities (m-n is odd)
    next unless (m - n).odd? && m.gcd(n) == 1

    # Calculate the primitive perimeter (k=1)
    primitive_l = 2 * m * (m + n)

    # If primitive_l itself is too large, no need to check its multiples
    break if primitive_l > LIMIT 

    # Loop for k (multiples of the primitive triple)
    # current_l = k * primitive_l
    k = 1
    loop do
      current_l = k * primitive_l
      break if current_l > LIMIT
      
      perimeter_counts[current_l] += 1
      k += 1
    end
  end
end

# Count how many perimeters can be formed in exactly one way
count_singular = 0
perimeter_counts.each do |count|
  count_singular += 1 if count == 1
end
# Or more idiomatically: count_singular = perimeter_counts.count(1)

puts "Number of values of L <= #{LIMIT} for which exactly one integer sided right angle triangle can be formed: #{count_singular}"
