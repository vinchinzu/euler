# Prime permutations
# Problem 49
# The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.

# There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

# What 12-digit number do you form by concatenating the three terms in this sequence?


require 'Prime'

set = Prime.first(1200)
set = set.delete_if{|i| i < 1000 or i >9999}

i=0
(0..500).each do |i|
try = [set[i], set[i] + 3330, set[i] +6660]

  if try.all?{|i| i.prime?}
	a = try[0].to_s.split(//).sort
	b = try[1].to_s.split(//).sort
	c = try[2].to_s.split(//).sort

	 if a==b and b==c
     puts try
	 end
  end
end

#set to print
