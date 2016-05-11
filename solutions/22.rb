#euler 22

 # 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

# For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, 
# is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

# What is the total of all the name scores in the file?


tex = File.readlines('names.txt', 'r');

a = tex[0] ;

b = a.split(',').to_a;
c = b.map{|x| x.gsub!('"','')};


def name_score(name)
  name.enum_for(:each_byte) \
    .map { |c| c - 64 } \
    .inject { |agg, n| agg + n }
end

def n_s(name)
  sum = 0
   name.each_byte { |c|
     sum += c - 64
  }
  sum
end


arr = c.each_with_index.map { |x,i| name_score(x)*(i+1) }.inject(:+);


puts arr


# For alt
# ans = c.map{|x| n_s(x)}.inject(:+)


# n_s(c[1])
# name_score(c[1])

# name = c[1]
 # sum = 0
   # name.each_byte { |c|
     # sum += c - 64
	 # puts c
	 # puts sum
  # }
  # sum