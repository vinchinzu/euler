# Digit canceling fractions
# Problem 33
# The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct,
# is obtained by cancelling the 9s.

# We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

# There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

# If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

a = (10..99).to_a
b = (10..99).to_a

	arr = []
	a.to_a.each do |x|
	  b.to_a.each do |y|
		arr << "#{x}/#{y}" if x < y 
	 end
	end

#create an array of "49/98" to strings... find a regex to drop the numbers and convert.. 
arr.each{|x| x.split("/")

def(n)
x = n.split("/")

x.map!{|x| x.to_f}
