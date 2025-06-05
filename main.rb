#!/usr/bin/env ruby

# Require all problem solution files from the solutions directory
Dir[File.join(__dir__, 'solutions', '*.rb')].each { |file| require file }

if __FILE__ == $0
  puts "Euler Problem Solutions:"
  puts "------------------------"

  # Problem 147: Rectangles in Cross-hatched Grids
  max_m_147 = 47
  max_n_147 = 43
  total_count_147 = problem_147(max_m_147, max_n_147)
  puts "Problem 147: Total rectangles in grids up to #{max_m_147}x#{max_n_147}: #{total_count_147}"

  # Problem 148: Exploring Pascal's Triangle
  count_148 = problem_148
  puts "Problem 148: Entries not divisible by 7 in first 10^9 rows: #{count_148}"

  # Problem 149: Maximum-sum Subsequence
  max_sum_149 = problem_149
  puts "Problem 149: Max sum in LFG table (2000x2000): #{max_sum_149}"

  # Problem 150: Sub-triangle Sums
  min_sum_150 = problem_150
  puts "Problem 150: Smallest sub-triangle sum (1000 rows): #{min_sum_150}"

  # Problem 151: A Preference for A5
  expected_value_151 = problem_151
  puts "Problem 151: Expected single sheets (A5 preference): #{'%.6f' % expected_value_151}"
  puts "------------------------"
end
