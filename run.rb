#write block of code to loop through and for each one. 

require 'benchmark'

path = File.expand_path(File.dirname(__FILE__))

results = []
total_time = 0.0

Dir.glob("solutions/*.rb").each do |x|
  puts x
  timer_start = Time.now
  load File.join(path, x)
  timer = ((Time.now - timer_start)) * 1000
  results << [File.basename(x), timer]
  puts "Time: #{timer} ms"

  # Append to results.txt immediately
  results_txt_path = File.join(path, "results.txt")
  begin
    File.open(results_txt_path, "a") do |f|
      f.puts "#{File.basename(x)}\t %10.3f ms" % timer
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
    if results.any?
      # Ensure the new block of results starts on a new line.
      # This is particularly for the case where README.md might not end with a newline.
      if f.pos > 0 # Check if the file is not empty (f.pos is current position, which is EOF for 'a' mode)
        f.seek(-1, IO::SEEK_END) # Go to the last character
        last_char = f.getc
        f.seek(0, IO::SEEK_END) # Return to the end of the file for appending
        f.puts if last_char != "\n" # Add a newline if the last character wasn't one
      end

      results.each do |filename, time|
        # This f.puts ensures that each specific result line ends with a newline.
        f.puts "#{filename}\t %10.3f ms" % time
      end
    end
  end
rescue => e
  puts "Failed to write README.md: #{e.message}"
end     