

total = 0 
(1..999).each do |i|
 total += i if (i%  3 == 0 || i % 5 == 0) 
end

puts total




n = n.to_i
s = s.to_i


n=10
s=100

def check_sum(n)   
  total = 0 
  (1..(n-1)).each do |i|
   total += i if (i%  3 == 0 || i % 5 == 0) 
  end
     total
end

    puts check_sum(n)
    puts check_sum(s)