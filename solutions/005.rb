# Smallest multiple
# Problem 5
# What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

def lcm(a, b)
  a.lcm(b)
end

result = (1..20).reduce { |acc, n| lcm(acc, n) }
puts result