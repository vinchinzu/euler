# Largest palindrome product
# Problem 4
# A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91  99.

# Find the largest palindrome made from the product of two 3-digit numbers.


max = 0 
(1..999).each do |x|
  (1..999).each do |y|
    if (x*y).to_s == (x*y).to_s.reverse
	  max = x*y if x*y > max
	  end
  end
end

puts max
