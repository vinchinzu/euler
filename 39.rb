# Integer right triangles
# Problem 39
# If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

# {20,48,52}, {24,45,51}, {30,40,50}

# For which value of p  1000, is the number of solutions maximised?
  
  
 def right?(a,b,c)
 a**2 + b**2 == c**2 ? true : false
end

 def test p 
  d = []
  (1..p/2).each do |x|
    (x..p/2).each do |y|
	 (y..p).each do |z|
		  if x + y + z == p
		    d << [x,y,z] if right?(x,y,z) 
		   end
		  end
	   end
	end
	return d.count
end

elements = {}
r= (1..1000).each {|i| elements[i] = test(i)}
#first test of brute force seems to be slow...

max = 0
set = []
elements.each do |x| 
if x.count > max

 max = x.count
 set << x
 else 
 next
 
 end
 end
 
puts set.last[0].inject(:+)
 

 
  
  

