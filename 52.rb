# Permuted multiples
# Problem 52
# It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.

# Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.'


# x = 125874


def m(x)
  m = x.to_s.split(//).sort
 end

def check(x)

  
  a = (m(x) == m(x*2) and m(x) == m(x*3) and m(x) == m(x*4) and m(x) == m(x*5) and m(x) == m(x*6) ) ? true : false
  a
end
	  

def check2(x)
  (2..6).each do |i|
    m(x) == m(x*i) ? true : false
  end
end


(1..200000).each do |x|
   if check2(x)
    puts x
   end
 end
 
 
i=1 
until check2(i)
 i += 1
end

puts i