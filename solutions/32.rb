# Pandigital products
# Problem 32
# We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

# The product 7254 is unusual, as the identity, 39  186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

# Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

# HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.


#Test to see if inputs are pandigital
def pan?(a,b,c)
test = (1..9).map{|x| x.to_s}
arr = [a,b,c]
arr = arr.to_s.gsub(/[^0-9]/,'').split('').to_a.sort
arr==test

end

#loop thorugh everything.. find set of a, b, c
#what are the limits? 

#brute force is a little slow
a = (3..485).to_a
b = (10..1970).to_a
d= []
a.each do |x|
 b.each do |y|
 d << [x,y,x*y] if pan?(x,y,x*y)
 end
end
 
third = d.map{|x| x[2]}.uniq.inject(:+)
puts third


#17 s; first run


