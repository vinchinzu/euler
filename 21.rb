# # Let d(n) be defined as the sum of proper divisors of n 
# (numbers less than n which divide evenly into n).
# # If d(a) = b and d(b) = a, where a ≠ b, then a and b are 
# an amicable pair and each of a and b are called amicable numbers.

# # For example, the proper divisors of 220 are
 # 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
 # The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

# # Evaluate the sum of all the amicable numbers under 10000.

#find all divisiors

def div(n)
  set = []
  (1..n).each do |x|
   n%x == 0 ? set << n/x : next
   end 
   set
end 

def d(n)
  t = div(n)
  t.shift
  t.inject(:+)
end

amic = []
(2..10000).each do |n|
  a = d(n)
  b = d(a)
 if n == b && n != a then 
   amic << [n, a]
  else 
 end
end
 
 puts amic.flatten.uniq.inject(:+)
 
 
  


# class Integer
  # def divisor_count
    # sum = 1
    # # prime_division return two dimensional array.
    # # for 48, [3,1], [2,4] is the result
    # self.prime_division.each do |x| 
      # sum *= (x[1] + 1)  
    # end
    # sum
  # end
# end
