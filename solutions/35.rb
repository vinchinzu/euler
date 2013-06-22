# Circular primes
# Problem 35
# The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

# There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

# How many circular primes are there below one million?


require 'Prime'

arr = Prime.take_while{|x| x < 1_000_000}

def rotate1(n, veces)
 rotated = n.to_s.split(//).rotate(veces).*''
 rotated
end 


def test(n)
 len = n.to_s.length
 arrl = (1..len).to_a
 rotate_array= []
  arrl.each do |x|
   rotate_array << rotate1(n,x)
  end
  rotate_array.all?{|i| i.to_i.prime?}  
end

set = []
arr.each{|x| set << x if test(x)}

puts set.count

#10.2 seconds
