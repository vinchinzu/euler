#write block of code to loop through and for each one. 

# require 'benchmark'



path = File.expand_path(File.dirname(__FILE__))

output = File.open( "results.txt", "w" )


Dir.glob("solutions/*.rb").each do |x|

timer_start = Time.now
require  path + '/' + x 

 timer = "Time: #{(round(Time.now - timer_start))*1000} ms"
 puts timer
 
 output << "#{File.basename(x)} \t #{timer}  \n"

end


output.close     