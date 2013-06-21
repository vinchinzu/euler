#project euler 26
require 'mathn'
require 'prime'

def can(n)
   arr = (1..n).map do |x|
   if x.prime? 
    x unless x%2 ==0 || x%5==0
	end
	end
	
	arr = arr.compact.sort.reverse
end

def cycle(n)
  arr = (1..n).map do |x|
   if x.prime? 
    1/x.to_f unless x%2 ==0 || x%5==0
   end
  end
  
  arr.map!{|x| x.to_s}
  arr.keep_if{|x| x.length>4}
end

def recur(s)
   arr = s.split(//)
   arr = arr.drop(2)
   
   first = if arr[0]==0 then
   
  
   
   arr
   
   
end
   

   