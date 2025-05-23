
# Maximum path sum I
# Problem 18
# By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

# 3
# 7 4
# 2 4 6
# 8 5 9 3

# That is, 3 + 7 + 4 + 9 = 23.

# Find the maximum total from top to bottom of the triangle below:

# 75
# 95 64
# 17 47 82
# 18 35 87 10
# 20 04 82 47 65
# 19 01 23 75 03 34
# 88 02 77 73 07 63 67
# 99 65 04 28 06 16 70 92
# 41 41 26 56 83 40 80 70 33
# 41 48 72 33 47 32 37 16 94 29
# 53 71 44 65 25 43 91 52 97 51 14
# 70 11 33 28 77 73 17 78 39 68 17 57
# 91 71 52 38 17 14 91 43 58 50 27 29 48
# 63 66 04 68 89 53 67 30 73 16 69 87 40 31
# 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
	
# set = []
# f = File.open("18.txt", "r")
 
 
# #read lines into array
# set = []
# f.each_line do |line|
  # set << line
# end
 
 # #remove and substitute text based lines breaks "\n"
 # set.each{|x| x.gsub!(/\n/,'') }
 
 # #split all the strings and map to array of arrays 
 # a = set.map{|x| x.split(' ').to_a }
 
 # # iterate over both arrays to convert al numbers to string
# a.each do |y|
 # y.map!{|x| x.to_i}
# end
 
 
 # f=1
 # s=0
 
 # a[0][0] 
 
 # sum = 0
 # sum +=  a[0][0] 

# while (f < (a.count ) )
 
 # if a[f][s] > a[f][s+1]
 # then 
 # c = a[f][s] 
 # else 
 # c = a[f][s+1]
 # s+=1
# end

# f +=1 
# sum += c

# end

# sum








# #Retest:
# le = []

# #run until all possibilities are checked - naive form algorithm
# while ( f+s < (a.count + a[(a.count-1)].count ) )

 
 # c = a[f][s] 
 # else 
 # c = a[f][s+1]
 # s+=1
# end

# f +=1 
# sum += c
# le << sum
# end


# y=0
# (0..14).each do |x|
  # puts a[x][y]  
# end

# y +=1

# (2..14).each do |x|
  # puts a[x][y]
# end









################


data = \
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"

rows = data.split("\n")




triangle = rows.map { |x| x.split(" ") \
  .map { |x| x.to_i } }
  

base  = rows.map { |x| x.split(" ") \
  .map { |x| x.to_i } }  
  
# (triangle.length - 1).downto(0) {|x|
 # puts a[x][1]
# }

  
(triangle.length - 1).downto(0) { |a|
  0.upto(a-1) { |b|  
    triangle [a-1][b] += [triangle [a][b], triangle [a][b+1]].max
  }  
}
 
  
puts triangle [0][0]
     # Get the maximum value for adjacent cells in current row.
    # Update the cell which would be one step prior in the path
    # with the new total. For example, compare the first two 
    # elements in row 15. Add the max of 04 and 62 to the first 
    # position of row 14.This provides the max total from row 14 
    # to 15 starting at the first position. Continue to work up 
    # the triangle until the maximum total emerges at the 
    # triangle's apex.   
	
	    # puts triangle [a-1][b]
