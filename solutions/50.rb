# The prime 41, can be written as the sum of six consecutive primes:

# 41 = 2 + 3 + 5 + 7 + 11 + 13
# This is the longest sum of consecutive primes that adds to a prime below one-hundred.

# The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

# Which prime, below one-million, can be written as the sum of the most consecutive primes?


require 'prime'

first = Prime.first(5000)


max = [0,0,0]

#number of consecutive j ; assumed less than 10
(0..10).each do |j|
# sum of consecutive and check to see is less than 1million and is prime
(j+1..first.count).each do |x|
   check = first[j..x].inject(:+)
   if  check < 1000000 && check.prime? 
      max = [j,x, check] if x > max[1] 
	 end
  end
end

puts max

# 24133 is prime... 2...541 total of 
# 958577 i sprime  2..545th 3863




# x = first.inject(:+)
# x.prime?

# puts x

# yourfile = "list.csv"
# File.open(yourfile, 'w') { |file| file.write(y) }




# require 'csv'

# CSV.open("list.csv", "w") do |csv|
   # y.each { |i| csv << i }
  # end