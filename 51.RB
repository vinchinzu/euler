# Prime digit replacements
# Problem 51
# By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

# By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

# Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

require 'Prime'

a = (0..9).to_a

primes = Prime.first(100)

primes.each do |b|

#i= b.to_s.length - 1
i = 0 
c = b.to_s.split(//).to_a
d = a.map{|x| (c[i].to_s + x.to_s + x.to_s ).to_i}
d.delete_if{|x| !x.prime?}
puts d.count if d.count > 5

end

def family_size(num)
 a = (0..9).to_a
 i= num.to_s.length - 1
 to_test = num.to_s.split(//).to_a
 d = a.map{|x| (x.to_s + x.to_s + to_test[i].to_s ).to_i}
 d.delete_if{|x| !x.prime?}
 d.count
end


	num = 10001


	test = (0..4).to_a.combination(2).to_a
	to_test = num.to_s.split(//).to_a
	i = "4"

	count=1 
	trial = to_test 
	   test.each do |x,y| 
		  trial = to_test 
		  trial[x] = i 
		  trial[y] = i
		  
		 d = trial.join.to_i
		 count += 1 if d.prime?
		end  



# replace two numbers and return prime family: 






b=541
i = 0
c = b.to_s.split(//).to_a
d = a.map{|x| (c[i].to_s + x.to_s + x.to_s).to_i}
d.delete_if{|x| !x.prime?}c
 puts d.count
 
 
 
 
 
 
 b = a.map{|x| (x.to_s + "3").to_i}


sum = 0

b.each do |x|
  sum += 1 if x.prime?
end

puts sum