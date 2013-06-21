# Integer right triangles
# Problem 39
# If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

# {20,48,52}, {24,45,51}, {30,40,50}

# For which value of p  1000, is the number of solutions maximised?


def right?(a,b,c)
 a**2 + b**2 == c**2 ? true : false
end

def get_set(p)
  a= 1
  b= 2
  c= 3
  
  (1..(p/2)-1).times do |x|
    (x+1..p/2).times do |y|
	  if


