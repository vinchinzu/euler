f = File.open("cipher1.txt", "r")

f.each+line do |line|
  text += line
 end
 
 text.gsub!(/\n/,'') 
 
 a = text.split(/,/).to_a
 
 a.map!{|x| x.to_i}
 
 res=Hash[a.group_by {|x| x}.map {|k,v| [k,v.count]}]
 
 
 

d = a.collect! { |element|
  (element == 79) ? "the" : element
}
