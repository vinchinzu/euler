limit = 4000000

#while n<limit

def fib(n)
   a = 0
   b = 1

  n.times do|i|
    a, b = b, a+b
  end

  return a
end

puts fib(100)

