#!/usr/bin/env ruby
# p105.rb – Project Euler 105
# Reads “sets.txt” (must be in the same directory) and prints the answer.
require "set"

FILE_NAME = "sets.txt"

def prefix_rule_ok?(a)
  (2..a.size).each { |k| return false if a.first(k).sum <= a.last(k - 1).sum }
  true
end

def special_sum_set?(a)
  return false unless prefix_rule_ok?(a)

  sums = {}
  (1...(1 << a.size)).each do |mask|
    s = 0
    bits = mask
    i = 0
    while bits > 0
      if bits & 1 == 1
        s += a[i]
      end
      i += 1
      bits >>= 1
    end
    return false if sums.key?(s)
    sums[s] = true
  end
  true
end

total = 0
File.readlines(FILE_NAME, chomp: true).each do |line|
  set = line.split(",").map(&:to_i).sort
  total += set.sum if special_sum_set?(set)
end

puts total

