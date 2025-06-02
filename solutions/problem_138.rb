# Problem 138: Isosceles triangles
#
# We are looking for isosceles triangles with base b, side L, such that h = b +/- 1.
# The height h can be found using Pythagoras: h^2 + (b/2)^2 = L^2.
# So, 4h^2 + b^2 = 4L^2.
#
# Case 1: h = b - 1
# 4(b-1)^2 + b^2 = 4L^2
# 4(b^2 - 2b + 1) + b^2 = 4L^2
# 4b^2 - 8b + 4 + b^2 = 4L^2
# 5b^2 - 8b + 4 = 4L^2
# This can be rewritten as (5b-4)^2 - 5(2L)^2 = -4. (Pell-like equation)
# Let X = 5b-4, Y = 2L. Then X^2 - 5Y^2 = -4.
# Solutions (X,Y) are (L_k, F_k) where L_k is k-th Lucas, F_k is k-th Fibonacci.
# We need k to be odd for X^2 - 5Y^2 = -4. So k = 2n+1.
# X = 5b-4 = L_{2n+1}
# Y = 2L   = F_{2n+1}
# From 5b-4 = L_{2n+1}, we need L_{2n+1} === -4 === 1 (mod 5).
# Lucas numbers mod 5 are (2,1,3,4, 2,1,3,4,...). L_k === 1 (mod 5) if k === 1 (mod 4).
# So 2n+1 must be of the form 4j+1.
# Smallest is 2n+1 = 1 (j=0) => L_1=1. 5b-4=1 => 5b=5 => b=1. Y=F_1=1. 2L=1 (L not integer).
# Next 2n+1 = 5 (j=1) => L_5=11. 5b-4=11 => 5b=15 => b=3. Y=F_5=5. 2L=5 (L not integer).
# Next 2n+1 = 9 (j=2) => L_9=76. 5b-4=76 => 5b=80 => b=16. Y=F_9=34. 2L=34 => L=17. (b=16, L=17, h=15) This is a solution. Index 2n+1 = 9.
#
# Case 2: h = b + 1
# 4(b+1)^2 + b^2 = 4L^2
# 4(b^2 + 2b + 1) + b^2 = 4L^2
# 5b^2 + 8b + 4 = 4L^2
# This can be rewritten as (5b+4)^2 - 5(2L)^2 = -4.
# Let X = 5b+4, Y = 2L. Then X^2 - 5Y^2 = -4.
# X = 5b+4 = L_{2n+1}
# Y = 2L   = F_{2n+1}
# From 5b+4 = L_{2n+1}, we need L_{2n+1} === 4 (mod 5).
# L_k === 4 (mod 5) if k === 3 (mod 4).
# So 2n+1 must be of the form 4j+3.
# Smallest is 2n+1 = 3 (j=0) => L_3=4. 5b+4=4 => 5b=0 => b=0 (not positive).
# Next 2n+1 = 7 (j=1) => L_7=29. 5b+4=29 => 5b=25 => b=5. Y=F_7=13. 2L=13 (L not integer).
# Next 2n+1 = 11 (j=2) => L_{11}=199. 5b+4=199 => 5b=195 => b=39. Y=F_{11}=89. 2L=89 (L not int).
# Next 2n+1 = 15 (j=3) => L_{15}=1364. 5b+4=1364 => 5b=1360 => b=272. Y=F_{15}=610. 2L=610 => L=305. (b=272, L=305, h=273) This is a solution. Index 2n+1 = 15.
#
# The problem statement's derivation is $L = F_{6j+3}/2$.
# For j=1: $m=9$. $F_9/2 = 34/2 = 17$. This matches $L=17$ from Case 1 (index 2n+1=9).
# For j=2: $m=15$. $F_{15}/2 = 610/2 = 305$. This matches $L=305$ from Case 2 (index 2n+1=15).
# This implies that the indices $2n+1$ for valid $L$ (where $F_{2n+1}$ is even) are $9, 15, 21, ...$
# These are of the form $6j+3$.
# $F_k$ is even if $k$ is a multiple of 3. All $6j+3$ are multiples of 3. So $F_{6j+3}$ is always even.
# Thus $L = F_{6j+3}/2$ will be an integer.

# Function to compute F_n (n-th Fibonacci number)
# F_0 = 0, F_1 = 1, F_2 = 1, F_3 = 2, ...
def fibonacci(n)
  return 0 if n == 0
  return 1 if n == 1
  
  a = 0 # Represents F_{i-2}
  b = 1 # Represents F_{i-1}
  # Loop from 2 up to n
  (2..n).each do
    c = a + b # F_i = F_{i-1} + F_{i-2}
    a = b     # Update F_{i-2} to previous F_{i-1}
    b = c     # Update F_{i-1} to current F_i
  end
  b # This is F_n
end

total_L_sum = 0
num_triangles = 12 # We need the sum for the twelve smallest triangles

(1..num_triangles).each do |j|
  # Calculate the Fibonacci index m = 6j+3
  m = 6 * j + 3
  
  # Compute F_m
  f_m = fibonacci(m)
  
  # L is F_m / 2. F_m is always even since m is a multiple of 3.
  l_val = f_m / 2
  
  total_L_sum += l_val
end

puts total_L_sum
