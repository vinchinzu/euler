# Digit canceling fractions
# Problem 33
# The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct,
# is obtained by cancelling the 9s.

# We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

# There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

# If the product of these four fractions is given in its lowest common terms, find the value of the denominator.



numerator_product, denominator_product = 1, 1


(10..98).each do |n|

  (n+1..99).each do |d|

    # Non-trivial check

    next if n % 10 == 0 && d & 10 == 0

 

    # See if opposite digits in numerator and denominator match

    if (n.to_s[0] == d.to_s[1] && n.to_s[1] < d.to_s[0])

      # Second digit of numerator over first digit of denominator

      fraction_as_float = Float(n.to_s[1]) / Float(d.to_s[0])

    elsif (n.to_s[1] == d.to_s[0] && n.to_s[0] < d.to_s[1])

      # First digit of numerator over second digit of denominator

      fraction_as_float = Float(n.to_s[0]) / Float(d.to_s[1])

    else

     next

  end

 

    # Do we get the same value if we cancel out like digits?

    if (Float(n) / Float(d) == fraction_as_float)

      numerator_product *= n
      denominator_product *= d

    end
  end

end
puts denominator_product / numerator_product








# a = (10..99).to_a
# b = (10..99).

# class String
  # def to_frac
    # numerator, denominator = split('/').map(&:to_f)
    # denominator ||= 1
    # numerator/denominator
  # end
# end




	# arr = []
	# a.to_a.each do |x|
	  # b.to_a.each do |y|
		# arr << "#{x}/#{y}" if x < y 
	 # end
	# end

# create an array of "49/98" to strings... find a regex to drop the numbers and convert.. 
# arr.each{|x| x.split("/")

# def(n)
# x = n.split("/")

# x.map!{|x| x.to_f}
