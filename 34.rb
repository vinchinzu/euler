# Digit factorials
# Problem 34
# 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

# Find the sum of all numbers which are equal to the sum of the factorial of their digits.

# Note: as 1! = 1 and 2! = 2 are not sums they are not included.

#factorial function, might be faster ways
 def fact(n)
  (1..n).inject(:*) || 1
  end


def curious(n)
  arr = n.to_s.split(//)
  sum = 0
  arr.each {|x| sum += fact(x.to_i)}
  if sum == n 
   true
  else
  false
 end
 end
 
 #stole this piece/ slows down to 29.749 seconds.. 
 def uplim
    ndig = 1
    while (10**(ndig-1)) <= (ndig*fact(9)) do
       result = ndig * fact(9)
       ndig += 1
    end 
    result   
end
 
 
 d = []
 (3..41000).each{|x| d << x if curious(x)}.inject(:+)
   puts d.inject(:+)
 
 
 #439 ms 6/21/2013
 # trial two with upper limite down to 30s
 
  