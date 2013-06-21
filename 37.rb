# Truncatable primes
# Problem 37
# The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

# Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

# NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

require 'Prime'


def test(p)
 arr = p.to_s.split(//)
 ntimes= arr.count
 
 d = []
 
 ntimes.times do |x|
 
 truncated = arr.drop(x)
  digit = truncated.*''
  d << digit
end

(ntimes).times do |x|
 
 truncated = arr.last(x)
  digit = truncated.*''
  d << digit
end

d.delete_if{|x| x == ""}
d.all?{|p| p.to_i.prime?}

end

arr = Prime.take_while{|x| x < 1_000_000}

list = []
arr.each{|p| list << p if test(p)}


puts list.count
puts list.inject(:+)
 