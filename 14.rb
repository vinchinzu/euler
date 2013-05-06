#longest collatz

# n = 3n+1 if n odd?
# n = 2n if n even? 

def calc_seq(n,s)
  if n==1
  return s
elsif n & 1 ==0
return calc_seq(n/2,s+1)
else
return calc_seq(3*n+1, s+1)
end
end

seq = (1..1000000).map do |i|
[i, calc_seq(i)]
end
puts seq.max by {|i| i[1]}


