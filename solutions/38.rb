# Pandigital multiples
# Problem 38
# Take the number 192 and multiply it by each of 1, 2, and 3:

# 192  1 = 192
# 192  2 = 384
# 192  3 = 576
# By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)

# The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

# What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n  1?


def pan?(n)
test = (1..9).map{|x| x.to_s}
arr = [a,b,c]
arr = arr.to_s.gsub(/[^0-9]/,'').split('').to_a.sort
arr==test

end

def pan(n) 
test = "123456789"
sorted = n.to_s.chars.to_a.sort.*''
test == sorted
end

d=[]
# (100000000..999999999).each do |x|
  # d << x if pan(x)
 # end

 test = "123456789"
 set = test.chars.to_a.permutation(9)	
 
 
 max = 0 
 (9..98765).each do |q|
 
 str=""
  n=1
 
 while str.length < 9
 str = str + (q*n).to_s
 n+=1
 end
 
 max  = str.to_i if pan(str) && str.to_i > max 
 
 end
 puts max
