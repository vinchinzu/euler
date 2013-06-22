# Truncatable primes
# Problem 37
# The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

# Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

# NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
require 'Prime'


def test2(p)
  arr = p.to_s.chars.to_a
  
  arr.length.times do 
    return false if (!arr.join().to_i.prime?)
	 arr.shift
	end
	
   nums = p.to_s
   while nums.length > 0
     return false if (!nums.to_i.prime?)
	 nums.chop!()
	 end
	
	return true
end

count = 0 
answer = 0

Prime.each do |x|
 next if x < 10
 break if count == 11
 if test2(x)
   count += 1 
   answer += x
 end 
end

puts answer




# require 'Prime'

# test doesn't count 0's

# def test(p)
 # arr = p.to_s.split(//)
 # ntimes= arr.count
 
 # d = []
 
 # ntimes.times do |x|
 
 # truncated = arr.drop(x)
  # digit = truncated.*''
  # d << digit
# end

# (ntimes).times do |x|
 
 # truncated = arr.last(x)
  # digit = truncated.*''
  # d << digit
# end

# d.delete_if{|x| x == ""}
# result = d.all?{|p| p.to_i.prime?}
# result
# end

# arr = Prime.take_while{|x| x < 1_000_000}

# list = []
# arr.each{|p| list << p if test(p)}


# puts list.count
# puts list.inject(:+)




  
 