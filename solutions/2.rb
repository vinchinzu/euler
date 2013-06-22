def fib 
  a, b,sum = 1,2,0
 while b < 4000000
 sum += b if b.even? 
 a, b = b, a+b
end
puts sum
end

puts fib


