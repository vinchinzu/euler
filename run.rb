#write block of code to loop through and for each one. 

# require 'benchmark'



path = File.expand_path(File.dirname(__FILE__))

Dir.glob("solutions/*.rb").each do |x|

timer_start = Time.now
require  path + '/' + x 
puts "Time: #{(Time.now - timer_start)*1000} ms"

end
