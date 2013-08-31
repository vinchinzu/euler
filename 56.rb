# Powerful digit sum
# Problem 56
# A googol (10^100) is a massive number: one followed by one-hundred zeros; 
# 100^100 is almost unimaginably large: one followed by two-hundred zeros. 
# Despite their size, the sum of the digits in each number is only 1.

# Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?

max = 0



def digit_sum(n)
 n.to_s.split(//).to_a.map{|x| x.to_i}.reduce(:+)
end

99.downto(1) do |y|
99.downto(1) do |x|
  a = digit_sum(x**y)
   #puts a if a > 400 
  (a > max ) ? max = a : next
  puts max
end
end

puts max