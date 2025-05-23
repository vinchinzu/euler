#write block of code to loop through and for each one. 

require 'benchmark'

path = File.expand_path(File.dirname(__FILE__))

results = []
total_time = 0.0

Dir.glob("solutions/*.rb").each do |x|
  timer_start = Time.now
  require  path + '/' + x 
  timer = ((Time.now - timer_start))*1000
  results << [File.basename(x), timer]
  puts "Time: #{timer} ms"
end

total_solved = results.size
average_time = total_solved > 0 ? results.map { |r| r[1] }.sum / total_solved : 0.0
total_time = results.map { |r| r[1] }.sum

File.open("README", "w") do |f|
  f.puts "\nMy Progress on Project Euler in Ruby\n"
  max_filename_length = results.map { |filename, _| filename.length }.max || 0
  max_time_length = results.map { |_, time| ('%.3f' % time).length }.max || 0
  results.each do |filename, time|
    f.puts sprintf("%-#{max_filename_length}s \t Time: %#{max_time_length}.3f ms  ", filename, time)
  end
  f.puts "\nTotal solved: #{total_solved}"
  f.puts "Total time: #{'%.3f' % total_time} ms"
  f.puts "Average time: #{'%.3f' % average_time} ms"
end     