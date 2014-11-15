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

set = Prime.first(10000)

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


def mk_per(arr)
 c_arr = []
 per = arr.permutation(2).to_a
 per.each do |a,b|
   x = (a.to_s + b.to_s).to_i
	 c_arr << x
   end
  c_arr
 end
 


def m(arr)
arr.all? {|x| x.prime?}
end

base.each do |x|
test = [3,7,109,673]
test << x

if m(mk_per(test))
puts test
puts m(mk_per(test))
end
end






# def mk_ar(arr)
 # c_arr = []
 # arr.each do |a|
   # arr.each do |b|
     # x = (a.to_s + b.to_s).to_i
	 # c_arr << x
   # end
 # end
 # c_arr
# end


#base.each do |a| 
 # base.each do |b|
  #  x = (a.to_s + b.to_s).to_i
	#if x.prime?
	# puts x
	#end
	#end
#end


#cheq_set(test)
