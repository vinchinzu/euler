answer=0
for i in 1..999999
 if i.to_s == i.to_s.reverse
  x = i.to_s(2)
  if x == x.reverse
	 answer = answer + i.to_i
  end 
 end
end
puts answer

#467 ms