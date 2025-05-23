# A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

# a2 + b2 = c2
# For example, 32 + 42 = 9 + 16 = 25 = 52.

# There exists exactly one Pythagorean triplet for which a + b + c = 1000.
# Find the product abc.

set = []
(1..333).each do |x|
 (2..666).each do |y|
  
  c = (x**2 + y**2)**(0.5)
  #if c.is_a? Integer
 #set << [x,y,c]
  set << [x,y,c]
#end
end
end

set.keep_if{|x| x.inject(:+)==1000}
puts set[0].inject(:*)


# set.each do |x|
 # if set[x].inject(:+) != 1000
   # set.drop(x); nil
 # end
# end


# sum = a + b + b
# product = a*b*c

# b = 31875000

# set_2 =[]

# (1..332).each do |x|
  # set_2 << b/x
# end



# a=1
# b=

# while a + b + c <= 1000
