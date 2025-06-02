# Integer right triangles
# Problem 39
# If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

# {20,48,52}, {24,45,51}, {30,40,50}

# For which value of p  1000, is the number of solutions maximised?

p = 1000
counts = {}

counts.default = 0 
(1..p/2).each do |x|
  (x..p/2).each do |y|
    break if x + y > (p/2)
	c = Math.sqrt(x**2 + y**2)
	next unless c % 1 == 0
    current_perimeter = x + y + c.to_i # c is already an integer if c % 1 == 0
    if current_perimeter <= p # Ensure perimeter is within the specified limit (1000)
      counts[current_perimeter] += 1
    end
   end
end
   
# Filter out any perimeters that might have been inadvertently processed if logic allowed > p
# (though the above check should handle it)
valid_counts = counts.select { |perimeter, _| perimeter <= p }
sort = valid_counts.sort {|a,b| a[1] <=> b[1]}

puts sort.last[0]
  
  
  
  
  
 # def right?(a,b,c)
 # a**2 + b**2 == c**2 ? true : false
# end

 # def test p 
  # d = []
  # (1..p/2).each do |x|
    # (x..p/2).each do |y|
	 # (y..p).each do |z|
		  # if x + y + z == p
		    # d << [x,y,z] if right?(x,y,z) 
		   # end
		  # end
	   # end
	# end
	# return d.count
# end

# elements = {}
# r= (1..1000).each {|i| elements[i] = test(i)}
# first test of brute force seems to be slow...

# max = 0
# set = []
# elements.each do |x| 
# if x.count > max

 # max = x.count
 # set << x
 # else 
 # next
 
 # end
 # end
 
# puts set.last[0].inject(:+)
 

 
  
  

