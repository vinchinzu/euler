#write block of code to loop through and for each one. 

require 'set'
require 'benchmark'

# Function to generate the progress chart in Markdown
def generate_progress_chart(solved_problem_numbers, max_problem_overall = 300, problems_per_block = 100, cols_per_row = 10)
  chart_string = "## Problems Solved Grid\\n\\n"
  num_blocks = (max_problem_overall.to_f / problems_per_block).ceil

  (0...num_blocks).each do |block_index|
    block_start_problem = block_index * problems_per_block + 1
    current_block_conceptual_end = (block_index + 1) * problems_per_block
    block_end_problem = [current_block_conceptual_end, max_problem_overall].min

    chart_string += "### Problems #{block_start_problem} - #{block_end_problem}\\n\\n"

    placeholder_header_cells = [" "] * cols_per_row # Using single space as placeholder
    chart_string += "| " + placeholder_header_cells.join(" | ") + " |\\n"
    
    alignment_row = "| " + ([":--:"] * cols_per_row).join(" | ") + " |\\n"
    chart_string += alignment_row

    table_content_lines = []
    num_rows_in_block = ((block_end_problem - block_start_problem + 1).to_f / cols_per_row).ceil

    (0...num_rows_in_block).each do |row_idx|
      row_cells = []
      (0...cols_per_row).each do |col_idx|
        problem_num = block_start_problem + row_idx * cols_per_row + col_idx
        
        if problem_num > block_end_problem
          row_cells << " " # Empty cell for padding
        else
          if solved_problem_numbers.include?(problem_num)
            row_cells << "**#{problem_num}**"
          else
            row_cells << problem_num.to_s
          end
        end
      end
      table_content_lines << "| " + row_cells.join(" | ") + " |"
    end
    
    chart_string += table_content_lines.join("\\n") + "\\n\\n" if table_content_lines.any?
  end
  chart_string
end

path = File.expand_path(File.dirname(__FILE__))

results = []
# total_time = 0.0 # total_time is calculated later by summing results

Dir.glob("solutions/*.rb").each do |x|
  timer_start = Time.now
  require path + '/' + x 
  timer = ((Time.now - timer_start)) * 1000
  results << [File.basename(x), timer]
  # puts "Time: #{timer} ms" # Outputting summary at the end might be cleaner
end

solved_problem_filenames = results.map { |filename, _| filename }
solved_problem_numbers = Set.new
solved_problem_filenames.each do |filename|
  match = filename.match(/problem_(\\d+)\\.rb/)
  solved_problem_numbers.add(match[1].to_i) if match
end

total_solved = results.size
# average_time calculation was correct
total_time = results.map { |r| r[1] }.sum # Ensure total_time is summed up correctly
average_time = total_solved > 0 ? total_time / total_solved : 0.0


File.open("README.md", "w") do |f|
  f.puts "# My Progress on Project Euler in Ruby\\n"

  max_chart_problem = 300 
  chart_content = generate_progress_chart(solved_problem_numbers, max_chart_problem)
  f.puts chart_content

  f.puts "## Summary\\n"
  f.puts "Total solved: #{total_solved}"
  f.puts "Total time: #{'%.3f' % total_time} ms"
  f.puts "Average time: #{'%.3f' % average_time} ms\\n"

  f.puts "## Solved Problems Details\\n"
  if results.any?
    # Sort results numerically by problem number for the detailed list
    sorted_results = results.sort_by { |filename, _| 
      match = filename.match(/problem_(\\d+)\\.rb/)
      match ? match[1].to_i : Float::INFINITY # Ensures files matching pattern sort correctly
    }
    
    max_filename_length = sorted_results.map { |filename, _| filename.length }.max || 0
    max_time_length = sorted_results.map { |_, time| ('%.3f' % time).length }.max || 0
    
    sorted_results.each do |filename, time|
      # Using markdown list format
      f.puts sprintf("* %-#{max_filename_length}s - Time: %#{max_time_length}.3f ms", filename, time)
    end
  else
    f.puts "No problems solved yet.\\n"
  end
end

puts "README.md updated successfully."
puts "Total solved: #{total_solved}"
puts "Total time: #{'%.3f' % total_time} ms"
puts "Average time: #{'%.3f' % average_time} ms"     