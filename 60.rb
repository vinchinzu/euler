# Prime pair sets
# Problem 60
# The primes 3, 7, 109, and 673, are quite remarkable. 
# By taking any two primes and concatenating them in any order the result
 # will always be prime. For example, taking 7 and 109, both 7109 and 1097 are prime. 
 # The sum of these four primes, 792, represents the lowest sum for a set of four primes
 # with this property.

# Find the lowest sum for a set of five primes for which any two primes concatenate 
# to produce another prime.




require 'Prime'

set = Prime.first(120)

test = [3,7,109,673]

base = []
set.each do |a|
	set.each do |b|
	 
	 if (a+b).prime?
	  base << a + b
	 end
	end
end
base


base.each do |a| 
  base.each do |b|
    x = (a.to_s + b.to_s).to_i
	if x.prime?
	 puts x
	end
	end
end


test = [3,7,109,673]
test << 43
test

cheq_set(test)

def check_array_primes(arr)
  arry{is.prime
  arr.all?{|x| x.isprime?}
end

def cheq_set(arr)
 c_arr = []
 arr.each do |a|
   arr.each do |b|
     x = (a.to_s + b.to_s).to_i
	if x.prime?
	 c_arr << x
    end
   end
 end
end

