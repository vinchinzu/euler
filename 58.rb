require 'prime'

def spiral(n)

i = 1
arr = []
j = 2
 
while i < n**2
  1.upto(4) do |x|
   i += j
   arr << i
  end
 j +=2
end
arr
end

a = spiral(30000)

j=27500

def prop(j,a)
i = j*2+1	
a[0..i].count{|x| x.prime?}*1000/a[0..i].count
end

#After trial and error estimated it down to last .. way too slow
j = 26201
until prop(j,a) < 100
 j += 2
 puts j
end
# def ratio(i)
  # z = spiral(i).count{|x| x.prime?}*100/spiral(i).count
# end



# $primeCount = 0.0
# $diagCount = 1.0

# ratio = 1.0

# def checkPrimes(arr)
  # arr.each do |elem|
    # if elem.prime?
      # $primeCount += 1
    # end
  # end
# end

# @n = 3

# while ratio >= 0.1
  # ratio = $primeCount/$diagCount
  # $diagCount = (2*@n) - 1
  # br = @n ** 2
  # bl = br - (@n - 1)
  # tl = br - (2*@n - 2)
  # tr = br - (3*@n - 3)
  # checkPrimes([br, bl, tl, tr])
  # @n += 2
  # ratio = $primeCount/$diagCount
# end

# puts(sprintf("The length of the side when the ratio of primes to diagonal elements is < 0.10 : %d \n", @n - 2))



# require "mathn"

# prime_count = 8
# total_count = 13
# i = 7

# while true
   # if prime_count/total_count < 0.1
      # break
   # end
   # i += 2
   # total_count += 4
   # if (i**2-(i-1)).prime?
      # prime_count += 1
   # end
   # if (i**2-2*(i-1)).prime?
      # prime_count += 1
   # end
   # if (i**2-3*(i-1)).prime?
      # prime_count += 1
   # end
# end

# puts i