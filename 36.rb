# Double-base palindromes
# Problem 36
# The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

# Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

# (Please note that the palindromic number, in either base, may not include leading zeros.)






def palindrome?(str)
  str == str.reverse
end

list = Array.new

1.upto(1_000_000).each do |x|
s= x.to_s(2)
if palindrome?(s) && palindrome?(x.to_s) 
 list << x
end
end

puts list.inject(:+)

#467 ms