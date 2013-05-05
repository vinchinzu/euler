

total = 0 
(1..10000).each do |i|
 total += i if (i%3 == 0 || i%5==0) 
end

puts total


