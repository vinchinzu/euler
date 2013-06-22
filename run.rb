#write block of code to loop through and for each one. 

# require 'benchmark'


timer_start = Time.now

path = File.expand_path(File.dirname(__FILE__))
require  path + '/5.rb'

puts "Time: #{(Time.now - timer_start)*1000} ms"



# Benchmark.bm do |x|
 # x.report("Solution: ") {
   # require  path + '/4.rb'
 # }
 # end
