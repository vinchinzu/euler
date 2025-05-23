#write block of code to loop through and for each one. 

require 'benchmark'

path = File.expand_path(File.dirname(__FILE__))

results = []
total_time = 0.0

Dir.glob("solutions/*.rb").each do |x|
  timer_start = Time.now
  load File.join(path, x)
  timer = ((Time.now - timer_start)) * 1000
  results << [File.basename(x), timer]
  puts "Time: #{timer} ms"

  # Append to results.txt immediately
  results_txt_path = File.join(path, "results.txt")
  begin
    File.open(results_txt_path, "a") do |f|
      f.puts "#{File.basename(x)}\tTime: %10.3f ms" % timer
    end
  rescue => e
    puts "Failed to write results.txt: #{e.message}"
  end
end

puts "Results: #{results.inspect}"  # Debug print

total_solved = results.size
average_time = total_solved > 0 ? results.map { |r| r[1] }.sum / total_solved : 0.0
total_time = results.map { |r| r[1] }.sum

readme_path = File.join(path, "README.md")
puts "Appending to README.md at #{readme_path}"
begin
  File.open(readme_path, "a") do |f|
    results.each do |filename, time|
      f.puts "#{filename}\tTime: %10.3f ms" % time
    end
  end
rescue => e
  puts "Failed to write README.md: #{e.message}"
end     