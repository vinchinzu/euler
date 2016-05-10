# Coin sums
# Problem 31
# In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:

# 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
# It is possible to make £2 in the following way:

# 1£1 + 1 50p + 2 20p + 1 5p + 1 2p + 3 1p
# How many different ways can £2 be made using any number of coins?

set = [1,2,5,10,20,50,100,200]


a=200
b=0


set = []
(0..200).each do |a|
 (0..100).each do |b|
   (0..40).each do |c|
    (0..20).each do |d|
     (0..10).each do |e|
	  (0..4).each do |f|
	   (0..2).each do |g|
	    (0..1).each do |h|
       if (a*1 + b * 2 + c * 5 + d * 10 + (e*20) + (f*50) + (g*100) + (h*200) ) == 200 then
        set << [a,b,c,d,e,f,g,h]
       end
      end
    end
   end
  end
 end
end
end
end



set
set.count

set = []
 	   (0..2).each do |a|
	    (0..4).each do |b|
		 (0..10).each do |c|	
          (0..20).each do |d|	
           (0..40).each do |e|
		    (0..100).each do |f|
		     tot = a*100 + b * 50 + c * 20 + d * 10 + e* 5 + f*2
            if  tot < 200 then
			  p = 200 -  tot
              set << [a,b,c,d,e,f, p]
end 
end
end
end
end
end
end




#possible to remove teh penny loop and just subtract the difference


		
		


		+ b * 2 + c * 5 + d * 10 + (e*20) + (f*50) + 






# while a != 0
  # a -= 1
  # b += 1
  # puts a, b, combos
  # combos +=1
# end



# max = 200
 
# sum = 0
# combos = 0


# max = set[0] * 

# while sum < max
  # coins = []
  # coins << set[0]
  # puts coins
  
  # sum = coins.inject(:+)
# end

# coins

#maybe a hash map/dictionary
#how to map a-z to 1-26



