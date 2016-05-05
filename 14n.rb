#14 memoized

require 'memoist'


def col(test)
  if test%2==0 
  out = test/2
  else 
 out = test*3 + 1
 end
out
end


#function to return amount number of factors
def col2(num)
 set = []
 set << num
 nex = col (num)
  while (nex != 1) 
   nex = col(nex)
   set << nex
  end

  count = set.count
  return count
end

memoize :col2


set = []
(1..100_000_000).each do |x|
set << [x, col2(x)]
end



max = 1
top = 1
start  = 750000

while (start < 1_000_000)
 set = []
 set << start
 nex = col (start)
  while (nex != 1) 
   nex = col(nex)
   set << nex
  end
 
if set.count > max then 
  max = set.count
  top = start
end

start +=1


#puts start,max, set.count
end

puts top
