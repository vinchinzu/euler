# Coin sums
# Problem 31
# In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:

# 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
# It is possible to make £2 in the following way:

# 1£1 + 1 50p + 2 20p + 1 5p + 1 2p + 3 1p
# How many different ways can £2 be made using any number of coins?

set = [1,2,5,10,20,50,100,200]


a=200
b=0

arr = [a*1,b*2] 

while a != 0
  a -= 1
  b += 1
  puts a, b, combos
  combos +=1
end



max = 200
 
sum = 0
combos = 0


max = set[0] * 

while sum < max
  coins = []
  coins << set[0]
  puts coins
  
  sum = coins.inject(:+)
end

coins

#maybe a hash map/dictionary
#how to map a-z to 1-26



